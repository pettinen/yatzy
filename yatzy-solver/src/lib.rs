use std::{
    collections::HashSet,
    hash::BuildHasher,
    iter::Sum,
    ops::{AddAssign, Mul},
};

use itertools::Itertools as _;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_rational::Ratio;
use num_traits::ToPrimitive as _;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use rustc_hash::FxBuildHasher;
use yatzy::{Combo, Die, Game};
use yatzy_compute_expected_values::{rational::prob, state_from_game};

pub use yatzy_compute_expected_values::{Choice, GameState};

pub trait Value: Sized {
    fn from_u8(input: u8) -> Self;
    fn from_u16(input: u16) -> Self;
    fn roll_1_prob<'a>() -> &'a Vec<([Die; 1], Self)>;
    fn roll_2_prob<'a>() -> &'a Vec<([Die; 2], Self)>;
    fn roll_3_prob<'a>() -> &'a Vec<([Die; 3], Self)>;
    fn roll_4_prob<'a>() -> &'a Vec<([Die; 4], Self)>;
    fn roll_5_prob<'a>() -> &'a Vec<([Die; 5], Self)>;
    fn zero() -> Self;
}

impl Value for f64 {
    fn from_u8(input: u8) -> Self {
        input.into()
    }

    fn from_u16(input: u16) -> Self {
        input.into()
    }

    fn roll_1_prob<'a>() -> &'a Vec<([Die; 1], Self)> {
        &ROLL_1_PROB_FLOAT
    }

    fn roll_2_prob<'a>() -> &'a Vec<([Die; 2], Self)> {
        &ROLL_2_PROB_FLOAT
    }

    fn roll_3_prob<'a>() -> &'a Vec<([Die; 3], Self)> {
        &ROLL_3_PROB_FLOAT
    }

    fn roll_4_prob<'a>() -> &'a Vec<([Die; 4], Self)> {
        &ROLL_4_PROB_FLOAT
    }

    fn roll_5_prob<'a>() -> &'a Vec<([Die; 5], Self)> {
        &ROLL_5_PROB_FLOAT
    }

    fn zero() -> Self {
        0.0
    }
}

impl Value for Ratio<BigUint> {
    fn from_u8(input: u8) -> Self {
        Ratio::from(BigUint::from(input))
    }

    fn from_u16(input: u16) -> Self {
        Ratio::from(BigUint::from(input))
    }

    fn roll_1_prob<'a>() -> &'a Vec<([Die; 1], Self)> {
        &ROLL_1_PROB_RATIO
    }

    fn roll_2_prob<'a>() -> &'a Vec<([Die; 2], Self)> {
        &ROLL_2_PROB_RATIO
    }

    fn roll_3_prob<'a>() -> &'a Vec<([Die; 3], Self)> {
        &ROLL_3_PROB_RATIO
    }

    fn roll_4_prob<'a>() -> &'a Vec<([Die; 4], Self)> {
        &ROLL_4_PROB_RATIO
    }

    fn roll_5_prob<'a>() -> &'a Vec<([Die; 5], Self)> {
        &ROLL_5_PROB_RATIO
    }

    fn zero() -> Self {
        Self::default()
    }
}

fn convert_prob_to_ratio(ratio: Ratio<u16>) -> Ratio<BigUint> {
    let (numer, denom) = ratio.into_raw();
    Ratio::new(numer.into(), denom.into())
}

