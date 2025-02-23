use std::collections::HashMap;

use itertools::Itertools as _;
use num_rational::Ratio;
use rayon::prelude::*;

use crate::{Combo, Die, Game};

mod prob;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Choice {
    Reroll1(Die),
    Reroll2(Die, Die),
    Reroll3(Die, Die, Die),
    Reroll4(Die, Die, Die, Die),
    Reroll5(Die, Die, Die, Die, Die),
    SelectCombo(Combo),
}

pub fn best_choice_0_rerolls(game: Game) -> (Combo, u8) {
    let mut scores = [
        Combo::Ones,
        Combo::Twos,
        Combo::Threes,
        Combo::Fours,
        Combo::Fives,
        Combo::Sixes,
        Combo::OnePair,
        Combo::TwoPairs,
        Combo::ThreeOfAKind,
        Combo::FourOfAKind,
        Combo::SmallStraight,
        Combo::LargeStraight,
        Combo::FullHouse,
        Combo::Chance,
        Combo::Yatzy,
    ].into_par_iter()
        .filter_map(|combo| {
            if game.combo(combo).is_none() {
                let points = combo.points(game.dice);
                Some((combo, points))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    let max = *scores.values().max().unwrap();
    scores.retain(|_, points| *points == max);

    // heuristic: "best" combo to fill if points are the same
    for combo in [
        Combo::Yatzy,
        Combo::LargeStraight,
        Combo::SmallStraight,
        Combo::FullHouse,
        Combo::Sixes,
        Combo::Fives,
        Combo::Fours,
        Combo::Threes,
        Combo::Twos,
        Combo::Ones,
        Combo::FourOfAKind,
        Combo::ThreeOfAKind,
        Combo::TwoPairs,
        Combo::OnePair,
        Combo::Chance,
    ] {
        if scores.contains_key(&combo) {
            return (combo, max);
        }
    }
    unreachable!()
}

pub fn best_choice_1_reroll(game: Game) -> (Choice, Ratio<u64>) {
    let mut choices = HashMap::new();

    let (no_reroll_best_combo, no_reroll_points) = best_choice_0_rerolls(game);
    let no_reroll_points = Ratio::from_integer(no_reroll_points.into());
    choices.insert(Choice::SelectCombo(no_reroll_best_combo), no_reroll_points);

    for dice_to_replace in (*game.dice).iter().tuple_combinations() {
        let (&d1,) = dice_to_replace;
        if choices.contains_key(&Choice::Reroll1(d1)) {
            continue;
        }
        let expected_value = prob::ROLL_1_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1], &replacement_dice).unwrap();
            let (_, points) = best_choice_0_rerolls(game);
            prob * u64::from(points)
        })
        .sum();
        choices.insert(Choice::Reroll1(d1), expected_value);
    }

    for dice_to_replace in game.dice.iter().tuple_combinations() {
        let (&d1, &d2) = dice_to_replace;
        if choices.contains_key(&Choice::Reroll2(d1, d2)) {
            continue;
        }
        let expected_value = prob::ROLL_2_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2], &replacement_dice).unwrap();
            let (_, points) = best_choice_0_rerolls(game);
            prob * u64::from(points)
        }).sum();
        choices.insert(Choice::Reroll2(d1, d2), expected_value);
    }

    for dice_to_replace in game.dice.iter().tuple_combinations() {
        let (&d1, &d2, &d3) = dice_to_replace;
        if choices.contains_key(&Choice::Reroll3(d1, d2, d3)) {
            continue;
        }
        let expected_value = prob::ROLL_3_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2, d3], &replacement_dice).unwrap();
            let (_, points) = best_choice_0_rerolls(game);
            prob * u64::from(points)
        }).sum();
        choices.insert(Choice::Reroll3(d1, d2, d3), expected_value);
    }

    for dice_to_replace in game.dice.iter().tuple_combinations() {
        let (&d1, &d2, &d3, &d4) = dice_to_replace;
        if choices.contains_key(&Choice::Reroll4(d1, d2, d3, d4)) {
            continue;
        }
        let expected_value = prob::ROLL_4_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2, d3, d4], &replacement_dice).unwrap();
            let (_, points) = best_choice_0_rerolls(game);
            prob * u64::from(points)
        }).sum();
        choices.insert(Choice::Reroll4(d1, d2, d3, d4), expected_value);
    }

    for dice_to_replace in game.dice.into_iter().tuple_combinations() {
        let (d1, d2, d3, d4, d5) = dice_to_replace;
        let expected_value = prob::ROLL_5_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2, d3, d4, d5], &replacement_dice).unwrap();
            let (_, points) = best_choice_0_rerolls(game);
            prob * u64::from(points)
        }).sum();
        choices.insert(Choice::Reroll5(d1, d2, d3, d4, d5), expected_value);
    }

    let max = *choices.values().max().unwrap();
    choices.into_iter().filter(|(_, points)| *points == max).next().unwrap()
}

