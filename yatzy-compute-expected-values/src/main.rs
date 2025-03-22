use std::{
    collections::{HashMap, HashSet},
    fs::OpenOptions,
    io::Write as _,
};

use chrono::Utc;
use num_bigint::BigUint;
use num_rational::Ratio;
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};
use yatzy::Dice;

use yatzy_compute_expected_values::{
    game_from_state, game_states_by_empty_field_count,
    rational::{clear_cache, expected_value_2_rerolls, prob},
    FieldState, GameState,
};

fn main() {
    let states = game_states_by_empty_field_count();
    let mut expected_values = match std::fs::read("checkpoint") {
        Ok(bytes) => postcard::from_bytes(&bytes).unwrap(),
        Err(error) => {
            eprintln!("could not open `checkpoint`: {error}");
            HashMap::with_capacity(958_974)
        }
    };

    if !expected_values.is_empty() {
        eprintln!("loaded checkpoint with {} states", expected_values.len());

        let state = GameState {
            numbers_total: 60,
            ones: FieldState::Filled,
            twos: FieldState::Empty,
            threes: FieldState::Filled,
            fours: FieldState::Filled,
            fives: FieldState::Filled,
            sixes: FieldState::Filled,
            one_pair: FieldState::Filled,
            two_pairs: FieldState::Filled,
            three_of_a_kind: FieldState::Filled,
            four_of_a_kind: FieldState::Filled,
            small_straight: FieldState::Filled,
            large_straight: FieldState::Filled,
            full_house: FieldState::Filled,
            chance: FieldState::Filled,
            yatzy: FieldState::Empty,
        };
        eprintln!("{:#?}: {}", state, expected_values.get(&state).unwrap());
        std::process::exit(0);
    }

    let mut total_states = 0;
    for n in 1..=15 {
        let state_count = states.get(&n).unwrap().len();
        total_states += state_count;
        eprintln!(
            "calculating expected values for game states with {} empty field(s) ({} states)",
            n, state_count,
        );
        loop {
            let states = states.get(&n).unwrap();
            let new_values = compute_expected_values(states, &expected_values, 10_000);
            if new_values.is_empty() {
                break;
            } else {
                eprint!("\n");
            }
            expected_values.extend(new_values);

            let bytes = postcard::to_allocvec(&expected_values).unwrap();
            let filename = format!("checkpoint-{}", Utc::now().format("%Y%m%dT%H%M%SZ"));
            match OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&filename)
            {
                Ok(mut file) => match file.write_all(&bytes) {
                    Ok(()) => {
                        eprintln!("checkpoint written to {filename}");
                    }
                    Err(error) => {
                        eprintln!("failed to write checkpoint: {error}");
                    }
                },
                Err(error) => {
                    eprintln!("failed to write checkpoint: {error}");
                }
            }
        }
        clear_cache();
    }
    eprintln!("{total_states} total states");
}

pub fn compute_expected_values(
    states: &HashSet<GameState>,
    expected_values: &HashMap<GameState, Ratio<BigUint>>,
    count: usize,
) -> HashMap<GameState, Ratio<BigUint>> {
    let mut i = 0;
    states
        .into_iter()
        .filter(|state| !expected_values.contains_key(state))
        .take(count)
        .map(|&state| {
            let value = prob::ROLL_5_PROB
                .into_par_iter()
                .map(|(dice_array, prob)| {
                    let dice = Dice::new_raw(dice_array);
                    let game = game_from_state(state, dice);
                    let (numer, denom) = prob.into_raw();
                    Ratio::new(numer.into(), denom.into())
                        * expected_value_2_rerolls(game, expected_values)
                })
                .sum();
            if i % 100 == 0 {
                eprint!(".");
            }
            i += 1;
            (state, value)
        })
        .collect()
}