lazy_static! {
    static ref ROLL_1_PROB_RATIO: Vec<([Die; 1], Ratio<BigUint>)> = prob::ROLL_1_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob_to_ratio(prob)))
        .collect();
    static ref ROLL_2_PROB_RATIO: Vec<([Die; 2], Ratio<BigUint>)> = prob::ROLL_2_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob_to_ratio(prob)))
        .collect();
    static ref ROLL_3_PROB_RATIO: Vec<([Die; 3], Ratio<BigUint>)> = prob::ROLL_3_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob_to_ratio(prob)))
        .collect();
    static ref ROLL_4_PROB_RATIO: Vec<([Die; 4], Ratio<BigUint>)> = prob::ROLL_4_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob_to_ratio(prob)))
        .collect();
    static ref ROLL_5_PROB_RATIO: Vec<([Die; 5], Ratio<BigUint>)> = prob::ROLL_5_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, convert_prob_to_ratio(prob)))
        .collect();
    static ref ROLL_1_PROB_FLOAT: Vec<([Die; 1], f64)> = prob::ROLL_1_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, prob.to_f64().unwrap()))
        .collect();
    static ref ROLL_2_PROB_FLOAT: Vec<([Die; 2], f64)> = prob::ROLL_2_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, prob.to_f64().unwrap()))
        .collect();
    static ref ROLL_3_PROB_FLOAT: Vec<([Die; 3], f64)> = prob::ROLL_3_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, prob.to_f64().unwrap()))
        .collect();
    static ref ROLL_4_PROB_FLOAT: Vec<([Die; 4], f64)> = prob::ROLL_4_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, prob.to_f64().unwrap()))
        .collect();
    static ref ROLL_5_PROB_FLOAT: Vec<([Die; 5], f64)> = prob::ROLL_5_PROB
        .into_iter()
        .map(|(dice, prob)| (dice, prob.to_f64().unwrap()))
        .collect();
}

fn expected_score<S, V>(game: Game, expected_values: &papaya::HashMap<GameState, V, S>) -> V
where
    S: BuildHasher,
    V: Value + AddAssign + Clone,
{
    if game.ended() {
        V::from_u16(game.score())
    } else {
        let state = state_from_game(game);

        let mut value = expected_values.pin().get(&state).unwrap().clone();
        for combo in Combo::iter() {
            value += V::from_u8(game.combo(combo).unwrap_or(0));
        }
        value
    }
}

pub fn best_choice_0_rerolls<S1, S2, S3, V>(
    game: Game,
    expected_values: &papaya::HashMap<GameState, V, S1>,
    cache: &papaya::HashMap<(Game, Option<Choice>), (Option<HashSet<Choice, S2>>, V), S3>,
) -> (HashSet<Choice, S2>, V)
where
    S1: BuildHasher,
    S2: BuildHasher + Clone + Default,
    S3: BuildHasher,
    V: Value + AddAssign + Clone + PartialOrd,
    for<'a> &'a V: PartialEq<&'a V>,
{
    assert!(game.rerolls_left() == 0);

    if let Some((Some(choices), value)) = cache.pin().get(&(game, None)) {
        return (choices.clone(), value.clone());
    }

    let mut best_choices = Vec::new();
    let mut max_expected_value = V::zero();

    for combo in Combo::iter() {
        if game.combo(combo).is_some() {
            continue;
        }
        let mut game = game.clone();
        game.set_combo_raw(combo, Some(combo.points(game.dice())));
        let value = expected_score(game, expected_values);

        if value >= max_expected_value {
            best_choices.push((Choice::SelectCombo(combo), value.clone()));
            max_expected_value = value;
        }
    }

    let best_choices: HashSet<Choice, S2> = best_choices
        .into_iter()
        .filter_map(|(choice, value)| (value == max_expected_value).then_some(choice))
        .collect();
    cache.pin().insert(
        (game, None),
        (Some(best_choices.clone()), max_expected_value.clone()),
    );
    (best_choices, max_expected_value)
}

