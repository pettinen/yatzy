#![allow(dead_code)]

use std::io::Write as _;

use yatzy::{solver, Combo, Game};

fn main() {
    let mut rng = rand::rng();

    let mut game = Game::new(&mut rng);

    while !game.ended() {
        print_game(&game);

        match game.rerolls_left() {
            0 => {
                let combo = solver::best_choice_0_rerolls(game).0;
                println!("Selecting {combo:?}");
                game.select_combo(combo, &mut rng).unwrap();
            }
            1 => {
                match solver::best_choice_1_reroll(game).0 {
                    solver::Choice::Reroll1(d1) => {
                        println!("Rerolling {d1}");
                        game.reroll(&[d1], &mut rng).unwrap();
                    }
                    solver::Choice::Reroll2(d1, d2) => {
                        println!("Rerolling {d1} {d2}");
                        game.reroll(&[d1, d2], &mut rng).unwrap();
                    }
                    solver::Choice::Reroll3(d1, d2, d3) => {
                        println!("Rerolling {d1} {d2} {d3}");
                        game.reroll(&[d1, d2, d3], &mut rng).unwrap();
                    }
                    solver::Choice::Reroll4(d1, d2, d3, d4) => {
                        println!("Rerolling {d1} {d2} {d3} {d4}");
                        game.reroll(&[d1, d2, d3, d4], &mut rng).unwrap();
                    }
                    solver::Choice::Reroll5(d1, d2, d3, d4, d5) => {
                        println!("Rerolling all dice");
                        game.reroll(&[d1, d2, d3, d4, d5], &mut rng).unwrap();
                    }
                    solver::Choice::SelectCombo(combo) => {
                        println!("Selecting {combo:?}");
                        game.select_combo(combo, &mut rng).unwrap();
                    }
                }
            }
            2 => {
                match solver::best_choice_2_rerolls(game).0 {
                    solver::Choice::Reroll1(d1) => {
                        println!("Rerolling {d1}");
                        game.reroll(&[d1], &mut rng).unwrap();
                    }
                    solver::Choice::Reroll2(d1, d2) => {
                        println!("Rerolling {d1} {d2}");
                        game.reroll(&[d1, d2], &mut rng).unwrap();
                    }
                    solver::Choice::Reroll3(d1, d2, d3) => {
                        println!("Rerolling {d1} {d2} {d3}");
                        game.reroll(&[d1, d2, d3], &mut rng).unwrap();
                    }
                    solver::Choice::Reroll4(d1, d2, d3, d4) => {
                        println!("Rerolling {d1} {d2} {d3} {d4}");
                        game.reroll(&[d1, d2, d3, d4], &mut rng).unwrap();
                    }
                    solver::Choice::Reroll5(d1, d2, d3, d4, d5) => {
                        println!("Rerolling all dice");
                        game.reroll(&[d1, d2, d3, d4, d5], &mut rng).unwrap();
                    }
                    solver::Choice::SelectCombo(combo) => {
                        println!("Selecting {combo:?}");
                        game.select_combo(combo, &mut rng).unwrap();
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    print_game(&game);
}

fn play_game() {
    let mut rng = rand::rng();
    let mut stdin_lines = std::io::stdin().lines();
    let mut stdout = std::io::stdout();
    let mut game = Game::new(&mut rng);

    loop {
        print_game(&game);

        print!("Your action: ");
        if let Err(error) = stdout.flush() {
            println!("error: {error}");
            break;
        }

        let input = match stdin_lines.next() {
            Some(Ok(line)) => line,
            Some(Err(error)) => {
                println!("error: {error}");
                break;
            }
            None => {
                break;
            }
        };
        let input = input.trim().to_lowercase();
        print!("\n");

        let select_combo_result = if input == "ones" {
            Some(game.select_combo(Combo::Ones, &mut rng))
        } else if input == "twos" {
            Some(game.select_combo(Combo::Twos, &mut rng))
        } else if input == "threes" {
            Some(game.select_combo(Combo::Threes, &mut rng))
        } else if input == "fours" {
            Some(game.select_combo(Combo::Fours, &mut rng))
        } else if input == "fives" {
            Some(game.select_combo(Combo::Fives, &mut rng))
        } else if input == "sixes" {
            Some(game.select_combo(Combo::Sixes, &mut rng))
        } else if input == "one pair" {
            Some(game.select_combo(Combo::OnePair, &mut rng))
        } else if input == "two pairs" {
            Some(game.select_combo(Combo::TwoPairs, &mut rng))
        } else if input == "three of a kind" {
            Some(game.select_combo(Combo::ThreeOfAKind, &mut rng))
        } else if input == "four of a kind" {
            Some(game.select_combo(Combo::FourOfAKind, &mut rng))
        } else if input == "small straight" {
            Some(game.select_combo(Combo::SmallStraight, &mut rng))
        } else if input == "large straight" {
            Some(game.select_combo(Combo::LargeStraight, &mut rng))
        } else if input == "full house" {
            Some(game.select_combo(Combo::FullHouse, &mut rng))
        } else if input == "chance" {
            Some(game.select_combo(Combo::Chance, &mut rng))
        } else if input == "yatzy" {
            Some(game.select_combo(Combo::Yatzy, &mut rng))
        } else {
            None
        };

        let reroll_dice_result = if input.starts_with("reroll ") {
            let mut reroll_dice = Vec::with_capacity(5);
            let mut valid = true;
            for char in input.chars().skip(7) {
                if ('1'..='6').contains(&char) {
                    if reroll_dice.len() >= 5 {
                        valid = false;
                        break;
                    }
                    reroll_dice.push(char.to_digit(10).unwrap() as u8);
                } else if char.is_whitespace() {
                } else {
                    valid = false;
                    break;
                }
            }
            if valid && reroll_dice.len() > 0 {
                Some(game.reroll(&reroll_dice, &mut rng))
            } else {
                None
            }
        } else {
            None
        };

        if let Some(Err(error)) = select_combo_result {
            println!("error: {error}");
        }
        if let Some(Err(error)) = reroll_dice_result {
            println!("error: {error}");
        }
        if select_combo_result.is_none() && reroll_dice_result.is_none() {
            println!("error: invalid input");
        }

        if game.ended() {
            print_game(&game);
            break;
        }
    }
}

fn print_score(name: &'static str, score: Option<u8>) {
    match score {
        Some(score) => {
            print_score_str(name, &score.to_string());
        }
        None => {
            println!("{name}");
        }
    }
}

fn print_score_str(name: &'static str, score: &str) {
    let mut line = String::from(name);
    for _ in 0..20 - name.len() - score.len() {
        line.push(' ');
    }
    line.push_str(score);
    println!("{line}");
}

fn print_game(game: &Game) {
    let ones = game.combo(Combo::Ones);
    let twos = game.combo(Combo::Twos);
    let threes = game.combo(Combo::Threes);
    let fours = game.combo(Combo::Fours);
    let fives = game.combo(Combo::Fives);
    let sixes = game.combo(Combo::Sixes);

    print_score("Ones", ones);
    print_score("Twos", twos);
    print_score("Threes", threes);
    print_score("Fours", fours);
    print_score("Fives", fives);
    print_score("Sixes", sixes);

    let bonus = if ones.unwrap_or(0)
        + twos.unwrap_or(0)
        + threes.unwrap_or(0)
        + fours.unwrap_or(0)
        + fives.unwrap_or(0)
        + sixes.unwrap_or(0)
        >= 63
    {
        Some(50)
    } else {
        None
    };
    print_score("Bonus", bonus);

    print_score("One pair", game.combo(Combo::OnePair));
    print_score("Two pairs", game.combo(Combo::TwoPairs));
    print_score("Three of a kind", game.combo(Combo::ThreeOfAKind));
    print_score("Four of a kind", game.combo(Combo::FourOfAKind));
    print_score("Small straight", game.combo(Combo::SmallStraight));
    print_score("Large straight", game.combo(Combo::LargeStraight));
    print_score("Full house", game.combo(Combo::FullHouse));
    print_score("Chance", game.combo(Combo::Chance));
    print_score("Yatzy", game.combo(Combo::Yatzy));
    print_score_str("Total", &game.score().to_string());
    println!();
    print!("Dice:");
    for die in *game.dice() {
        print!(" {die}");
    }
    println!();
    let rerolls_left = game.rerolls_left();
    if rerolls_left == 1 {
        println!("1 reroll left");
    } else {
        println!("{rerolls_left} rerolls left");
    }
    println!();
}
