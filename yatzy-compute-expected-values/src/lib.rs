use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use yatzy::{Combo, Dice, Die, Game};

pub mod float;
pub mod rational;

lazy_static! {
    static ref CACHE: papaya::HashMap<Game, f64> = papaya::HashMap::new();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum NumberState {
    Empty,
    Filled0,
    Filled1,
    Filled2,
    Filled3,
    Filled4,
    Filled5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum FieldState {
    Empty,
    Filled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GameState {
    pub numbers_total: u8,
    pub ones: FieldState,
    pub twos: FieldState,
    pub threes: FieldState,
    pub fours: FieldState,
    pub fives: FieldState,
    pub sixes: FieldState,
    pub one_pair: FieldState,
    pub two_pairs: FieldState,
    pub three_of_a_kind: FieldState,
    pub four_of_a_kind: FieldState,
    pub small_straight: FieldState,
    pub large_straight: FieldState,
    pub full_house: FieldState,
    pub chance: FieldState,
    pub yatzy: FieldState,
}

pub fn game_from_state(state: GameState, dice: Dice) -> Game {
    let mut numbers_filled = false;
    let ones = match state.ones {
        FieldState::Empty => None,
        FieldState::Filled => {
            if numbers_filled {
                Some(0)
            } else {
                numbers_filled = true;
                Some(state.numbers_total)
            }
        }
    };
    let twos = match state.twos {
        FieldState::Empty => None,
        FieldState::Filled => {
            if numbers_filled {
                Some(0)
            } else {
                numbers_filled = true;
                Some(state.numbers_total)
            }
        }
    };
    let threes = match state.threes {
        FieldState::Empty => None,
        FieldState::Filled => {
            if numbers_filled {
                Some(0)
            } else {
                numbers_filled = true;
                Some(state.numbers_total)
            }
        }
    };
    let fours = match state.fours {
        FieldState::Empty => None,
        FieldState::Filled => {
            if numbers_filled {
                Some(0)
            } else {
                numbers_filled = true;
                Some(state.numbers_total)
            }
        }
    };
    let fives = match state.fives {
        FieldState::Empty => None,
        FieldState::Filled => {
            if numbers_filled {
                Some(0)
            } else {
                numbers_filled = true;
                Some(state.numbers_total)
            }
        }
    };
    let sixes = match state.sixes {
        FieldState::Empty => None,
        FieldState::Filled => {
            if numbers_filled {
                Some(0)
            } else {
                Some(state.numbers_total)
            }
        }
    };

    Game::new_raw(
        dice,
        2,
        ones,
        twos,
        threes,
        fours,
        fives,
        sixes,
        match state.one_pair {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.two_pairs {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.three_of_a_kind {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.four_of_a_kind {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.small_straight {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.large_straight {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.full_house {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.chance {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.yatzy {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
    )
}

pub fn state_from_game(game: Game) -> GameState {
    let mut possible_remaining_numbers = 0;
    let mut numbers_total = 0;

    match game.combo(Combo::Ones) {
        None => {
            possible_remaining_numbers += 5;
        }
        Some(n) => {
            numbers_total += n;
        }
    }
    match game.combo(Combo::Twos) {
        None => {
            possible_remaining_numbers += 10;
        }
        Some(n) => {
            numbers_total += n;
        }
    }
    match game.combo(Combo::Threes) {
        None => {
            possible_remaining_numbers += 15;
        }
        Some(n) => {
            numbers_total += n;
        }
    }
    match game.combo(Combo::Fours) {
        None => {
            possible_remaining_numbers += 20;
        }
        Some(n) => {
            numbers_total += n;
        }
    }
    match game.combo(Combo::Fives) {
        None => {
            possible_remaining_numbers += 25;
        }
        Some(n) => {
            numbers_total += n;
        }
    }
    match game.combo(Combo::Sixes) {
        None => {
            possible_remaining_numbers += 30;
        }
        Some(n) => {
            numbers_total += n;
        }
    }

    if numbers_total > 63 {
        numbers_total = 63;
    }

    if numbers_total + possible_remaining_numbers < 63 {
        numbers_total = 0;
    }

    GameState {
        numbers_total,
        ones: match game.combo(Combo::Ones) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        twos: match game.combo(Combo::Twos) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        threes: match game.combo(Combo::Threes) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        fours: match game.combo(Combo::Fours) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        fives: match game.combo(Combo::Fives) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        sixes: match game.combo(Combo::Sixes) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        one_pair: match game.combo(Combo::OnePair) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        two_pairs: match game.combo(Combo::TwoPairs) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        three_of_a_kind: match game.combo(Combo::ThreeOfAKind) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        four_of_a_kind: match game.combo(Combo::FourOfAKind) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        small_straight: match game.combo(Combo::SmallStraight) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        large_straight: match game.combo(Combo::LargeStraight) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        full_house: match game.combo(Combo::FullHouse) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        chance: match game.combo(Combo::Chance) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        yatzy: match game.combo(Combo::Yatzy) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
    }
}

pub fn game_states_by_empty_field_count() -> HashMap<u8, HashSet<GameState>> {
    let number_states = [
        NumberState::Empty,
        NumberState::Filled0,
        NumberState::Filled1,
        NumberState::Filled2,
        NumberState::Filled3,
        NumberState::Filled4,
        NumberState::Filled5,
    ];
    let field_states = [FieldState::Empty, FieldState::Filled];

    let mut map = HashMap::with_capacity(15);
    for n in 1..=15 {
        map.insert(n, HashSet::new());
    }

    for (n1, n2, n3, n4, n5, n6) in itertools::iproduct!(
        number_states,
        number_states,
        number_states,
        number_states,
        number_states,
        number_states,
    ) {
        for (f1, f2, f3, f4, f5, f6, f7, f8, f9) in itertools::iproduct!(
            field_states,
            field_states,
            field_states,
            field_states,
            field_states,
            field_states,
            field_states,
            field_states,
            field_states,
        ) {
            let mut empty = 0;
            let mut numbers_total = 0;
            let mut possible_remaining_numbers = 0;

            for (n, state) in [(1, n1), (2, n2), (3, n3), (4, n4), (5, n5), (6, n6)] {
                match state {
                    NumberState::Empty => {
                        empty += 1;
                        possible_remaining_numbers += 5 * n;
                    }
                    NumberState::Filled0 => {}
                    NumberState::Filled1 => {
                        numbers_total += n;
                    }
                    NumberState::Filled2 => {
                        numbers_total += 2 * n;
                    }
                    NumberState::Filled3 => {
                        numbers_total += 3 * n;
                    }
                    NumberState::Filled4 => {
                        numbers_total += 4 * n;
                    }
                    NumberState::Filled5 => {
                        numbers_total += 5 * n;
                    }
                }
            }

            if numbers_total > 63 {
                numbers_total = 63;
            }

            // if the game cannot possible attain the bonus anymore, set numbers_total = 0
            if numbers_total + possible_remaining_numbers < 63 {
                numbers_total = 0;
            }

            for state in [f1, f2, f3, f4, f5, f6, f7, f8, f9] {
                if state == FieldState::Empty {
                    empty += 1;
                }
            }

            if empty == 0 {
                continue;
            }

            let state = GameState {
                numbers_total,
                ones: match n1 {
                    NumberState::Empty => FieldState::Empty,
                    _ => FieldState::Filled,
                },
                twos: match n2 {
                    NumberState::Empty => FieldState::Empty,
                    _ => FieldState::Filled,
                },
                threes: match n3 {
                    NumberState::Empty => FieldState::Empty,
                    _ => FieldState::Filled,
                },
                fours: match n4 {
                    NumberState::Empty => FieldState::Empty,
                    _ => FieldState::Filled,
                },
                fives: match n5 {
                    NumberState::Empty => FieldState::Empty,
                    _ => FieldState::Filled,
                },
                sixes: match n6 {
                    NumberState::Empty => FieldState::Empty,
                    _ => FieldState::Filled,
                },
                one_pair: f1,
                two_pairs: f2,
                three_of_a_kind: f3,
                four_of_a_kind: f4,
                small_straight: f5,
                large_straight: f6,
                full_house: f7,
                chance: f8,
                yatzy: f9,
            };

            map.get_mut(&empty).unwrap().insert(state);
        }
    }

    map
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Choice {
    SelectCombo(Combo),
    Reroll1([Die; 1]),
    Reroll2([Die; 2]),
    Reroll3([Die; 3]),
    Reroll4([Die; 4]),
    Reroll5([Die; 5]),
}
