use std::collections::HashSet;

use itertools::Itertools;
use num_rational::Ratio;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{solver::prob, Combo, Dice, Die, Game};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Choice {
    Reroll1(Die),
    Reroll2(Die, Die),
    Reroll3(Die, Die, Die),
    Reroll4(Die, Die, Die, Die),
    Reroll5(Die, Die, Die, Die, Die),
    SelectCombo(Combo),
}

fn one_reroll_best_choices(dice: Dice, combo: Combo) -> (Vec<Choice>, Ratio<u64>) {
    let mut choices = Vec::new();

    let select_combo_expected_value = Ratio::from_integer(combo.points(dice).into());
    choices.push((Choice::SelectCombo(combo), select_combo_expected_value));

    let mut max_expected_value = select_combo_expected_value;

    for dice_to_replace in dice.iter().tuple_combinations() {
        let (&d1,) = dice_to_replace;
        let value = prob::ROLL_1_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut dice = dice.clone();
            dice.replace(&[d1], &replacement_dice).unwrap();
            prob * u64::from(combo.points(dice))
        })
        .sum();
        if value >= max_expected_value {
            choices.push((Choice::Reroll1(d1), value));
            max_expected_value = value;
        }
    }

    for dice_to_replace in dice.iter().tuple_combinations() {
        let (&d1, &d2) = dice_to_replace;
        let value = prob::ROLL_2_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut dice = dice.clone();
            dice.replace(&[d1, d2], &replacement_dice).unwrap();
            prob * u64::from(combo.points(dice))
        })
        .sum();
        if value >= max_expected_value {
            choices.push((Choice::Reroll2(d1, d2), value));
            max_expected_value = value;
        }
    }

    for dice_to_replace in dice.iter().tuple_combinations() {
        let (&d1, &d2, &d3) = dice_to_replace;
        let value = prob::ROLL_3_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut dice = dice.clone();
            dice.replace(&[d1, d2, d3], &replacement_dice).unwrap();
            prob * u64::from(combo.points(dice))
        })
        .sum();
        if value >= max_expected_value {
            choices.push((Choice::Reroll3(d1, d2, d3), value));
            max_expected_value = value;
        }
    }

    for dice_to_replace in dice.iter().tuple_combinations() {
        let (&d1, &d2, &d3, &d4) = dice_to_replace;
        let value = prob::ROLL_4_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut dice = dice.clone();
            dice.replace(&[d1, d2, d3, d4], &replacement_dice).unwrap();
            prob * u64::from(combo.points(dice))
        })
        .sum();
        if value >= max_expected_value {
            choices.push((Choice::Reroll4(d1, d2, d3, d4), value));
            max_expected_value = value;
        }
    }

    for dice_to_replace in dice.iter().tuple_combinations() {
        let (&d1, &d2, &d3, &d4, &d5) = dice_to_replace;
        let value = prob::ROLL_5_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut dice = dice.clone();
            dice.replace(&[d1, d2, d3, d4, d5], &replacement_dice).unwrap();
            prob * u64::from(combo.points(dice))
        })
        .sum();
        if value >= max_expected_value {
            choices.push((Choice::Reroll5(d1, d2, d3, d4, d5), value));
            max_expected_value = value;
        }
    }

    (
        choices.into_iter().filter_map(|(choice, value)| (value == max_expected_value).then_some(choice)).collect(),
        max_expected_value,
    )
}