fn best_choice_1_reroll_non_parallel<S1, S2, S3, V>(
    game: Game,
    expected_values: &papaya::HashMap<GameState, V, S1>,
    cache: &papaya::HashMap<(Game, Option<Choice>), (Option<HashSet<Choice, S2>>, V), S3>,
) -> (HashSet<Choice, S2>, V)
where
    S1: BuildHasher,
    S2: BuildHasher + Clone + Default,
    S3: BuildHasher,
    V: Value + AddAssign + Clone + PartialOrd + for<'a> Sum<<&'a V as Mul<V>>::Output>,
    for<'a> &'a V: Mul<V> + PartialEq<&'a V>,
{
    assert!(game.rerolls_left() == 1);

    if let Some((Some(choices), value)) = cache.pin().get(&(game, None)) {
        return (choices.clone(), value.clone());
    }

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

    let mut best_choices = Vec::new();
    let mut max_expected_value = V::zero();

    for choice in choices {
        let value = if let Some((None, value)) = cache.pin().get(&(game, Some(choice))) {
            value.clone()
        } else {
            let value = match choice {
                Choice::SelectCombo(combo) => {
                    let mut game = game.clone();
                    game.set_combo_raw(combo, Some(combo.points(game.dice())));
                    expected_score(game, expected_values)
                }
                Choice::Reroll1(dice) => V::roll_1_prob()
                    .iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        game.replace_dice(&dice, new_dice).unwrap();
                        game.set_rerolls(0);
                        let (_, value) = best_choice_0_rerolls(game, expected_values, cache);
                        prob * value
                    })
                    .sum(),
                Choice::Reroll2(dice) => V::roll_2_prob()
                    .iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        game.replace_dice(&dice, new_dice).unwrap();
                        game.set_rerolls(0);
                        let (_, value) = best_choice_0_rerolls(game, expected_values, cache);
                        prob * value
                    })
                    .sum(),
                Choice::Reroll3(dice) => V::roll_3_prob()
                    .iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        game.replace_dice(&dice, new_dice).unwrap();
                        game.set_rerolls(0);
                        let (_, value) = best_choice_0_rerolls(game, expected_values, cache);
                        prob * value
                    })
                    .sum(),
                Choice::Reroll4(dice) => V::roll_4_prob()
                    .iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        game.replace_dice(&dice, new_dice).unwrap();
                        game.set_rerolls(0);
                        let (_, value) = best_choice_0_rerolls(game, expected_values, cache);
                        prob * value
                    })
                    .sum(),
                Choice::Reroll5(dice) => V::roll_5_prob()
                    .iter()
                    .map(|(new_dice, prob)| {
                        let mut game = game.clone();
                        game.replace_dice(&dice, new_dice).unwrap();
                        game.set_rerolls(0);
                        let (_, value) = best_choice_0_rerolls(game, expected_values, cache);
                        prob * value
                    })
                    .sum(),
            };
            cache
                .pin()
                .insert((game, Some(choice)), (None, value.clone()));
            value
        };

        if value >= max_expected_value {
            best_choices.push((choice, value.clone()));
            max_expected_value = value;
        }
    }

    let best_choices: HashSet<Choice, S2> = best_choices
        .into_iter()
        .filter_map(|(choice, value)| (value == max_expected_value).then_some(choice))
        .collect();
    cache.pin().insert(
        (game, None),
        (Some(best_choices.clone()), max_expected_value.clone()),
    );
    (best_choices, max_expected_value)
}

pub fn best_choice_1_reroll<S1, S2, S3, V>(
    game: Game,
    expected_values: &papaya::HashMap<GameState, V, S1>,
    cache: &papaya::HashMap<(Game, Option<Choice>), (Option<HashSet<Choice, S2>>, V), S3>,
) -> (HashSet<Choice, S2>, V)
where
    S1: BuildHasher + Sync,
    S2: BuildHasher + Clone + Default + Send + Sync,
    S3: BuildHasher + Sync,
    V: Value
        + AddAssign
        + Clone
        + PartialOrd
        + Sum
        + for<'a> Sum<<&'a V as Mul<V>>::Output>
        + Send
        + Sync,
    for<'a> &'a V: Mul<V> + PartialEq<&'a V>,
    for<'a> <&'a V as Mul<V>>::Output: Send + Sync,
{
    assert!(game.rerolls_left() == 1);

    if let Some((Some(choices), value)) = cache.pin().get(&(game, None)) {
        return (choices.clone(), value.clone());
    }

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

    let mut best_choices = Vec::new();
    let mut max_expected_value = V::zero();

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => {
                let mut game = game.clone();
                game.set_combo_raw(combo, Some(combo.points(game.dice())));
                expected_score(game, expected_values)
            }
            Choice::Reroll1(dice) => V::roll_1_prob()
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values, cache);
                    prob * value
                })
                .sum(),
            Choice::Reroll2(dice) => V::roll_2_prob()
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values, cache);
                    prob * value
                })
                .sum(),
            Choice::Reroll3(dice) => V::roll_3_prob()
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values, cache);
                    prob * value
                })
                .sum(),
            Choice::Reroll4(dice) => V::roll_4_prob()
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values, cache);
                    prob * value
                })
                .sum(),
            Choice::Reroll5(dice) => V::roll_5_prob()
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(0);
                    let (_, value) = best_choice_0_rerolls(game, expected_values, cache);
                    prob * value
                })
                .sum(),
        };

        if value >= max_expected_value {
            best_choices.push((choice, value.clone()));
            max_expected_value = value;
        }
    }

    let best_choices: HashSet<Choice, S2> = best_choices
        .into_iter()
        .filter_map(|(choice, value)| (value == max_expected_value).then_some(choice))
        .collect();
    cache.pin().insert(
        (game, None),
        (Some(best_choices.clone()), max_expected_value.clone()),
    );
    (best_choices, max_expected_value)
}

