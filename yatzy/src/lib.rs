use std::{
    hash::{Hash, Hasher},
    ops::Deref,
};

use lazy_static::lazy_static;
use rand::{
    Rng,
    distr::{Distribution as _, Uniform},
};

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

#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum NewDiceError {
    #[error("invalid die value")]
    InvalidDieValue,
}

impl Dice {
    pub fn new(mut dice: [Die; 5]) -> Result<Self, NewDiceError> {
        for die in dice {
            if die < 1 || die > 6 {
                return Err(NewDiceError::InvalidDieValue);
            }
        }
        dice.sort_unstable();
        Ok(Self { array: dice })
    }

    pub fn new_random<R: Rng>(rng: &mut R) -> Self {
        let mut array = [(); 5].map(|_| DISTRIBUTION.sample(rng));
        array.sort_unstable();
        Self { array }
    }

    pub fn new_raw(dice: [Die; 5]) -> Self {
        Self { array: dice }
    }

    pub fn replace(&mut self, old: &[Die], new: &[Die]) -> Result<(), DiceReplaceError> {
        assert!(
            old.len() == new.len(),
            "`old` and `new` must be of equal lengths"
        );
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
        new_dice.sort_unstable();
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
        new_dice.sort_unstable();
        self.array = new_dice;
        Ok(())
    }