fn two_rerolls_best_choices(dice: Dice, combo: Combo) -> (Vec<Choice>, Ratio<u64>) {
    let mut choices = Vec::new();

    let select_combo_expected_value = Ratio::from_integer(combo.points(dice).into());
    choices.push((Choice::SelectCombo(combo), select_combo_expected_value));

    let mut max_expected_value = select_combo_expected_value;

    for dice_to_replace in dice.iter().tuple_combinations() {
        let (&d1,) = dice_to_replace;
        let value = prob::ROLL_1_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut dice = dice.clone();
            dice.replace(&[d1], &replacement_dice).unwrap();
            let (_, points) = one_reroll_best_choices(dice, combo);
            prob * points
        })
        .sum();
        if value >= max_expected_value {
            choices.push((Choice::Reroll1(d1), value));
            max_expected_value = value;
        }
    }

    for dice_to_replace in dice.iter().tuple_combinations() {
        let (&d1, &d2) = dice_to_replace;
        let value = prob::ROLL_2_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut dice = dice.clone();
            dice.replace(&[d1, d2], &replacement_dice).unwrap();
            let (_, points) = one_reroll_best_choices(dice, combo);
            prob * points
        })
        .sum();
        if value >= max_expected_value {
            choices.push((Choice::Reroll2(d1, d2), value));
            max_expected_value = value;
        }
    }

    for dice_to_replace in dice.iter().tuple_combinations() {
        let (&d1, &d2, &d3) = dice_to_replace;
        let value = prob::ROLL_3_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut dice = dice.clone();
            dice.replace(&[d1, d2, d3], &replacement_dice).unwrap();
            let (_, points) = one_reroll_best_choices(dice, combo);
            prob * points
        })
        .sum();
        if value >= max_expected_value {
            choices.push((Choice::Reroll3(d1, d2, d3), value));
            max_expected_value = value;
        }
    }

    for dice_to_replace in dice.iter().tuple_combinations() {
        let (&d1, &d2, &d3, &d4) = dice_to_replace;
        let value = prob::ROLL_4_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut dice = dice.clone();
            dice.replace(&[d1, d2, d3, d4], &replacement_dice).unwrap();
            let (_, points) = one_reroll_best_choices(dice, combo);
            prob * points
        })
        .sum();
        if value >= max_expected_value {
            choices.push((Choice::Reroll4(d1, d2, d3, d4), value));
            max_expected_value = value;
        }
    }

    for dice_to_replace in dice.iter().tuple_combinations() {
        let (&d1, &d2, &d3, &d4, &d5) = dice_to_replace;
        let value = prob::ROLL_5_PROB.into_par_iter().map(|(replacement_dice, prob)| {
            let mut dice = dice.clone();
            dice.replace(&[d1, d2, d3, d4, d5], &replacement_dice).unwrap();
            let (_, points) = one_reroll_best_choices(dice, combo);
            prob * points
        })
        .sum();
        if value >= max_expected_value {
            choices.push((Choice::Reroll5(d1, d2, d3, d4, d5), value));
            max_expected_value = value;
        }
    }

    (
        choices.into_iter().filter_map(|(choice, value)| (value == max_expected_value).then_some(choice)).collect(),
        max_expected_value,
    )
}

pub fn round1_2rerolls_optimal_choices_count(game: Game) -> usize {
    let mut set = HashSet::new();

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
        if game.combo(combo).is_none() {
            let (choices, _) = two_rerolls_best_choices(game.dice(), combo);
            set.extend(choices);
        }
    }

    set.len()
}

