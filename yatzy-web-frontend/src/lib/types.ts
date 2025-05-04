export type Die = 1 | 2 | 3 | 4 | 5 | 6;
export type Dice = [Die, Die, Die, Die, Die];
export type Combo =
    | "ones"
    | "twos"
    | "threes"
    | "fours"
    | "fives"
    | "sixes"
    | "one_pair"
    | "two_pairs"
    | "three_of_a_kind"
    | "four_of_a_kind"
    | "small_straight"
    | "large_straight"
    | "full_house"
    | "chance"
    | "yatzy";
export type Reroll =
    | [Die]
    | [Die, Die]
    | [Die, Die, Die]
    | [Die, Die, Die, Die]
    | [Die, Die, Die, Die, Die];
export type Choice = Combo | Reroll;
export type Score = number | null;
export type RerollsLeft = 0 | 1 | 2;