    pub fn reroll_all<R: Rng>(&mut self, rng: &mut R) {
        let mut array = [(); 5].map(|_| DISTRIBUTION.sample(rng));
        array.sort_unstable();
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
    pub fn iter() -> ComboIterator {
        ComboIterator { current: None }
    }

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

#[derive(Copy, Clone, Debug)]
pub struct ComboIterator {
    current: Option<Combo>,
}

impl Iterator for ComboIterator {
    type Item = Combo;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.current {
            None => Some(Combo::Ones),
            Some(Combo::Ones) => Some(Combo::Twos),
            Some(Combo::Twos) => Some(Combo::Threes),
            Some(Combo::Threes) => Some(Combo::Fours),
            Some(Combo::Fours) => Some(Combo::Fives),
            Some(Combo::Fives) => Some(Combo::Sixes),
            Some(Combo::Sixes) => Some(Combo::OnePair),
            Some(Combo::OnePair) => Some(Combo::TwoPairs),
            Some(Combo::TwoPairs) => Some(Combo::ThreeOfAKind),
            Some(Combo::ThreeOfAKind) => Some(Combo::FourOfAKind),
            Some(Combo::FourOfAKind) => Some(Combo::SmallStraight),
            Some(Combo::SmallStraight) => Some(Combo::LargeStraight),
            Some(Combo::LargeStraight) => Some(Combo::FullHouse),
            Some(Combo::FullHouse) => Some(Combo::Chance),
            Some(Combo::Chance) => Some(Combo::Yatzy),
            Some(Combo::Yatzy) => None,
        };
        if next.is_some() {
            self.current = next;
        }
        next
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GameOptions {
    pub dice: [Die; 5],
    pub rerolls_left: u8,
    pub ones: Option<u8>,
    pub twos: Option<u8>,
    pub threes: Option<u8>,
    pub fours: Option<u8>,
    pub fives: Option<u8>,
    pub sixes: Option<u8>,
    pub one_pair: Option<u8>,
    pub two_pairs: Option<u8>,
    pub three_of_a_kind: Option<u8>,
    pub four_of_a_kind: Option<u8>,
    pub small_straight: Option<u8>,
    pub large_straight: Option<u8>,
    pub full_house: Option<u8>,
    pub chance: Option<u8>,
    pub yatzy: Option<u8>,
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
pub enum NewGameError {
    #[error("invalid value for combo {0:?}")]
    InvalidCombo(Combo),
    #[error("invalid dice")]
    InvalidDice(#[from] NewDiceError),
    #[error("invalid number of rerolls left")]
    InvalidRerollsLeft,
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
        self.round() == 15
    }

    pub fn has_bonus(&self) -> bool {
        self.ones.unwrap_or(0)
            + self.twos.unwrap_or(0)
            + self.threes.unwrap_or(0)
            + self.fours.unwrap_or(0)
            + self.fives.unwrap_or(0)
            + self.sixes.unwrap_or(0)
            >= 63
    }

    pub fn new(options: GameOptions) -> Result<Self, NewGameError> {
        if options.rerolls_left > 2 {
            return Err(NewGameError::InvalidRerollsLeft);
        }
        if let Some(ones) = options.ones {
            if ![0, 1, 2, 3, 4, 5].contains(&ones) {
                return Err(NewGameError::InvalidCombo(Combo::Ones));
            }
        }
        if let Some(twos) = options.twos {
            if ![0, 2, 4, 6, 8, 10].contains(&twos) {
                return Err(NewGameError::InvalidCombo(Combo::Twos));
            }
        }
        if let Some(threes) = options.threes {
            if ![0, 3, 6, 9, 12, 15].contains(&threes) {
                return Err(NewGameError::InvalidCombo(Combo::Threes));
            }
        }
        if let Some(fours) = options.fours {
            if ![0, 4, 8, 12, 16, 20].contains(&fours) {
                return Err(NewGameError::InvalidCombo(Combo::Fours));
            }
        }
        if let Some(fives) = options.fives {
            if ![0, 5, 10, 15, 20, 25].contains(&fives) {
                return Err(NewGameError::InvalidCombo(Combo::Fives));
            }
        }
        if let Some(sixes) = options.sixes {
            if ![0, 6, 12, 18, 24, 30].contains(&sixes) {
                return Err(NewGameError::InvalidCombo(Combo::Sixes));
            }
        }
        if let Some(one_pair) = options.one_pair {
            if ![0, 2, 4, 6, 8, 10, 12].contains(&one_pair) {
                return Err(NewGameError::InvalidCombo(Combo::OnePair));
            }
        }
        if let Some(two_pairs) = options.two_pairs {
            if ![0, 6, 8, 10, 12, 14, 16, 18, 20, 22].contains(&two_pairs) {
                return Err(NewGameError::InvalidCombo(Combo::TwoPairs));
            }
        }
        if let Some(three_of_a_kind) = options.three_of_a_kind {
            if ![0, 3, 6, 9, 12, 15, 18].contains(&three_of_a_kind) {
                return Err(NewGameError::InvalidCombo(Combo::ThreeOfAKind));
            }
        }
        if let Some(four_of_a_kind) = options.four_of_a_kind {
            if ![0, 4, 8, 12, 16, 20, 24].contains(&four_of_a_kind) {
                return Err(NewGameError::InvalidCombo(Combo::FourOfAKind));
            }
        }
        if let Some(small_straight) = options.small_straight {
            if ![0, 15].contains(&small_straight) {
                return Err(NewGameError::InvalidCombo(Combo::SmallStraight));
            }
        }
        if let Some(large_straight) = options.large_straight {
            if ![0, 20].contains(&large_straight) {
                return Err(NewGameError::InvalidCombo(Combo::LargeStraight));
            }
        }
        if let Some(full_house) = options.full_house {
            if full_house != 0
                && (!(7..=28).contains(&full_house) || full_house == 10 || full_house == 25)
            {
                return Err(NewGameError::InvalidCombo(Combo::FullHouse));
            }
        }
        if let Some(chance) = options.chance {
            if chance != 0 && !(5..=30).contains(&chance) {
                return Err(NewGameError::InvalidCombo(Combo::Chance));
            }
        }
        if let Some(yatzy) = options.yatzy {
            if ![0, 50].contains(&yatzy) {
                return Err(NewGameError::InvalidCombo(Combo::Yatzy));
            }
        }

        Ok(Self {
            dice: Dice::new(options.dice)?,
            rerolls_left: options.rerolls_left,
            ones: options.ones,
            twos: options.twos,
            threes: options.threes,
            fours: options.fours,
            fives: options.fives,
            sixes: options.sixes,
            one_pair: options.one_pair,
            two_pairs: options.two_pairs,
            three_of_a_kind: options.three_of_a_kind,
            four_of_a_kind: options.four_of_a_kind,
            small_straight: options.small_straight,
            large_straight: options.large_straight,
            full_house: options.full_house,
            chance: options.chance,
            yatzy: options.yatzy,
        })
    }

    pub fn new_random<R: Rng>(rng: &mut R) -> Self {
        Self {
            dice: Dice::new_random(rng),
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

    pub fn new_raw(
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
    ) -> Self {
        Self {
            dice,
            rerolls_left,
            ones,
            twos,
            threes,
            fours,
            fives,
            sixes,
            one_pair,
            two_pairs,
            three_of_a_kind,
            four_of_a_kind,
            small_straight,
            large_straight,
            full_house,
            chance,
            yatzy,
        }
    }

    pub fn replace_dice(&mut self, old: &[Die], new: &[Die]) -> Result<(), DiceReplaceError> {
        self.dice.replace(old, new)
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

    pub fn round(&self) -> u8 {
        Combo::iter()
            .map(|combo| self.combo(combo).is_some().then_some(1).unwrap_or(0))
            .sum()
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

        ones + twos
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

    pub fn select_combo<R: Rng>(
        &mut self,
        combo: Combo,
        rng: &mut R,
    ) -> Result<(), SelectComboError> {
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
        if self.ended() {
            self.rerolls_left = 0;
        } else {
            self.dice.reroll_all(rng);
            self.rerolls_left = 2;
        }
        Ok(())
    }

    pub fn set_combo_raw(&mut self, combo: Combo, points: Option<u8>) {
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
        match points {
            Some(n) => {
                combo_ref.replace(n);
            }
            None => {
                combo_ref.take();
            }
        }
    }

    pub fn set_rerolls(&mut self, rerolls_left: u8) {
        assert!(rerolls_left <= 2);
        self.rerolls_left = rerolls_left;
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

pub fn print_game(game: Game) {
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

    let upper_section_total = ones.unwrap_or(0)
        + twos.unwrap_or(0)
        + threes.unwrap_or(0)
        + fours.unwrap_or(0)
        + fives.unwrap_or(0)
        + sixes.unwrap_or(0);

    print_score("Upper section", Some(upper_section_total));

    let bonus = if upper_section_total >= 63 {
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
}
