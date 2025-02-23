use std::{
    hash::{Hash, Hasher},
    ops::Deref,
};

use lazy_static::lazy_static;
use rand::{
    distr::{Distribution as _, Uniform},
    Rng,
};

pub mod solver;

pub type Die = u8;

lazy_static! {
    static ref DISTRIBUTION: Uniform<Die> = Uniform::new_inclusive(1, 6).unwrap();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dice {
    array: [Die; 5],
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum DiceReplaceError {
    #[error("selected dice are not in hand")]
    InvalidDice,
}

impl Dice {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        let mut array = [(); 5].map(|_| DISTRIBUTION.sample(rng));
        array.sort();
        Self { array }
    }

    pub fn replace(&mut self, old: &[Die], new: &[Die]) -> Result<(), DiceReplaceError> {
        assert!(old.len() == new.len(), "`old` and `new` must be of equal lengths");
        let mut new_dice = self.array.clone();
        for die in old {
            if let Some(index) = new_dice.iter().position(|x| x == die) {
                new_dice[index] = 0;
            } else {
                return Err(DiceReplaceError::InvalidDice);
            }
        }
        let mut i = 0;
        for die in &mut new_dice {
            if *die == 0 {
                *die = new[i];
                i += 1;
            }
        }
        new_dice.sort();
        self.array = new_dice;
        Ok(())
    }

    pub fn reroll<R: Rng>(&mut self, dice: &[Die], rng: &mut R) -> Result<(), DiceReplaceError> {
        let mut new_dice = self.array.clone();
        for die in dice {
            if let Some(index) = new_dice.iter().position(|x| x == die) {
                new_dice[index] = 0;
            } else {
                return Err(DiceReplaceError::InvalidDice);
            }
        }
        for die in &mut new_dice {
            if *die == 0 {
                *die = DISTRIBUTION.sample(rng);
            }
        }
        new_dice.sort();
        self.array = new_dice;
        Ok(())
    }

    pub fn reroll_all<R: Rng>(&mut self, rng: &mut R) {
        let mut array = [(); 5].map(|_| DISTRIBUTION.sample(rng));
        array.sort();
        self.array = array;
    }
}

impl Deref for Dice {
    type Target = [Die; 5];

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl Hash for Dice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.array.hash(state)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Combo {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FourOfAKind,
    SmallStraight,
    LargeStraight,
    FullHouse,
    Chance,
    Yatzy,
}

impl Combo {
    pub fn points(&self, dice: Dice) -> u8 {
        match self {
            Self::Ones => dice.iter().filter(|&&x| x == 1).count() as u8,
            Self::Twos => 2 * dice.iter().filter(|&&x| x == 2).count() as u8,
            Self::Threes => 3 * dice.iter().filter(|&&x| x == 3).count() as u8,
            Self::Fours => 4 * dice.iter().filter(|&&x| x == 4).count() as u8,
            Self::Fives => 5 * dice.iter().filter(|&&x| x == 5).count() as u8,
            Self::Sixes => 6 * dice.iter().filter(|&&x| x == 6).count() as u8,
            Self::OnePair => {
                if dice[3] == dice[4] {
                    2 * dice[3]
                } else if dice[2] == dice[3] {
                    2 * dice[2]
                } else if dice[1] == dice[2] {
                    2 * dice[1]
                } else if dice[0] == dice[1] {
                    2 * dice[0]
                } else {
                    0
                }
            }
            Self::TwoPairs => {
                if dice[0] == dice[1] && dice[1] != dice[2] && dice[2] == dice[3] {
                    2 * dice[0] + 2 * dice[2]
                } else if dice[0] == dice[1] && dice[1] != dice[3] && dice[3] == dice[4] {
                    2 * dice[0] + 2 * dice[3]
                } else if dice[1] == dice[2] && dice[2] != dice[3] && dice[3] == dice[4] {
                    2 * dice[1] + 2 * dice[3]
                } else {
                    0
                }
            }
            Self::ThreeOfAKind => {
                if dice[2] == dice[3] && dice[3] == dice[4] {
                    3 * dice[2]
                } else if dice[1] == dice[2] && dice[2] == dice[3] {
                    3 * dice[1]
                } else if dice[0] == dice[1] && dice[1] == dice[2] {
                    3 * dice[0]
                } else {
                    0
                }
            }
            Self::FourOfAKind => {
                if dice[0] == dice[1] && dice[1] == dice[2] && dice[2] == dice[3] {
                    4 * dice[0]
                } else if dice[1] == dice[2] && dice[2] == dice[3] && dice[3] == dice[4] {
                    4 * dice[1]
                } else {
                    0
                }
            }
            Self::SmallStraight => {
                if *dice == [1, 2, 3, 4, 5] {
                    15
                } else {
                    0
                }
            }
            Self::LargeStraight => {
                if *dice == [2, 3, 4, 5, 6] {
                    20
                } else {
                    0
                }
            }
            Self::FullHouse => {
                if dice[0] == dice[1]
                    && dice[1] == dice[2]
                    && dice[2] != dice[3]
                    && dice[3] == dice[4]
                {
                    3 * dice[0] + 2 * dice[3]
                } else if dice[0] == dice[1]
                    && dice[1] != dice[2]
                    && dice[2] == dice[3]
                    && dice[3] == dice[4]
                {
                    2 * dice[0] + 3 * dice[2]
                } else {
                    0
                }
            }
            Self::Chance => dice.iter().sum(),
            Self::Yatzy => {
                if dice[0] == dice[1]
                    && dice[1] == dice[2]
                    && dice[2] == dice[3]
                    && dice[3] == dice[4]
                {
                    50
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Game {
    dice: Dice,
    rerolls_left: u8,
    ones: Option<u8>,
    twos: Option<u8>,
    threes: Option<u8>,
    fours: Option<u8>,
    fives: Option<u8>,
    sixes: Option<u8>,
    one_pair: Option<u8>,
    two_pairs: Option<u8>,
    three_of_a_kind: Option<u8>,
    four_of_a_kind: Option<u8>,
    small_straight: Option<u8>,
    large_straight: Option<u8>,
    full_house: Option<u8>,
    chance: Option<u8>,
    yatzy: Option<u8>,
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum RerollError {
    #[error("game ended")]
    GameEnded,
    #[error("selected dice are not in hand")]
    InvalidDice,
    #[error("no rerolls left")]
    NoRerollsLeft,
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum SelectComboError {
    #[error("combo already filled")]
    ComboAlreadyFilled,
    #[error("game ended")]
    GameEnded,
}

impl Game {
    pub fn combo(&self, combo: Combo) -> Option<u8> {
        match combo {
            Combo::Ones => self.ones,
            Combo::Twos => self.twos,
            Combo::Threes => self.threes,
            Combo::Fours => self.fours,
            Combo::Fives => self.fives,
            Combo::Sixes => self.sixes,
            Combo::OnePair => self.one_pair,
            Combo::TwoPairs => self.two_pairs,
            Combo::ThreeOfAKind => self.three_of_a_kind,
            Combo::FourOfAKind => self.four_of_a_kind,
            Combo::SmallStraight => self.small_straight,
            Combo::LargeStraight => self.large_straight,
            Combo::FullHouse => self.full_house,
            Combo::Chance => self.chance,
            Combo::Yatzy => self.yatzy,
        }
    }

    pub fn dice(&self) -> Dice {
        self.dice
    }

    pub fn ended(&self) -> bool {
        self.ones.is_some()
            && self.twos.is_some()
            && self.threes.is_some()
            && self.fours.is_some()
            && self.fives.is_some()
            && self.sixes.is_some()
            && self.one_pair.is_some()
            && self.two_pairs.is_some()
            && self.three_of_a_kind.is_some()
            && self.four_of_a_kind.is_some()
            && self.small_straight.is_some()
            && self.large_straight.is_some()
            && self.full_house.is_some()
            && self.chance.is_some()
            && self.yatzy.is_some()
    }

    pub fn new<R: Rng>(rng: &mut R) -> Self {
        Self {
            dice: Dice::new(rng),
            rerolls_left: 2,
            ones: None,
            twos: None,
            threes: None,
            fours: None,
            fives: None,
            sixes: None,
            one_pair: None,
            two_pairs: None,
            three_of_a_kind: None,
            four_of_a_kind: None,
            small_straight: None,
            large_straight: None,
            full_house: None,
            chance: None,
            yatzy: None,
        }
    }

    pub fn reroll<R: Rng>(&mut self, dice: &[Die], rng: &mut R) -> Result<(), RerollError> {
        if self.ended() {
            return Err(RerollError::GameEnded);
        }
        if self.rerolls_left == 0 {
            return Err(RerollError::NoRerollsLeft);
        }
        match self.dice.reroll(dice, rng) {
            Ok(()) => {}
            Err(DiceReplaceError::InvalidDice) => {
                return Err(RerollError::InvalidDice);
            }
        }
        self.rerolls_left -= 1;
        Ok(())
    }

    pub fn rerolls_left(&self) -> u8 {
        self.rerolls_left
    }

    pub fn score(&self) -> u16 {
        let ones: u16 = self.ones.unwrap_or(0).into();
        let twos: u16 = self.twos.unwrap_or(0).into();
        let threes: u16 = self.threes.unwrap_or(0).into();
        let fours: u16 = self.fours.unwrap_or(0).into();
        let fives: u16 = self.fives.unwrap_or(0).into();
        let sixes: u16 = self.sixes.unwrap_or(0).into();
        let one_pair: u16 = self.one_pair.unwrap_or(0).into();
        let two_pairs: u16 = self.two_pairs.unwrap_or(0).into();
        let three_of_a_kind: u16 = self.three_of_a_kind.unwrap_or(0).into();
        let four_of_a_kind: u16 = self.four_of_a_kind.unwrap_or(0).into();
        let small_straight: u16 = self.small_straight.unwrap_or(0).into();
        let large_straight: u16 = self.large_straight.unwrap_or(0).into();
        let full_house: u16 = self.full_house.unwrap_or(0).into();
        let chance: u16 = self.chance.unwrap_or(0).into();
        let yatzy: u16 = self.yatzy.unwrap_or(0).into();

        let bonus = if ones + twos + threes + fours + fives + sixes >= 63 {
            50
        } else {
            0
        };

        ones
            + twos
            + threes
            + fours
            + fives
            + sixes
            + bonus
            + one_pair
            + two_pairs
            + three_of_a_kind
            + four_of_a_kind
            + small_straight
            + large_straight
            + full_house
            + chance
            + yatzy
    }

    pub fn select_combo<R: Rng>(&mut self, combo: Combo, rng: &mut R) -> Result<(), SelectComboError> {
        if self.ended() {
            return Err(SelectComboError::GameEnded);
        }
        let combo_ref = match combo {
            Combo::Ones => &mut self.ones,
            Combo::Twos => &mut self.twos,
            Combo::Threes => &mut self.threes,
            Combo::Fours => &mut self.fours,
            Combo::Fives => &mut self.fives,
            Combo::Sixes => &mut self.sixes,
            Combo::OnePair => &mut self.one_pair,
            Combo::TwoPairs => &mut self.two_pairs,
            Combo::ThreeOfAKind => &mut self.three_of_a_kind,
            Combo::FourOfAKind => &mut self.four_of_a_kind,
            Combo::SmallStraight => &mut self.small_straight,
            Combo::LargeStraight => &mut self.large_straight,
            Combo::FullHouse => &mut self.full_house,
            Combo::Chance => &mut self.chance,
            Combo::Yatzy => &mut self.yatzy,
        };
        if combo_ref.is_some() {
            return Err(SelectComboError::ComboAlreadyFilled);
        }
        combo_ref.replace(combo.points(self.dice));
        if !self.ended() {
            self.dice.reroll_all(rng);
            self.rerolls_left = 2;
        }
        Ok(())
    }
}
