use std::collections::{HashMap, HashSet};

use itertools::Itertools as _;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_rational::Ratio;
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};
use yatzy::{Combo, Game};

use crate::{Choice, GameState, state_from_game};

pub mod prob;

type ExpectedValue = Ratio<BigUint>;

lazy_static! {
    static ref CACHE: papaya::HashMap<Game, ExpectedValue> = papaya::HashMap::new();
}

fn convert_prob(ratio: Ratio<u16>) -> ExpectedValue {
    let (numer, denom) = ratio.into_raw();
    ExpectedValue::new(numer.into(), denom.into())
}

fn expected_value_0_rerolls(
    game: Game,
    expected_values: &HashMap<GameState, ExpectedValue>,
) -> ExpectedValue {
    let mut max_expected_value = ExpectedValue::new(0_u8.into(), 1_u8.into());

    for combo in Combo::iter() {
        if game.combo(combo).is_some() {
            continue;
        }
        let mut game = game.clone();
        let combo_points = combo.points(game.dice());
        game.set_combo_raw(combo, Some(combo_points));
        let value = ExpectedValue::from(BigUint::from(combo_points))
            + if game.ended() {
                ExpectedValue::from(BigUint::from(
                    game.has_bonus().then_some(50_u8).unwrap_or(0_u8),
                ))
            } else {
                let state = state_from_game(game);
                expected_values.get(&state).unwrap().clone()
            };
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    max_expected_value
}

fn expected_value_1_reroll(
    game: Game,
    expected_values: &HashMap<GameState, ExpectedValue>,
) -> ExpectedValue {
    let mut choices = HashSet::new();

    for combo in Combo::iter() {
        if game.combo(combo).is_none() {
            choices.insert(Choice::SelectCombo(combo));
        }
    }
    for dice in game.dice().into_iter().array_combinations() {
        choices.insert(Choice::Reroll1(dice));
    }
    for dice in game.dice().into_iter().array_combinations() {
        choices.insert(Choice::Reroll2(dice));
    }
    for dice in game.dice().into_iter().array_combinations() {
        choices.insert(Choice::Reroll3(dice));
    }
    for dice in game.dice().into_iter().array_combinations() {
        choices.insert(Choice::Reroll4(dice));
    }
    for dice in game.dice().into_iter().array_combinations() {
        choices.insert(Choice::Reroll5(dice));
    }

    let mut max_expected_value = ExpectedValue::new(0_u8.into(), 1_u8.into());

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => {
                let mut game = game.clone();
                // TODO set combo to actual points so that has_bonus computes properly
                let combo_points = combo.points(game.dice());
                game.set_combo_raw(combo, Some(combo_points));
                ExpectedValue::from(BigUint::from(combo_points))
                    + if game.ended() {
                        ExpectedValue::from(BigUint::from(
                            game.has_bonus().then_some(50_u8).unwrap_or(0_u8),
                        ))
                    } else {
                        let state = state_from_game(game);
                        expected_values.get(&state).unwrap().clone()
                    }
            }
            Choice::Reroll1(dice) => prob::ROLL_1_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(0);
                    convert_prob(prob)
                        * CACHE.pin().get_or_insert_with(game, || {
                            expected_value_0_rerolls(game, expected_values)
                        })
                })
                .sum(),
            Choice::Reroll2(dice) => prob::ROLL_2_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(0);
                    convert_prob(prob)
                        * CACHE.pin().get_or_insert_with(game, || {
                            expected_value_0_rerolls(game, expected_values)
                        })
                })
                .sum(),
            Choice::Reroll3(dice) => prob::ROLL_3_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(0);
                    convert_prob(prob)
                        * CACHE.pin().get_or_insert_with(game, || {
                            expected_value_0_rerolls(game, expected_values)
                        })
                })
                .sum(),
            Choice::Reroll4(dice) => prob::ROLL_4_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(0);
                    convert_prob(prob)
                        * CACHE.pin().get_or_insert_with(game, || {
                            expected_value_0_rerolls(game, expected_values)
                        })
                })
                .sum(),
            Choice::Reroll5(dice) => prob::ROLL_5_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(0);
                    convert_prob(prob)
                        * CACHE.pin().get_or_insert_with(game, || {
                            expected_value_0_rerolls(game, expected_values)
                        })
                })
                .sum(),
        };
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    max_expected_value
}

pub fn expected_value_2_rerolls(
    game: Game,
    expected_values: &HashMap<GameState, ExpectedValue>,
) -> ExpectedValue {
    let mut choices = HashSet::new();

    for combo in Combo::iter() {
        if game.combo(combo).is_none() {
            choices.insert(Choice::SelectCombo(combo));
        }
    }
    for dice in game.dice().into_iter().array_combinations() {
        choices.insert(Choice::Reroll1(dice));
    }
    for dice in game.dice().into_iter().array_combinations() {
        choices.insert(Choice::Reroll2(dice));
    }
    for dice in game.dice().into_iter().array_combinations() {
        choices.insert(Choice::Reroll3(dice));
    }
    for dice in game.dice().into_iter().array_combinations() {
        choices.insert(Choice::Reroll4(dice));
    }
    for dice in game.dice().into_iter().array_combinations() {
        choices.insert(Choice::Reroll5(dice));
    }

    let mut max_expected_value = ExpectedValue::new(0_u8.into(), 1_u8.into());

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => {
                let mut game = game.clone();
                let combo_points = combo.points(game.dice());
                game.set_combo_raw(combo, Some(combo_points));
                ExpectedValue::from(BigUint::from(combo_points))
                    + if game.ended() {
                        ExpectedValue::from(BigUint::from(
                            game.has_bonus().then_some(50_u8).unwrap_or(0_u8),
                        ))
                    } else {
                        let state = state_from_game(game);
                        expected_values.get(&state).unwrap().clone()
                    }
            }
            Choice::Reroll1(dice) => prob::ROLL_1_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(1);
                    convert_prob(prob)
                        * CACHE.pin().get_or_insert_with(game, || {
                            expected_value_1_reroll(game, expected_values)
                        })
                })
                .sum(),
            Choice::Reroll2(dice) => prob::ROLL_2_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(1);
                    convert_prob(prob)
                        * CACHE.pin().get_or_insert_with(game, || {
                            expected_value_1_reroll(game, expected_values)
                        })
                })
                .sum(),
            Choice::Reroll3(dice) => prob::ROLL_3_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(1);
                    convert_prob(prob)
                        * CACHE.pin().get_or_insert_with(game, || {
                            expected_value_1_reroll(game, expected_values)
                        })
                })
                .sum(),
            Choice::Reroll4(dice) => prob::ROLL_4_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(1);
                    convert_prob(prob)
                        * CACHE.pin().get_or_insert_with(game, || {
                            expected_value_1_reroll(game, expected_values)
                        })
                })
                .sum(),
            Choice::Reroll5(dice) => prob::ROLL_5_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(1);
                    convert_prob(prob)
                        * CACHE.pin().get_or_insert_with(game, || {
                            expected_value_1_reroll(game, expected_values)
                        })
                })
                .sum(),
        };
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    max_expected_value
}

pub fn clear_cache() {
    CACHE.pin().clear();
}
