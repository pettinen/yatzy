use std::collections::{HashMap, HashSet};

use itertools::Itertools as _;
use lazy_static::lazy_static;
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};
use yatzy::{Combo, Game};

use crate::{state_from_game, Choice, GameState};

pub mod prob;

lazy_static! {
    static ref CACHE: papaya::HashMap<Game, f64> = papaya::HashMap::new();
}

fn expected_value_0_rerolls(game: Game, expected_values: &HashMap<GameState, f64>) -> f64 {
    let mut max_expected_value = 0_f64;

    for combo in Combo::iter() {
        if game.combo(combo).is_some() {
            continue;
        }
        let mut game = game.clone();
        let combo_points = f64::from(combo.points(game.dice()));
        game.set_combo_raw(combo, Some(0));
        let value = combo_points
            + if game.ended() {
                game.has_bonus().then_some(50_f64).unwrap_or(0_f64)
            } else {
                let state = state_from_game(game);
                *expected_values.get(&state).unwrap()
            };
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    max_expected_value
}

fn expected_value_1_reroll(game: Game, expected_values: &HashMap<GameState, f64>) -> f64 {
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

    let mut max_expected_value = 0_f64;

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => {
                let mut game = game.clone();
                let combo_points = f64::from(combo.points(game.dice()));
                game.set_combo_raw(combo, Some(0));
                combo_points
                    + if game.ended() {
                        game.has_bonus().then_some(50_f64).unwrap_or(0_f64)
                    } else {
                        let state = state_from_game(game);
                        *expected_values.get(&state).unwrap()
                    }
            }
            Choice::Reroll1(dice) => prob::ROLL_1_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(0);
                    prob * *CACHE.pin().get_or_insert_with(game, || {
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
                    prob * *CACHE.pin().get_or_insert_with(game, || {
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
                    prob * *CACHE.pin().get_or_insert_with(game, || {
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
                    prob * *CACHE.pin().get_or_insert_with(game, || {
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
                    prob * *CACHE.pin().get_or_insert_with(game, || {
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

pub fn expected_value_2_rerolls(game: Game, expected_values: &HashMap<GameState, f64>) -> f64 {
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

    let mut max_expected_value = 0_f64;

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => {
                let mut game = game.clone();
                let combo_points = f64::from(combo.points(game.dice()));
                game.set_combo_raw(combo, Some(0));
                combo_points
                    + if game.ended() {
                        game.has_bonus().then_some(50_f64).unwrap_or(0_f64)
                    } else {
                        let state = state_from_game(game);
                        *expected_values.get(&state).unwrap()
                    }
            }
            Choice::Reroll1(dice) => prob::ROLL_1_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(1);
                    prob * *CACHE
                        .pin()
                        .get_or_insert_with(game, || expected_value_1_reroll(game, expected_values))
                })
                .sum(),
            Choice::Reroll2(dice) => prob::ROLL_2_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(1);
                    prob * *CACHE
                        .pin()
                        .get_or_insert_with(game, || expected_value_1_reroll(game, expected_values))
                })
                .sum(),
            Choice::Reroll3(dice) => prob::ROLL_3_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(1);
                    prob * *CACHE
                        .pin()
                        .get_or_insert_with(game, || expected_value_1_reroll(game, expected_values))
                })
                .sum(),
            Choice::Reroll4(dice) => prob::ROLL_4_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(1);
                    prob * *CACHE
                        .pin()
                        .get_or_insert_with(game, || expected_value_1_reroll(game, expected_values))
                })
                .sum(),
            Choice::Reroll5(dice) => prob::ROLL_5_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, &new_dice);
                    game.set_rerolls(1);
                    prob * *CACHE
                        .pin()
                        .get_or_insert_with(game, || expected_value_1_reroll(game, expected_values))
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