pub fn best_choice_2_rerolls<S1, S2, S3, V>(
    game: Game,
    expected_values: &papaya::HashMap<GameState, V, S1>,
    cache: &papaya::HashMap<(Game, Option<Choice>), (Option<HashSet<Choice, S2>>, V), S3>,
) -> (HashSet<Choice, S2>, V)
where
    S1: BuildHasher + Sync,
    S2: BuildHasher + Default + Clone + Send + Sync,
    S3: BuildHasher + Sync,
    V: Value
        + AddAssign
        + Clone
        + PartialOrd
        + Send
        + Sum
        + for<'a> Sum<<&'a V as Mul<V>>::Output>
        + Sync,
    for<'a> &'a V: Mul<V> + PartialEq<&'a V>,
    for<'a> <&'a V as Mul<V>>::Output: Send + Sync,
{
    assert!(game.rerolls_left() == 2);

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

    let mut best_choices = Vec::new();
    let mut max_expected_value = V::zero();

    for choice in choices {
        let value = match choice {
            Choice::SelectCombo(combo) => {
                let mut game = game.clone();
                game.set_combo_raw(combo, Some(combo.points(game.dice())));
                expected_score(game, expected_values)
            }
            Choice::Reroll1(dice) => V::roll_1_prob()
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(1);
                    let (_, value) =
                        best_choice_1_reroll_non_parallel(game, expected_values, cache);
                    prob * value
                })
                .sum(),
            Choice::Reroll2(dice) => V::roll_2_prob()
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(1);
                    let (_, value) =
                        best_choice_1_reroll_non_parallel(game, expected_values, cache);
                    prob * value
                })
                .sum(),
            Choice::Reroll3(dice) => V::roll_3_prob()
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(1);
                    let (_, value) =
                        best_choice_1_reroll_non_parallel(game, expected_values, cache);
                    prob * value
                })
                .sum(),
            Choice::Reroll4(dice) => V::roll_4_prob()
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(1);
                    let (_, value) =
                        best_choice_1_reroll_non_parallel(game, expected_values, cache);
                    prob * value
                })
                .sum(),
            Choice::Reroll5(dice) => V::roll_5_prob()
                .par_iter()
                .map(|(new_dice, prob)| {
                    let mut game = game.clone();
                    game.replace_dice(&dice, new_dice).unwrap();
                    game.set_rerolls(1);
                    let (_, value) =
                        best_choice_1_reroll_non_parallel(game, expected_values, cache);
                    prob * value
                })
                .sum(),
        };

        if value >= max_expected_value {
            best_choices.push((choice, value.clone()));
            max_expected_value = value;
        }
    }

    let best_choices = best_choices
        .into_iter()
        .filter_map(|(choice, value)| (value == max_expected_value).then_some(choice))
        .collect();
    (best_choices, max_expected_value)
}