pub fn best_choice_2_rerolls(game: Game) -> (Choice, Ratio<u64>) {
    let mut choices = HashMap::new();

    let (no_reroll_best_combo, no_reroll_points) = best_choice_0_rerolls(game);
    let no_reroll_points = Ratio::from_integer(no_reroll_points.into());
    choices.insert(Choice::SelectCombo(no_reroll_best_combo), no_reroll_points);

    let choices1 = game.dice.iter().tuple_combinations().par_bridge().filter_map(|dice_to_replace| {
        let (&d1,) = dice_to_replace;
        if choices.contains_key(&Choice::Reroll1(d1)) {
            return None;
        }
        let expected_value = prob::ROLL_1_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1], &replacement_dice).unwrap();
            let (_, points) = best_choice_1_reroll(game);
            prob * points
        }).sum();
        Some((Choice::Reroll1(d1), expected_value))
    })
    .collect::<HashMap<_, _>>();

    let choices2 = game.dice.iter().tuple_combinations().par_bridge().filter_map(|dice_to_replace| {
        let (&d1, &d2) = dice_to_replace;
        if choices.contains_key(&Choice::Reroll2(d1, d2)) {
            return None;
        }
        let expected_value = prob::ROLL_2_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2], &replacement_dice).unwrap();
            let (_, points) = best_choice_1_reroll(game);
            prob * points
        }).sum();
        Some((Choice::Reroll2(d1, d2), expected_value))
    }).collect::<HashMap<_, _>>();

    let choices3 = game.dice.iter().tuple_combinations().par_bridge().filter_map(|dice_to_replace| {
        let (&d1, &d2, &d3) = dice_to_replace;
        if choices.contains_key(&Choice::Reroll3(d1, d2, d3)) {
            return None;
        }
        let expected_value: Ratio<u64> = prob::ROLL_3_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2, d3], &replacement_dice).unwrap();
            let (_, points) = best_choice_1_reroll(game);
            prob * points
        }).sum();
        Some((Choice::Reroll3(d1, d2, d3), expected_value))
    }).collect::<HashMap<_, _>>();

    let choices4 = game.dice.iter().tuple_combinations().par_bridge().filter_map(|dice_to_replace| {
        let (&d1, &d2, &d3, &d4) = dice_to_replace;
    if choices.contains_key(&Choice::Reroll4(d1, d2, d3, d4)) {
            return None;
        }
        let expected_value = prob::ROLL_4_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2, d3, d4], &replacement_dice).unwrap();
            let (_, points) = best_choice_1_reroll(game);
            prob * points
        }).sum();
        Some((Choice::Reroll4(d1, d2, d3, d4), expected_value))
    }).collect::<HashMap<_, _>>();

    let choices5 = game.dice.into_iter().tuple_combinations().par_bridge().map(|dice_to_replace| {
        let (d1, d2, d3, d4, d5) = dice_to_replace;
        let expected_value = prob::ROLL_5_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2, d3, d4, d5], &replacement_dice).unwrap();
            let (_, points) = best_choice_1_reroll(game);
            prob * points
        }).sum();
        (Choice::Reroll5(d1, d2, d3, d4, d5), expected_value)
    }).collect::<HashMap<_, _>>();

    choices.extend(choices1);
    choices.extend(choices2);
    choices.extend(choices3);
    choices.extend(choices4);
    choices.extend(choices5);
    let max = *choices.values().max().unwrap();
    choices.into_iter().filter(|(_, points)| *points == max).next().unwrap()
}
