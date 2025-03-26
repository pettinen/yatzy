use std::collections::{HashMap, HashSet};

use itertools::Itertools as _;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_rational::Ratio;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use yatzy::{Combo, Dice, Die, Game};

use crate::{Choice, GameState, state_from_game};

pub mod prob;

type ExpectedValue = Ratio<BigUint>;

fn convert_prob(ratio: Ratio<u16>) -> ExpectedValue {
    let (numer, denom) = ratio.into_raw();
    ExpectedValue::new(numer.into(), denom.into())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CacheKey {
    Retain0(u8),
    Retain1(u8, [Die; 1]),
    Retain2(u8, [Die; 2]),
    Retain3(u8, [Die; 3]),
    Retain4(u8, [Die; 4]),
    SelectCombo(Combo, u8, Dice),
    SingleRoll(u8, Dice),
}

lazy_static! {
    static ref CACHE: papaya::HashMap<(CacheKey, GameState), ExpectedValue> = papaya::HashMap::new();
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

    let original_dice = game.dice();
    let state = state_from_game(game);

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => if let Some(value) = CACHE.pin().get(&(CacheKey::SelectCombo(combo, 1, original_dice), state)) {
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
                CACHE.pin().insert((CacheKey::SelectCombo(combo, 1, original_dice), state), value.clone());
                value
            },
            Choice::Reroll1(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.remove(retained_dice.iter().position(|&die| die == rerolled_die).unwrap());
                }
                let retained_dice = retained_dice.try_into().unwrap();

                if let Some(value) = CACHE.pin().get(&(CacheKey::Retain4(1, retained_dice), state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_1_PROB
                    .par_iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        _ = game.replace_dice(&dice, new_dice);
                        game.set_rerolls(0);
                        prob * CACHE.pin().get_or_insert_with((CacheKey::SingleRoll(0, game.dice()), state), || {
                            expected_value_0_rerolls(game, expected_values)
                        })
                    })
                    .sum();
                    CACHE.pin().insert((CacheKey::Retain4(1, retained_dice), state), value.clone());
                    value
                }
            }
            Choice::Reroll2(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.remove(retained_dice.iter().position(|&die| die == rerolled_die).unwrap());
                }
                let retained_dice = retained_dice.try_into().unwrap();

                if let Some(value) = CACHE.pin().get(&(CacheKey::Retain3(1, retained_dice), state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_2_PROB
                    .par_iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        _ = game.replace_dice(&dice, new_dice);
                        game.set_rerolls(0);
                        prob * CACHE.pin().get_or_insert_with((CacheKey::SingleRoll(0, game.dice()), state), || {
                            expected_value_0_rerolls(game, expected_values)
                        })
                    })
                    .sum();
                    CACHE.pin().insert((CacheKey::Retain3(1, retained_dice), state), value.clone());
                    value
                }
            }
            Choice::Reroll3(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.remove(retained_dice.iter().position(|&die| die == rerolled_die).unwrap());
                }
                let retained_dice = retained_dice.try_into().unwrap();

                if let Some(value) = CACHE.pin().get(&(CacheKey::Retain2(1, retained_dice), state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_3_PROB
                    .par_iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        _ = game.replace_dice(&dice, new_dice);
                        game.set_rerolls(0);
                        prob * CACHE.pin().get_or_insert_with((CacheKey::SingleRoll(0, game.dice()), state), || {
                            expected_value_0_rerolls(game, expected_values)
                        })
                    })
                    .sum();
                    CACHE.pin().insert((CacheKey::Retain2(1, retained_dice), state), value.clone());
                    value
                }
            }
            Choice::Reroll4(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.remove(retained_dice.iter().position(|&die| die == rerolled_die).unwrap());
                }
                let retained_dice = retained_dice.try_into().unwrap();

                if let Some(value) = CACHE.pin().get(&(CacheKey::Retain1(1, retained_dice), state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_4_PROB
                    .par_iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        _ = game.replace_dice(&dice, new_dice);
                        game.set_rerolls(0);
                        prob * CACHE.pin().get_or_insert_with((CacheKey::SingleRoll(0, game.dice()), state), || {
                            expected_value_0_rerolls(game, expected_values)
                        })
                    })
                    .sum();
                    CACHE.pin().insert((CacheKey::Retain1(1, retained_dice), state), value.clone());
                    value
                }
            }
            Choice::Reroll5(dice) => if let Some(value) = CACHE.pin().get(&(CacheKey::Retain0(1), state)) {
                value.clone()
            } else {
                let value: ExpectedValue = ROLL_5_PROB
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, new_dice);
                    game.set_rerolls(0);
                    prob * CACHE.pin().get_or_insert_with((CacheKey::SingleRoll(0, game.dice()), state), || {
                        expected_value_0_rerolls(game, expected_values)
                    })
                })
                .sum();
                CACHE.pin().insert((CacheKey::Retain0(1), state), value.clone());
                value
            },
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

    let original_dice = game.dice();
    let state = state_from_game(game);

    let mut max_expected_value = ExpectedValue::new(0_u8.into(), 1_u8.into());

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => if let Some(value) = CACHE.pin().get(&(CacheKey::SelectCombo(combo, 2, original_dice), state)) {
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
                CACHE.pin().insert((CacheKey::SelectCombo(combo, 2, original_dice), state), value.clone());
                value
            },
            Choice::Reroll1(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.remove(retained_dice.iter().position(|&die| die == rerolled_die).unwrap());
                }
                let retained_dice = retained_dice.try_into().unwrap();

                if let Some(value) = CACHE.pin().get(&(CacheKey::Retain4(2, retained_dice), state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_1_PROB
                    .par_iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        _ = game.replace_dice(&dice, new_dice);
                        game.set_rerolls(1);
                        prob * CACHE
                            .pin()
                            .get_or_insert_with((CacheKey::SingleRoll(1, game.dice()), state), || expected_value_1_reroll(game, expected_values))
                    })
                    .sum();
                    CACHE.pin().insert((CacheKey::Retain4(2, retained_dice), state), value.clone());
                    value
                }
            }
            Choice::Reroll2(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.remove(retained_dice.iter().position(|&die| die == rerolled_die).unwrap());
                }
                let retained_dice = retained_dice.try_into().unwrap();

                if let Some(value) = CACHE.pin().get(&(CacheKey::Retain3(2, retained_dice), state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_2_PROB
                    .par_iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        _ = game.replace_dice(&dice, new_dice);
                        game.set_rerolls(1);
                        prob * CACHE
                            .pin()
                            .get_or_insert_with((CacheKey::SingleRoll(1, game.dice()), state), || expected_value_1_reroll(game, expected_values))
                    })
                    .sum();
                    CACHE.pin().insert((CacheKey::Retain3(2, retained_dice), state), value.clone());
                    value
                }
            }
            Choice::Reroll3(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.remove(retained_dice.iter().position(|&die| die == rerolled_die).unwrap());
                }
                let retained_dice = retained_dice.try_into().unwrap();

                if let Some(value) = CACHE.pin().get(&(CacheKey::Retain2(2, retained_dice), state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_3_PROB
                    .par_iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        _ = game.replace_dice(&dice, new_dice);
                        game.set_rerolls(1);
                        prob * CACHE
                            .pin()
                            .get_or_insert_with((CacheKey::SingleRoll(1, game.dice()), state), || expected_value_1_reroll(game, expected_values))
                    })
                    .sum();
                    CACHE.pin().insert((CacheKey::Retain2(2, retained_dice), state), value.clone());
                    value
                }
            }
            Choice::Reroll4(dice) => {
                let mut retained_dice = original_dice.into_iter().collect::<Vec<_>>();
                for rerolled_die in dice {
                    retained_dice.remove(retained_dice.iter().position(|&die| die == rerolled_die).unwrap());
                }
                let retained_dice = retained_dice.try_into().unwrap();

                if let Some(value) = CACHE.pin().get(&(CacheKey::Retain1(2, retained_dice), state)) {
                    value.clone()
                } else {
                    let value: ExpectedValue = ROLL_4_PROB
                    .par_iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        _ = game.replace_dice(&dice, new_dice);
                        game.set_rerolls(1);
                        prob * CACHE
                            .pin()
                            .get_or_insert_with((CacheKey::SingleRoll(1, game.dice()), state), || expected_value_1_reroll(game, expected_values))
                    })
                    .sum();
                    CACHE.pin().insert((CacheKey::Retain1(2, retained_dice), state), value.clone());
                    value
                }
            }
            Choice::Reroll5(dice) => if let Some(value) = CACHE.pin().get(&(CacheKey::Retain0(2), state)) {
                value.clone()
            } else {
                let value: ExpectedValue = ROLL_5_PROB
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    _ = game.replace_dice(&dice, new_dice);
                    game.set_rerolls(1);
                    prob * CACHE
                        .pin()
                        .get_or_insert_with((CacheKey::SingleRoll(1, game.dice()), state), || expected_value_1_reroll(game, expected_values))
                })
                .sum();
                CACHE.pin().insert((CacheKey::Retain0(2), state), value.clone());
                value
            },
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
