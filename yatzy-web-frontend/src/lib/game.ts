import type { Dice } from "$lib/types";

export const points = {
    ones(dice: Dice): number {
        return dice.filter((die) => die === 1).length;
    },
    twos(dice: Dice): number {
        return 2 * dice.filter((die) => die === 2).length;
    },
    threes(dice: Dice): number {
        return 3 * dice.filter((die) => die === 3).length;
    },
    fours(dice: Dice): number {
        return 4 * dice.filter((die) => die === 4).length;
    },
    fives(dice: Dice): number {
        return 5 * dice.filter((die) => die === 5).length;
    },
    sixes(dice: Dice): number {
        return 6 * dice.filter((die) => die === 6).length;
    },
    one_pair(dice: Dice): number {
        for (const n of [6, 5, 4, 3, 2, 1]) {
            if (dice.filter((die) => die === n).length >= 2) {
                return 2 * n;
            }
        }
        return 0;
    },
    two_pairs(dice: Dice): number {
        let pair1: number | null = null;
        let pair2: number | null = null;
        for (const n of [6, 5, 4, 3, 2, 1]) {
            if (dice.filter((die) => die === n).length >= 2) {
                if (pair1 === null) {
                    pair1 = n;
                } else {
                    pair2 = n;
                    break;
                }
            }
        }
        if (pair1 !== null && pair2 !== null) {
            return 2 * pair1 + 2 * pair2;
        }
        return 0;
    },
    three_of_a_kind(dice: Dice): number {
        for (const n of [6, 5, 4, 3, 2, 1]) {
            if (dice.filter((die) => die === n).length >= 3) {
                return 3 * n;
            }
        }
        return 0;
    },
    four_of_a_kind(dice: Dice): number {
        for (const n of [6, 5, 4, 3, 2, 1]) {
            if (dice.filter((die) => die === n).length >= 4) {
                return 4 * n;
            }
        }
        return 0;
    },
    small_straight(dice: Dice): number {
        const sorted = dice.toSorted();
        if (
            sorted[0] === 1 &&
            sorted[1] === 2 &&
            sorted[2] === 3 &&
            sorted[3] === 4 &&
            sorted[4] === 5
        ) {
            return 15;
        }
        return 0;
    },
    large_straight(dice: Dice): number {
        const sorted = dice.toSorted();
        if (
            sorted[0] === 2 &&
            sorted[1] === 3 &&
            sorted[2] === 4 &&
            sorted[3] === 5 &&
            sorted[4] === 6
        ) {
            return 20;
        }
        return 0;
    },
    full_house(dice: Dice): number {
        const sorted = dice.toSorted();
        if (
            sorted[0] === sorted[1] &&
            sorted[1] === sorted[2] &&
            sorted[2] !== sorted[3] &&
            sorted[3] === sorted[4]
        ) {
            return 3 * sorted[0] + 2 * sorted[3];
        }
        if (
            sorted[0] === sorted[1] &&
            sorted[1] !== sorted[2] &&
            sorted[2] === sorted[3] &&
            sorted[3] === sorted[4]
        ) {
            return 2 * sorted[0] + 3 * sorted[2];
        }
        return 0;
    },
    chance(dice: Dice): number {
        return dice[0] + dice[1] + dice[2] + dice[3] + dice[4];
    },
    yatzy(dice: Dice): number {
        if (dice.filter((die) => die === dice[0]).length === 5) {
            return 50;
        }
        return 0;
    },
};

export const possible_scores = {
    ones: [0, 1, 2, 3, 4, 5],
    twos: [0, 2, 4, 6, 8, 10],
    threes: [0, 3, 6, 9, 12, 15],
    fours: [0, 4, 8, 12, 16, 20],
    fives: [0, 5, 10, 15, 20, 25],
    sixes: [0, 6, 12, 18, 24, 30],
    one_pair: [0, 2, 4, 6, 8, 10, 12],
    two_pairs: [0, 6, 8, 10, 12, 14, 16, 18, 20, 22],
    three_of_a_kind: [0, 3, 6, 9, 12, 15, 18],
    four_of_a_kind: [0, 4, 8, 12, 16, 20, 24],
    small_straight: [0, 15],
    large_straight: [0, 20],
    full_house: [
        0, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 26,
        27, 28,
    ],
    chance: [
        0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
        23, 24, 25, 26, 27, 28, 29, 30,
    ],
    yatzy: [0, 50],
};

export const validators = {
    dice(value: string): boolean {
        return /^[1-6]{5}$/.test(value);
    },
    rerolls_left(value: string): boolean {
        return ["0", "1", "2"].includes(value);
    },
    ones(value: string): boolean {
        return value === "" || possible_scores.ones.map(String).includes(value);
    },
    twos(value: string): boolean {
        return value === "" || possible_scores.twos.map(String).includes(value);
    },
    threes(value: string): boolean {
        return (
            value === "" || possible_scores.threes.map(String).includes(value)
        );
    },
    fours(value: string): boolean {
        return (
            value === "" || possible_scores.fours.map(String).includes(value)
        );
    },
    fives(value: string): boolean {
        return (
            value === "" || possible_scores.fives.map(String).includes(value)
        );
    },
    sixes(value: string): boolean {
        return (
            value === "" || possible_scores.sixes.map(String).includes(value)
        );
    },
    one_pair(value: string): boolean {
        return (
            value === "" || possible_scores.one_pair.map(String).includes(value)
        );
    },
    two_pairs(value: string): boolean {
        return (
            value === "" ||
            possible_scores.two_pairs.map(String).includes(value)
        );
    },
    three_of_a_kind(value: string): boolean {
        return (
            value === "" ||
            possible_scores.three_of_a_kind.map(String).includes(value)
        );
    },
    four_of_a_kind(value: string): boolean {
        return (
            value === "" ||
            possible_scores.four_of_a_kind.map(String).includes(value)
        );
    },
    small_straight(value: string): boolean {
        return (
            value === "" ||
            possible_scores.small_straight.map(String).includes(value)
        );
    },
    large_straight(value: string): boolean {
        return (
            value === "" ||
            possible_scores.large_straight.map(String).includes(value)
        );
    },
    full_house(value: string): boolean {
        return (
            value === "" ||
            possible_scores.full_house.map(String).includes(value)
        );
    },
    chance(value: string): boolean {
        return (
            value === "" || possible_scores.chance.map(String).includes(value)
        );
    },
    yatzy(value: string): boolean {
        return (
            value === "" || possible_scores.yatzy.map(String).includes(value)
        );
    },
};