pub fn best_choice(game: Game) -> (Option<Choice>, Ratio<u64>) {
    if game.ended() {
        return (None, Ratio::from_integer(game.score().into()));
    }

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

    let mut best = None;
    let mut max_expected_value = Ratio::ZERO;

    match game.rerolls_left() {
        0 => {
            for combo in combos {
                if game.combo(combo).is_some() {
                    continue;
                }
                let mut game = game.clone(); // TODO needed?
                game.set_combo(combo);
                let value = if game.ended() {
                    Ratio::from_integer(game.score().into())
                } else {
                    game.set_rerolls(2);
                    prob::ROLL_5_PROB.into_par_iter().map(|(replacement_dice, prob)| {
                        let mut game = game.clone(); // TODO needed?
                        game.replace_dice(&*game.dice(), &replacement_dice).unwrap();
                        let (_, value) = best_choice(game);
                        prob * value
                    })
                    .sum()
                };

                if value > max_expected_value {
                    best = Some(Choice::SelectCombo(combo));
                    max_expected_value = value;
                }
            }
            assert!(best.is_some()); // likely panics, change above to
                                     // value >= max_expected_value?
        }
        1 => {
            let mut optimal_choices = HashSet::new();
            for combo in combos {
                if game.combo(combo).is_some() {
                    continue;
                }
                let (choices, _) = one_reroll_best_choices(game.dice(), combo);
                optimal_choices.extend(choices);
            }

            for choice in optimal_choices {
                let value = match choice {
                    Choice::SelectCombo(combo) => {
                        let mut game = game.clone(); // TODO needed?
                        game.set_combo(combo);
                        game.set_rerolls(2);
                        let (_, value) = best_choice(game);
                        value
                    }
                    Choice::Reroll1(d1) => {
                        prob::ROLL_1_PROB.into_par_iter().map(|(replacement_dice, prob)| {
                            let mut game = game.clone(); // TODO needed?
                            game.replace_dice(&[d1], &replacement_dice).unwrap();
                            game.set_rerolls(0);
                            let (_, value) = best_choice(game);
                            prob * value
                        })
                        .sum()
                    }
                    Choice::Reroll2(d1, d2) => {
                        prob::ROLL_2_PROB.into_par_iter().map(|(replacement_dice, prob)| {
                            let mut game = game.clone(); // TODO needed?
                            game.replace_dice(&[d1, d2], &replacement_dice).unwrap();
                            game.set_rerolls(0);
                            let (_, value) = best_choice(game);
                            prob * value
                        })
                        .sum()
                    }
                    Choice::Reroll3(d1, d2, d3) => {
                        prob::ROLL_3_PROB.into_par_iter().map(|(replacement_dice, prob)| {
                            let mut game = game.clone(); // TODO needed?
                            game.replace_dice(&[d1, d2, d3], &replacement_dice).unwrap();
                            game.set_rerolls(0);
                            let (_, value) = best_choice(game);
                            prob * value
                        })
                        .sum()
                    }
                    Choice::Reroll4(d1, d2, d3, d4) => {
                        prob::ROLL_4_PROB.into_par_iter().map(|(replacement_dice, prob)| {
                            let mut game = game.clone(); // TODO needed?
                            game.replace_dice(&[d1, d2, d3, d4], &replacement_dice).unwrap();
                            game.set_rerolls(0);
                            let (_, value) = best_choice(game);
                            prob * value
                        })
                        .sum()
                    }
                    Choice::Reroll5(d1, d2, d3, d4, d5) => {
                        prob::ROLL_5_PROB.into_par_iter().map(|(replacement_dice, prob)| {
                            let mut game = game.clone(); // TODO needed?
                            game.replace_dice(&[d1, d2, d3, d4, d5], &replacement_dice).unwrap();
                            game.set_rerolls(0);
                            let (_, value) = best_choice(game);
                            prob * value
                        })
                        .sum()
                    }
                };

                if value > max_expected_value {
                    best = Some(choice);
                    max_expected_value = value;
                }
            }

            assert!(best.is_some());
        }
        2 => {
            let mut optimal_choices = HashSet::new();
            for combo in combos {
                if game.combo(combo).is_some() {
                    continue;
                }
                let (choices, _) = two_rerolls_best_choices(game.dice(), combo);
                //println!("{:?} has {} optimal choice(s) ({:?})", combo, choices.len(), choices[0]);
                optimal_choices.extend(choices);
            }
            //println!("total {} optimal choices", optimal_choices.len());
            for choice in optimal_choices {
                let value = match choice {
                    Choice::SelectCombo(combo) => {
                        let mut game = game.clone(); // TODO needed?
                        game.set_combo(combo);
                        if game.ended() {
                            game.set_rerolls(0);
                        } else {
                            game.set_rerolls(2);
                        }
                        let (_, value) = best_choice(game);
                        value
                    }
                    Choice::Reroll1(d1) => {
                        prob::ROLL_1_PROB.into_par_iter().map(|(replacement_dice, prob)| {
                            let mut game = game.clone(); // TODO needed?
                            game.replace_dice(&[d1], &replacement_dice).unwrap();
                            game.set_rerolls(1);
                            let (_, value) = best_choice(game);
                            prob * value
                        })
                        .sum()
                    }
                    Choice::Reroll2(d1, d2) => {
                        prob::ROLL_2_PROB.into_par_iter().map(|(replacement_dice, prob)| {
                            let mut game = game.clone(); // TODO needed?
                            game.replace_dice(&[d1, d2], &replacement_dice).unwrap();
                            game.set_rerolls(1);
                            let (_, value) = best_choice(game);
                            prob * value
                        })
                        .sum()
                    }
                    Choice::Reroll3(d1, d2, d3) => {
                        prob::ROLL_3_PROB.into_par_iter().map(|(replacement_dice, prob)| {
                            let mut game = game.clone(); // TODO needed?
                            game.replace_dice(&[d1, d2, d3], &replacement_dice).unwrap();
                            game.set_rerolls(1);
                            let (_, value) = best_choice(game);
                            prob * value
                        })
                        .sum()
                    }
                    Choice::Reroll4(d1, d2, d3, d4) => {
                        prob::ROLL_4_PROB.into_par_iter().map(|(replacement_dice, prob)| {
                            let mut game = game.clone(); // TODO needed?
                            game.replace_dice(&[d1, d2, d3, d4], &replacement_dice).unwrap();
                            game.set_rerolls(1);
                            let (_, value) = best_choice(game);
                            prob * value
                        })
                        .sum()
                    }
                    Choice::Reroll5(d1, d2, d3, d4, d5) => {
                        prob::ROLL_5_PROB.into_par_iter().map(|(replacement_dice, prob)| {
                            let mut game = game.clone(); // TODO needed?
                            game.replace_dice(&[d1, d2, d3, d4, d5], &replacement_dice).unwrap();
                            game.set_rerolls(1);
                            let (_, value) = best_choice(game);
                            prob * value
                        })
                        .sum()
                    }
                };

                if value > max_expected_value {
                    best = Some(choice);
                    max_expected_value = value;
                }
            }

            assert!(best.is_some());
        }
        _ => unreachable!(),
    }

    (best, max_expected_value)
}
