use std::{
    collections::{HashMap, HashSet},
    fs::OpenOptions,
    hash::BuildHasher,
    io::Write as _,
};

use chrono::Utc;
use num_bigint::BigUint;
use num_rational::Ratio;
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};
use rustc_hash::FxBuildHasher;
use yatzy::Dice;

use yatzy_compute_expected_values::{
    FieldState, GameState, game_from_state, game_states_by_empty_field_count,
    rational::{expected_value_2_rerolls, prob},
};

fn main() {
    let states = game_states_by_empty_field_count();
    let mut expected_values = match std::fs::read("checkpoint") {
        Ok(bytes) => postcard::from_bytes(&bytes).unwrap(),
        Err(error) => {
            eprintln!("could not open `checkpoint`: {error}");
            HashMap::with_capacity_and_hasher(958_974, FxBuildHasher)
        }
    };

    if !expected_values.is_empty() {
        eprintln!("loaded checkpoint with {} states", expected_values.len());

        let initial_state = GameState {
            numbers_total: 0,
            ones: FieldState::Empty,
            twos: FieldState::Empty,
            threes: FieldState::Empty,
            fours: FieldState::Empty,
            fives: FieldState::Empty,
            sixes: FieldState::Empty,
            one_pair: FieldState::Empty,
            two_pairs: FieldState::Empty,
            three_of_a_kind: FieldState::Empty,
            four_of_a_kind: FieldState::Empty,
            small_straight: FieldState::Empty,
            large_straight: FieldState::Empty,
            full_house: FieldState::Empty,
            chance: FieldState::Empty,
            yatzy: FieldState::Empty,
        };
        if let Some(value) = expected_values.get(&initial_state) {
            eprintln!("expected value for the entire game: {value}");
        }
    }

    let mut total_states = 0;
    for n in 1..=15 {
        if n > 2 { break; }
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
            continue;
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
    }
    eprintln!("{total_states} total states");
}

pub fn compute_expected_values<S1: BuildHasher, S2: BuildHasher + Sync>(
    states: &HashSet<GameState, S1>,
    expected_values: &HashMap<GameState, Ratio<BigUint>, S2>,
    count: usize,
) -> HashMap<GameState, Ratio<BigUint>, FxBuildHasher> {
    let mut i = 0;
    states
        .into_iter()
        .filter(|state| !expected_values.contains_key(state))
        .take(count)
        .map(|&state| {
            let cache = papaya::HashMap::with_hasher(FxBuildHasher);
            let value = prob::ROLL_5_PROB
                .into_par_iter()
                .map(|(dice_array, prob)| {
                    let dice = Dice::new_raw(dice_array);
                    let game = game_from_state(state, dice);
                    let (numer, denom) = prob.into_raw();
                    Ratio::new(numer.into(), denom.into())
                        * expected_value_2_rerolls(game, expected_values, &cache)
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
