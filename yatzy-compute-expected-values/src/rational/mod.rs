use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasher,
};

use itertools::Itertools as _;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_rational::Ratio;
use rustc_hash::FxBuildHasher;
use yatzy::{Combo, Dice, Die, Game};

use crate::{Choice, GameState, state_from_game};

pub mod prob;

type ExpectedValue = Ratio<BigUint>;

fn convert_prob(ratio: Ratio<u16>) -> ExpectedValue {
    let (numer, denom) = ratio.into_raw();
    ExpectedValue::new(numer.into(), denom.into())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CacheKey {
    Reroll(Game),
    Retain0(u8, GameState),
    Retain1(u8, [Die; 1], GameState),
    Retain2(u8, [Die; 2], GameState),
    Retain3(u8, [Die; 3], GameState),
    Retain4(u8, [Die; 4], GameState),
    SelectCombo(Combo, u8, Dice, GameState),
    ZeroRerolls(Game),
}

lazy_static! {
    static ref ROLL_1_PROB: Vec<([Die; 1], ExpectedValue)> = prob::ROLL_1_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob(prob)))
        .collect();
    static ref ROLL_2_PROB: Vec<([Die; 2], ExpectedValue)> = prob::ROLL_2_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob(prob)))
        .collect();
    static ref ROLL_3_PROB: Vec<([Die; 3], ExpectedValue)> = prob::ROLL_3_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob(prob)))
        .collect();
    static ref ROLL_4_PROB: Vec<([Die; 4], ExpectedValue)> = prob::ROLL_4_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob(prob)))
        .collect();
    static ref ROLL_5_PROB: Vec<([Die; 5], ExpectedValue)> = prob::ROLL_5_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob(prob)))
        .collect();
}

fn expected_value_0_rerolls<S1: BuildHasher, S2: BuildHasher>(
    game: Game,
    expected_values: &HashMap<GameState, ExpectedValue, S1>,
    cache: &papaya::HashMap<CacheKey, ExpectedValue, S2>,
) -> ExpectedValue {
    if let Some(value) = cache.pin().get(&CacheKey::ZeroRerolls(game)) {
        return value.clone();
    }

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

    cache
        .pin()
        .insert(CacheKey::ZeroRerolls(game), max_expected_value.clone());
    max_expected_value
}

