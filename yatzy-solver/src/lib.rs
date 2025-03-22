use std::collections::{HashMap, HashSet};

use itertools::Itertools as _;
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};
use yatzy::{Combo, Game};
use yatzy_compute_expected_values::{Choice, GameState, float::prob, state_from_game};

fn expected_score(game: Game, expected_values: &HashMap<GameState, f64>) -> f64 {
    if game.ended() {
        game.score().into()
    } else {
        let state = state_from_game(game);

        let mut value = *expected_values.get(&state).unwrap();
        for combo in Combo::iter() {
            value += f64::from(game.combo(combo).unwrap_or(0));
        }
        value
    }
}

pub fn best_choice_0_rerolls(
    game: Game,
    expected_values: &HashMap<GameState, f64>,
) -> (Choice, f64) {
    let mut best_choice = None;
    let mut max_expected_value = 0_f64;

    for combo in Combo::iter() {
        if game.combo(combo).is_some() {
            continue;
        }
        let mut game = game.clone();
        game.set_combo_raw(combo, Some(combo.points(game.dice())));
        let value = expected_score(game, expected_values);

        if best_choice.is_none() || value > max_expected_value {
            best_choice = Some(Choice::SelectCombo(combo));
            max_expected_value = value;
        }
    }

    (best_choice.unwrap(), max_expected_value)
}

pub fn best_choice_1_reroll(
    game: Game,
    expected_values: &HashMap<GameState, f64>,
) -> (Choice, f64) {
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

    let mut best_choice = None;
    let mut max_expected_value = 0_f64;

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => {
                let mut game = game.clone();
                game.set_combo_raw(combo, Some(combo.points(game.dice())));
                expected_score(game, expected_values)
            }
            Choice::Reroll1(dice) => prob::ROLL_1_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, &new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll2(dice) => prob::ROLL_2_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, &new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll3(dice) => prob::ROLL_3_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, &new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll4(dice) => prob::ROLL_4_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, &new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll5(dice) => prob::ROLL_5_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, &new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values);
                    prob * value
                })
                .sum(),
        };

        if best_choice.is_none() || value > max_expected_value {
            best_choice = Some(choice);
            max_expected_value = value;
        }
    }

    (best_choice.unwrap(), max_expected_value)
}

pub fn best_choice_2_rerolls(
    game: Game,
    expected_values: &HashMap<GameState, f64>,
) -> (Choice, f64) {
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

    let mut best_choice = None;
    let mut max_expected_value = 0_f64;

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => {
                let mut game = game.clone();
                game.set_combo_raw(combo, Some(combo.points(game.dice())));
                expected_score(game, expected_values)
            }
            Choice::Reroll1(dice) => prob::ROLL_1_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, &new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_1_reroll(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll2(dice) => prob::ROLL_2_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, &new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_1_reroll(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll3(dice) => prob::ROLL_3_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, &new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_1_reroll(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll4(dice) => prob::ROLL_4_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, &new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_1_reroll(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll5(dice) => prob::ROLL_5_PROB
                .into_par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, &new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_1_reroll(game, expected_values);
                    prob * value
                })
                .sum(),
        };

        if best_choice.is_none() || value > max_expected_value {
            best_choice = Some(choice);
            max_expected_value = value;
        }
    }

    (best_choice.unwrap(), max_expected_value)
}
