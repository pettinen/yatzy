use std::collections::HashMap;

use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_rational::Ratio;
use num_traits::ToPrimitive as _;
use rustc_hash::FxBuildHasher;
use yatzy::{Combo, Game, print_game};
use yatzy_compute_expected_values::{Choice, GameState};

use yatzy_solver::{best_choice_0_rerolls, best_choice_1_reroll, best_choice_2_rerolls};

lazy_static! {
    static ref EXPECTED_VALUES: papaya::HashMap<GameState, f64, FxBuildHasher> =
        papaya::HashMap::with_capacity_and_hasher(958_974, FxBuildHasher);
}

fn main() {
    let expected_values = EXPECTED_VALUES.pin();

    let map: HashMap<GameState, Ratio<BigUint>, FxBuildHasher> =
        match std::fs::read("expected-values") {
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
        expected_values.insert(k, v.to_f64().unwrap());
    }

    let mut total = 0_u64;
    let n = 10_000;
    for i in 1..=n {
        let score = benchmark(false);
        total += u64::from(score);
        if i % 100 == 0 {
            println!("average: {} (N={})", (total as f64) / f64::from(i), i);
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
        let cache = papaya::HashMap::with_hasher(FxBuildHasher);

        let (choices, _) = match game.rerolls_left() {
            0 => best_choice_0_rerolls::<_, FxBuildHasher, _, f64>(game, &EXPECTED_VALUES, &cache),
            1 => best_choice_1_reroll::<_, FxBuildHasher, _, f64>(game, &EXPECTED_VALUES, &cache),
            2 => best_choice_2_rerolls::<_, FxBuildHasher, _, f64>(game, &EXPECTED_VALUES, &cache),
            _ => unreachable!(),
        };
        let choice = choices.into_iter().next().unwrap();

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
                    println!(
                        "Rerolling {} {} {} {} {}",
                        dice[0], dice[1], dice[2], dice[3], dice[4],
                    );
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
