use std::collections::HashMap;

use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_rational::Ratio;
use yatzy::{Combo, Game, print_game};
use yatzy_compute_expected_values::{Choice, GameState};

use yatzy_solver::{best_choice_0_rerolls, best_choice_1_reroll, best_choice_2_rerolls};

lazy_static! {
    static ref EXPECTED_VALUES: papaya::HashMap<GameState, Ratio<BigUint>> =
        papaya::HashMap::with_capacity(958_974);
}

fn main() {
    let expected_values = EXPECTED_VALUES.pin();

    /* TODO uncomment once precomputation finishes
    let map: HashMap<GameState, Ratio<BigUint>> = match std::fs::read("expected-values") {
        Ok(bytes) => match postcard::from_bytes(&bytes) {
            Ok(map) => map,
            Err(error) => {
                eprintln!("failed to read `expected-values`: {error}");
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("failed to open `expected-values`: {error}");
            std::process::exit(1);
        }
    };

    for (k, v) in map {
        expected_values.insert(k, v);
    }
    */

    // TODO remove
    let partial: HashMap<GameState, Ratio<BigUint>> =
        postcard::from_bytes(&std::fs::read("expected-values-partially-computed").unwrap())
            .unwrap();
    for (k, v) in partial {
        expected_values.insert(k, v);
    }

    let mut total = 0_u32;
    let n = 1000;
    for i in 1..=n {
        let score = benchmark(false);
        total += u32::from(score);
        if i % 100 == 0 {
            println!("average: {} (N={})", f64::from(total) / f64::from(i), i);
        }
    }

    //benchmark(true);
}

fn benchmark(print: bool) -> u16 {
    let mut rng = rand::rng();
    let mut game = Game::new_random(&mut rng);

    if print {
        print_game(game);
    }

    while !game.ended() {
        let cache = papaya::HashMap::new();

        let (choice, _) = match game.rerolls_left() {
            0 => best_choice_0_rerolls(game, &EXPECTED_VALUES, &cache),
            1 => best_choice_1_reroll(game, &EXPECTED_VALUES, &cache),
            2 => best_choice_2_rerolls(game, &EXPECTED_VALUES, &cache),
            _ => unreachable!(),
        };

        if print {
            println!();
        }

        match choice {
            Choice::SelectCombo(combo) => {
                game.select_combo(combo, &mut rng).unwrap();
                if print {
                    println!(
                        "Selecting {}",
                        match combo {
                            Combo::Ones => "ones",
                            Combo::Twos => "twos",
                            Combo::Threes => "threes",
                            Combo::Fours => "fours",
                            Combo::Fives => "fives",
                            Combo::Sixes => "sixes",
                            Combo::OnePair => "one pair",
                            Combo::TwoPairs => "two pairs",
                            Combo::ThreeOfAKind => "three of a kind",
                            Combo::FourOfAKind => "four of a kind",
                            Combo::SmallStraight => "small straight",
                            Combo::LargeStraight => "large straight",
                            Combo::FullHouse => "full house",
                            Combo::Chance => "chance",
                            Combo::Yatzy => "Yatzy",
                        },
                    );
                }
            }
            Choice::Reroll1(dice) => {
                game.reroll(&dice, &mut rng).unwrap();
                if print {
                    println!("Rerolling {}", dice[0]);
                }
            }
            Choice::Reroll2(dice) => {
                game.reroll(&dice, &mut rng).unwrap();
                if print {
                    println!("Rerolling {} {}", dice[0], dice[1]);
                }
            }
            Choice::Reroll3(dice) => {
                game.reroll(&dice, &mut rng).unwrap();
                if print {
                    println!("Rerolling {} {} {}", dice[0], dice[1], dice[2]);
                }
            }
            Choice::Reroll4(dice) => {
                game.reroll(&dice, &mut rng).unwrap();
                if print {
                    println!("Rerolling {} {} {} {}", dice[0], dice[1], dice[2], dice[3]);
                }
            }
            Choice::Reroll5(dice) => {
                game.reroll(&dice, &mut rng).unwrap();
                if print {
                    println!("Rerolling {} {} {} {} {}", dice[0], dice[1], dice[2], dice[3], dice[4]);
                }
            }
        }
        if print {
            println!();
            print_game(game);
        }
    }

    game.score()
}