fn expected_value_1_reroll<S1: BuildHasher, S2: BuildHasher>(
    game: Game,
    expected_values: &HashMap<GameState, ExpectedValue, S1>,
    cache: &papaya::HashMap<CacheKey, ExpectedValue, S2>,
) -> ExpectedValue {
    let mut choices = HashSet::with_hasher(FxBuildHasher);

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

    let original_dice = game.dice();
    let state = state_from_game(game);

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => {
                if let Some(value) =
                    cache
                        .pin()
                        .get(&CacheKey::SelectCombo(combo, 1, original_dice, state))
                {
                    value.clone()
                } else {
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
                    cache.pin().insert(
                        CacheKey::SelectCombo(combo, 1, original_dice, state),
                        value.clone(),
                    );
                    value
                }
            }
            Choice::Reroll1(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.swap_remove(
                        retained_dice
                            .iter()
                            .position(|&die| die == rerolled_die)
                            .unwrap(),
                    );
                }
                let mut retained_dice: [Die; 4] = retained_dice.try_into().unwrap();
                retained_dice.sort_unstable();

                if let Some(value) = cache.pin().get(&CacheKey::Retain4(1, retained_dice, state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_1_PROB
                        .iter()
                        .map(|(new_dice, prob)| {
                            let mut game = game.clone();
                            _ = game.replace_dice(&dice, new_dice);
                            game.set_rerolls(0);
                            prob * cache.pin().get_or_insert_with(CacheKey::Reroll(game), || {
                                expected_value_0_rerolls(game, expected_values, cache)
                            })
                        })
                        .sum();
                    cache
                        .pin()
                        .insert(CacheKey::Retain4(1, retained_dice, state), value.clone());
                    value
                }
            }
            Choice::Reroll2(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.swap_remove(
                        retained_dice
                            .iter()
                            .position(|&die| die == rerolled_die)
                            .unwrap(),
                    );
                }
                let mut retained_dice: [Die; 3] = retained_dice.try_into().unwrap();
                retained_dice.sort_unstable();

                if let Some(value) = cache.pin().get(&CacheKey::Retain3(1, retained_dice, state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_2_PROB
                        .iter()
                        .map(|(new_dice, prob)| {
                            let mut game = game.clone();
                            _ = game.replace_dice(&dice, new_dice);
                            game.set_rerolls(0);
                            prob * cache.pin().get_or_insert_with(CacheKey::Reroll(game), || {
                                expected_value_0_rerolls(game, expected_values, cache)
                            })
                        })
                        .sum();
                    cache
                        .pin()
                        .insert(CacheKey::Retain3(1, retained_dice, state), value.clone());
                    value
                }
            }
            Choice::Reroll3(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.swap_remove(
                        retained_dice
                            .iter()
                            .position(|&die| die == rerolled_die)
                            .unwrap(),
                    );
                }
                let mut retained_dice: [Die; 2] = retained_dice.try_into().unwrap();
                retained_dice.sort_unstable();

                if let Some(value) = cache.pin().get(&CacheKey::Retain2(1, retained_dice, state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_3_PROB
                        .iter()
                        .map(|(new_dice, prob)| {
                            let mut game = game.clone();
                            _ = game.replace_dice(&dice, new_dice);
                            game.set_rerolls(0);
                            prob * cache.pin().get_or_insert_with(CacheKey::Reroll(game), || {
                                expected_value_0_rerolls(game, expected_values, cache)
                            })
                        })
                        .sum();
                    cache
                        .pin()
                        .insert(CacheKey::Retain2(1, retained_dice, state), value.clone());
                    value
                }
            }
            Choice::Reroll4(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.swap_remove(
                        retained_dice
                            .iter()
                            .position(|&die| die == rerolled_die)
                            .unwrap(),
                    );
                }
                let retained_dice: [Die; 1] = retained_dice.try_into().unwrap();

                if let Some(value) = cache.pin().get(&CacheKey::Retain1(1, retained_dice, state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_4_PROB
                        .iter()
                        .map(|(new_dice, prob)| {
                            let mut game = game.clone();
                            _ = game.replace_dice(&dice, new_dice);
                            game.set_rerolls(0);
                            prob * cache.pin().get_or_insert_with(CacheKey::Reroll(game), || {
                                expected_value_0_rerolls(game, expected_values, cache)
                            })
                        })
                        .sum();
                    cache
                        .pin()
                        .insert(CacheKey::Retain1(1, retained_dice, state), value.clone());
                    value
                }
            }
            Choice::Reroll5(dice) => {
                if let Some(value) = cache.pin().get(&CacheKey::Retain0(1, state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_5_PROB
                        .iter()
                        .map(|(new_dice, prob)| {
                            let mut game = game.clone();
                            _ = game.replace_dice(&dice, new_dice);
                            game.set_rerolls(0);
                            prob * cache.pin().get_or_insert_with(CacheKey::Reroll(game), || {
                                expected_value_0_rerolls(game, expected_values, cache)
                            })
                        })
                        .sum();
                    cache
                        .pin()
                        .insert(CacheKey::Retain0(1, state), value.clone());
                    value
                }
            }
        };

        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    max_expected_value
}

