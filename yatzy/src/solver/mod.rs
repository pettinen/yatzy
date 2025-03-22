use std::collections::HashMap;

use itertools::Itertools as _;
use num_rational::Ratio;
use rayon::prelude::*;

use crate::{Combo, Die, Game};

mod prob;
pub mod v2;

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
    let mut max = 0;
    let mut best_combo = None;

    let combos = [
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
    ];
    for combo in combos {
        if game.combo(combo).is_none() {
            let points = combo.points(game.dice);
            if points > max {
                best_combo = Some(combo);
                max = points;
            }
        }
    }

    if max == 0 {
        for combo in combos {
            if game.combo(combo).is_none() {
                best_combo = Some(combo);
                break;
            }
        }
    }

    (best_combo.unwrap(), max)
}

pub fn best_choice_1_reroll(game: Game) -> (Choice, Ratio<u64>) {
    let mut choices = HashMap::new();

    let (no_reroll_best_combo, no_reroll_points) = best_choice_0_rerolls(game);
    let no_reroll_points = Ratio::from_integer(no_reroll_points.into());
    choices.insert(Choice::SelectCombo(no_reroll_best_combo), no_reroll_points);

    let choices1 = game.dice.iter().tuple_combinations().map(|dice_to_replace| {
        let (&d1,) = dice_to_replace;
        /*if choices.contains_key(&Choice::Reroll1(d1)) {
            return None;
        }*/
        let expected_value = prob::ROLL_1_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1], &replacement_dice).unwrap();
            let (_, points) = best_choice_0_rerolls(game);
            prob * u64::from(points)
        })
        .sum();
        (Choice::Reroll1(d1), expected_value)
    });

    let choices2 = game.dice.iter().tuple_combinations().map(|dice_to_replace| {
        let (&d1, &d2) = dice_to_replace;
        /*if choices.contains_key(&Choice::Reroll2(d1, d2)) {
            return None;
        }*/
        let expected_value = prob::ROLL_2_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2], &replacement_dice).unwrap();
            let (_, points) = best_choice_0_rerolls(game);
            prob * u64::from(points)
        }).sum();
        (Choice::Reroll2(d1, d2), expected_value)
    });

    let choices3 = game.dice.iter().tuple_combinations().map(|dice_to_replace| {
        let (&d1, &d2, &d3) = dice_to_replace;
        /*if choices.contains_key(&Choice::Reroll3(d1, d2, d3)) {
            return None;
        }*/
        let expected_value = prob::ROLL_3_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2, d3], &replacement_dice).unwrap();
            let (_, points) = best_choice_0_rerolls(game);
            prob * u64::from(points)
        }).sum();
        (Choice::Reroll3(d1, d2, d3), expected_value)
    });

    let choices4 = game.dice.iter().tuple_combinations().map(|dice_to_replace| {
        let (&d1, &d2, &d3, &d4) = dice_to_replace;
        /*if choices.contains_key(&Choice::Reroll4(d1, d2, d3, d4)) {
            return None;
        }*/
        let expected_value = prob::ROLL_4_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2, d3, d4], &replacement_dice).unwrap();
            let (_, points) = best_choice_0_rerolls(game);
            prob * u64::from(points)
        }).sum();
        (Choice::Reroll4(d1, d2, d3, d4), expected_value)
    });

    let choices5 = game.dice.into_iter().tuple_combinations().map(|dice_to_replace| {
        let (d1, d2, d3, d4, d5) = dice_to_replace;
        let expected_value = prob::ROLL_5_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2, d3, d4, d5], &replacement_dice).unwrap();
            let (_, points) = best_choice_0_rerolls(game);
            prob * u64::from(points)
        }).sum();
        (Choice::Reroll5(d1, d2, d3, d4, d5), expected_value)
    });

    choices.extend(choices1);
    choices.extend(choices2);
    choices.extend(choices3);
    choices.extend(choices4);
    choices.extend(choices5);
    let max = *choices.values().max().unwrap();
    choices.into_iter().filter(|(_, points)| *points == max).next().unwrap()
}

pub fn best_choice_2_rerolls(game: Game) -> (Choice, Ratio<u64>) {
    let mut choices = HashMap::new();

    let (no_reroll_best_combo, no_reroll_points) = best_choice_0_rerolls(game);
    let no_reroll_points = Ratio::from_integer(no_reroll_points.into());
    choices.insert(Choice::SelectCombo(no_reroll_best_combo), no_reroll_points);

    let choices1 = game.dice.iter().tuple_combinations().map(|dice_to_replace| {
        let (&d1,) = dice_to_replace;
        /*if choices.contains_key(&Choice::Reroll1(d1)) {
            return None;
        }*/
        let expected_value = prob::ROLL_1_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1], &replacement_dice).unwrap();
            let (_, points) = best_choice_1_reroll(game);
            prob * points
        }).sum();
        (Choice::Reroll1(d1), expected_value)
    });

    let choices2 = game.dice.iter().tuple_combinations().map(|dice_to_replace| {
        let (&d1, &d2) = dice_to_replace;
        /*if choices.contains_key(&Choice::Reroll2(d1, d2)) {
            return None;
        }*/
        let expected_value = prob::ROLL_2_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2], &replacement_dice).unwrap();
            let (_, points) = best_choice_1_reroll(game);
            prob * points
        }).sum();
        (Choice::Reroll2(d1, d2), expected_value)
    });

    let choices3 = game.dice.iter().tuple_combinations().map(|dice_to_replace| {
        let (&d1, &d2, &d3) = dice_to_replace;
        /*if choices.contains_key(&Choice::Reroll3(d1, d2, d3)) {
            return None;
        }*/
        let expected_value: Ratio<u64> = prob::ROLL_3_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2, d3], &replacement_dice).unwrap();
            let (_, points) = best_choice_1_reroll(game);
            prob * points
        }).sum();
        (Choice::Reroll3(d1, d2, d3), expected_value)
    });

    let choices4 = game.dice.iter().tuple_combinations().map(|dice_to_replace| {
        let (&d1, &d2, &d3, &d4) = dice_to_replace;
        /*if choices.contains_key(&Choice::Reroll4(d1, d2, d3, d4)) {
            return None;
        }*/
        let expected_value = prob::ROLL_4_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2, d3, d4], &replacement_dice).unwrap();
            let (_, points) = best_choice_1_reroll(game);
            prob * points
        }).sum();
        (Choice::Reroll4(d1, d2, d3, d4), expected_value)
    });

    let choices5 = game.dice.into_iter().tuple_combinations().map(|dice_to_replace| {
        let (d1, d2, d3, d4, d5) = dice_to_replace;
        let expected_value = prob::ROLL_5_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut game = game.clone();
            game.dice.replace(&[d1, d2, d3, d4, d5], &replacement_dice).unwrap();
            let (_, points) = best_choice_1_reroll(game);
            prob * points
        }).sum();
        (Choice::Reroll5(d1, d2, d3, d4, d5), expected_value)
    });

    choices.extend(choices1);
    choices.extend(choices2);
    choices.extend(choices3);
    choices.extend(choices4);
    choices.extend(choices5);
    let max = *choices.values().max().unwrap();
    choices.into_iter().filter(|(_, points)| *points == max).next().unwrap()
}
