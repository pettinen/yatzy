use std::collections::{HashMap, HashSet};

use itertools::Itertools as _;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_rational::Ratio;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use yatzy::{Combo, Die, Game};
use yatzy_compute_expected_values::{Choice, GameState, rational::prob, state_from_game};

fn convert_prob(ratio: Ratio<u16>) -> Ratio<BigUint> {
    let (numer, denom) = ratio.into_raw();
    Ratio::new(numer.into(), denom.into())
}

lazy_static! {
    static ref ROLL_1_PROB: Vec<([Die; 1], Ratio<BigUint>)> = prob::ROLL_1_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob(prob)))
        .collect();
    static ref ROLL_2_PROB: Vec<([Die; 2], Ratio<BigUint>)> = prob::ROLL_2_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob(prob)))
        .collect();
    static ref ROLL_3_PROB: Vec<([Die; 3], Ratio<BigUint>)> = prob::ROLL_3_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob(prob)))
        .collect();
    static ref ROLL_4_PROB: Vec<([Die; 4], Ratio<BigUint>)> = prob::ROLL_4_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob(prob)))
        .collect();
    static ref ROLL_5_PROB: Vec<([Die; 5], Ratio<BigUint>)> = prob::ROLL_5_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob(prob)))
        .collect();
}

fn expected_score(
    game: Game,
    expected_values: &HashMap<GameState, Ratio<BigUint>>,
) -> Ratio<BigUint> {
    if game.ended() {
        Ratio::from(BigUint::from(game.score()))
    } else {
        let state = state_from_game(game);

        let mut value = expected_values.get(&state).unwrap().clone();
        for combo in Combo::iter() {
            value += Ratio::from(BigUint::from(game.combo(combo).unwrap_or(0)));
        }
        value
    }
}

pub fn best_choice_0_rerolls(
    game: Game,
    expected_values: &HashMap<GameState, Ratio<BigUint>>,
) -> (Choice, Ratio<BigUint>) {
    let mut best_choice = None;
    let mut max_expected_value = Ratio::from(BigUint::from(0_u8));

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
    expected_values: &HashMap<GameState, Ratio<BigUint>>,
) -> (Choice, Ratio<BigUint>) {
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
    let mut max_expected_value = Ratio::from(BigUint::from(0_u8));

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => {
                let mut game = game.clone();
                game.set_combo_raw(combo, Some(combo.points(game.dice())));
                expected_score(game, expected_values)
            }
            Choice::Reroll1(dice) => ROLL_1_PROB
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll2(dice) => ROLL_2_PROB
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll3(dice) => ROLL_3_PROB
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll4(dice) => ROLL_4_PROB
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll5(dice) => ROLL_5_PROB
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
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
    expected_values: &HashMap<GameState, Ratio<BigUint>>,
) -> (Choice, Ratio<BigUint>) {
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
    let mut max_expected_value = Ratio::from(BigUint::from(0_u8));

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => {
                let mut game = game.clone();
                game.set_combo_raw(combo, Some(combo.points(game.dice())));
                expected_score(game, expected_values)
            }
            Choice::Reroll1(dice) => ROLL_1_PROB
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_1_reroll(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll2(dice) => ROLL_2_PROB
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_1_reroll(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll3(dice) => ROLL_3_PROB
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_1_reroll(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll4(dice) => ROLL_4_PROB
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_1_reroll(game, expected_values);
                    prob * value
                })
                .sum(),
            Choice::Reroll5(dice) => ROLL_5_PROB
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
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
