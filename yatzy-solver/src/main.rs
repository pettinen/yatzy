use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use yatzy::{print_game, Game};
use yatzy_compute_expected_values::{Choice, GameState};

use yatzy_solver::{best_choice_0_rerolls, best_choice_1_reroll, best_choice_2_rerolls};

lazy_static! {
    static ref EXPECTED_VALUES: Mutex<HashMap<GameState, f64>> = Mutex::new(HashMap::with_capacity(958_974));
}

fn main() {
    let map: HashMap<GameState, f64> = match std::fs::read("expected-values-random") {
        Ok(bytes) => {
            match postcard::from_bytes(&bytes) {
                Ok(map) => map,
                Err(error) => {
                    eprintln!("failed to read `expected-values`: {error}");
                    std::process::exit(1);
                }
            }
        }
        Err(error) => {
            eprintln!("failed to open `expected-values`: {error}");
            std::process::exit(1);
        }
    };

    let mut expected_values = EXPECTED_VALUES.lock().unwrap();
    expected_values.extend(map);

    let mut rng = rand::rng();
    let mut game = Game::new_random(&mut rng);

    print_game(game);

    while !game.ended() {
        let (choice, _) = match game.rerolls_left() {
            0 => best_choice_0_rerolls(game, &expected_values),
            1 => best_choice_1_reroll(game, &expected_values),
            2 => best_choice_2_rerolls(game, &expected_values),
            _ => unreachable!(),
        };
        match choice {
            Choice::SelectCombo(combo) => {
                game.select_combo(combo, &mut rng).unwrap();
                println!("selected {combo:?}");
            }
            Choice::Reroll1(dice) => {
                game.reroll(&dice, &mut rng).unwrap();
                println!("rerolling {dice:?}");
            }
            Choice::Reroll2(dice) => {
                game.reroll(&dice, &mut rng).unwrap();
                println!("rerolling {dice:?}");
            }
            Choice::Reroll3(dice) => {
                game.reroll(&dice, &mut rng).unwrap();
                println!("rerolling {dice:?}");
            }
            Choice::Reroll4(dice) => {
                game.reroll(&dice, &mut rng).unwrap();
                println!("rerolling {dice:?}");
            }
            Choice::Reroll5(dice) => {
                game.reroll(&dice, &mut rng).unwrap();
                println!("rerolling {dice:?}");
            }
        }
        print_game(game);
    }
}