pub fn expected_value_2_rerolls<S1: BuildHasher, S2: BuildHasher>(
    game: Game,
    expected_values: &HashMap<GameState, ExpectedValue, S1>,
    cache: &papaya::HashMap<CacheKey, ExpectedValue, S2>,
) -> ExpectedValue {
    let mut choices = HashSet::with_hasher(FxBuildHasher);

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

    let original_dice = game.dice();
    let state = state_from_game(game);

    let mut max_expected_value = ExpectedValue::new(0_u8.into(), 1_u8.into());

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => {
                if let Some(value) =
                    cache
                        .pin()
                        .get(&CacheKey::SelectCombo(combo, 2, original_dice, state))
                {
                    value.clone()
                } else {
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
                    cache.pin().insert(
                        CacheKey::SelectCombo(combo, 2, original_dice, state),
                        value.clone(),
                    );
                    value
                }
            }
            Choice::Reroll1(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.swap_remove(
                        retained_dice
                            .iter()
                            .position(|&die| die == rerolled_die)
                            .unwrap(),
                    );
                }
                let mut retained_dice: [Die; 4] = retained_dice.try_into().unwrap();
                retained_dice.sort_unstable();

                if let Some(value) = cache.pin().get(&CacheKey::Retain4(2, retained_dice, state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_1_PROB
                        .iter()
                        .map(|(new_dice, prob)| {
                            let mut game = game.clone();
                            _ = game.replace_dice(&dice, new_dice);
                            game.set_rerolls(1);
                            prob * cache.pin().get_or_insert_with(CacheKey::Reroll(game), || {
                                expected_value_1_reroll(game, expected_values, cache)
                            })
                        })
                        .sum();
                    cache
                        .pin()
                        .insert(CacheKey::Retain4(2, retained_dice, state), value.clone());
                    value
                }
            }
            Choice::Reroll2(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.swap_remove(
                        retained_dice
                            .iter()
                            .position(|&die| die == rerolled_die)
                            .unwrap(),
                    );
                }
                let mut retained_dice: [Die; 3] = retained_dice.try_into().unwrap();
                retained_dice.sort_unstable();

                if let Some(value) = cache.pin().get(&CacheKey::Retain3(2, retained_dice, state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_2_PROB
                        .iter()
                        .map(|(new_dice, prob)| {
                            let mut game = game.clone();
                            _ = game.replace_dice(&dice, new_dice);
                            game.set_rerolls(1);
                            prob * cache.pin().get_or_insert_with(CacheKey::Reroll(game), || {
                                expected_value_1_reroll(game, expected_values, cache)
                            })
                        })
                        .sum();
                    cache
                        .pin()
                        .insert(CacheKey::Retain3(2, retained_dice, state), value.clone());
                    value
                }
            }
            Choice::Reroll3(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.swap_remove(
                        retained_dice
                            .iter()
                            .position(|&die| die == rerolled_die)
                            .unwrap(),
                    );
                }
                let mut retained_dice: [Die; 2] = retained_dice.try_into().unwrap();
                retained_dice.sort_unstable();

                if let Some(value) = cache.pin().get(&CacheKey::Retain2(2, retained_dice, state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_3_PROB
                        .iter()
                        .map(|(new_dice, prob)| {
                            let mut game = game.clone();
                            _ = game.replace_dice(&dice, new_dice);
                            game.set_rerolls(1);
                            prob * cache.pin().get_or_insert_with(CacheKey::Reroll(game), || {
                                expected_value_1_reroll(game, expected_values, cache)
                            })
                        })
                        .sum();
                    cache
                        .pin()
                        .insert(CacheKey::Retain2(2, retained_dice, state), value.clone());
                    value
                }
            }
            Choice::Reroll4(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.swap_remove(
                        retained_dice
                            .iter()
                            .position(|&die| die == rerolled_die)
                            .unwrap(),
                    );
                }
                let retained_dice = retained_dice.try_into().unwrap();

                if let Some(value) = cache.pin().get(&CacheKey::Retain1(2, retained_dice, state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_4_PROB
                        .iter()
                        .map(|(new_dice, prob)| {
                            let mut game = game.clone();
                            _ = game.replace_dice(&dice, new_dice);
                            game.set_rerolls(1);
                            prob * cache.pin().get_or_insert_with(CacheKey::Reroll(game), || {
                                expected_value_1_reroll(game, expected_values, cache)
                            })
                        })
                        .sum();
                    cache
                        .pin()
                        .insert(CacheKey::Retain1(2, retained_dice, state), value.clone());
                    value
                }
            }
            Choice::Reroll5(dice) => {
                if let Some(value) = cache.pin().get(&CacheKey::Retain0(2, state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_5_PROB
                        .iter()
                        .map(|(new_dice, prob)| {
                            let mut game = game.clone();
                            _ = game.replace_dice(&dice, new_dice);
                            game.set_rerolls(1);
                            prob * cache.pin().get_or_insert_with(CacheKey::Reroll(game), || {
                                expected_value_1_reroll(game, expected_values, cache)
                            })
                        })
                        .sum();
                    cache
                        .pin()
                        .insert(CacheKey::Retain0(2, state), value.clone());
                    value
                }
            }
        };

        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    max_expected_value
}
