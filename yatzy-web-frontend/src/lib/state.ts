import { possible_scores } from "$lib/game";
import type { Dice, RerollsLeft, Score } from "$lib/types";

export interface State {
    dice: Dice;
    rerolls_left: RerollsLeft;
    ones: Score;
    twos: Score;
    threes: Score;
    fours: Score;
    fives: Score;
    sixes: Score;
    one_pair: Score;
    two_pairs: Score;
    three_of_a_kind: Score;
    four_of_a_kind: Score;
    small_straight: Score;
    large_straight: Score;
    full_house: Score;
    chance: Score;
    yatzy: Score;
}

const possible_values = {
    ones: [null, ...possible_scores.ones],
    twos: [null, ...possible_scores.twos],
    threes: [null, ...possible_scores.threes],
    fours: [null, ...possible_scores.fours],
    fives: [null, ...possible_scores.fives],
    sixes: [null, ...possible_scores.sixes],
    one_pair: [null, ...possible_scores.one_pair],
    two_pairs: [null, ...possible_scores.two_pairs],
    three_of_a_kind: [null, ...possible_scores.three_of_a_kind],
    four_of_a_kind: [null, ...possible_scores.four_of_a_kind],
    small_straight: [null, ...possible_scores.small_straight],
    large_straight: [null, ...possible_scores.large_straight],
    full_house: [null, ...possible_scores.full_house],
    chance: [null, ...possible_scores.chance],
    yatzy: [null, ...possible_scores.yatzy],
};

const bit_length = (value: number): BigInt =>
    BigInt(Math.ceil(Math.log2(value)));
const bit_lengths = {
    die: bit_length(7),
    rerolls_left: bit_length(3),
    ones: bit_length(possible_values.ones.length),
    twos: bit_length(possible_values.twos.length),
    threes: bit_length(possible_values.threes.length),
    fours: bit_length(possible_values.fours.length),
    fives: bit_length(possible_values.fives.length),
    sixes: bit_length(possible_values.sixes.length),
    one_pair: bit_length(possible_values.one_pair.length),
    two_pairs: bit_length(possible_values.two_pairs.length),
    three_of_a_kind: bit_length(possible_values.three_of_a_kind.length),
    four_of_a_kind: bit_length(possible_values.four_of_a_kind.length),
    small_straight: bit_length(possible_values.small_straight.length),
    large_straight: bit_length(possible_values.large_straight.length),
    full_house: bit_length(possible_values.full_house.length),
    chance: bit_length(possible_values.chance.length),
    yatzy: bit_length(possible_values.yatzy.length),
};

export const from_state = (state: State): BigInt | null => {
    let value = 0n;

    for (const die of state.dice) {
        value <<= bit_lengths.die;
        value |= BigInt(die);
    }

    value <<= bit_lengths.rerolls_left;
    value |= BigInt(state.rerolls_left);

    const ones_index = possible_values.ones.indexOf(state.ones);
    if (ones_index === -1) {
        return null;
    }
    value <<= bit_lengths.ones;
    value |= BigInt(ones_index);

    const twos_index = possible_values.twos.indexOf(state.twos);
    if (twos_index === -1) {
        return null;
    }
    value <<= bit_lengths.twos;
    value |= BigInt(twos_index);

    const threes_index = possible_values.threes.indexOf(state.threes);
    if (threes_index === -1) {
        return null;
    }
    value <<= bit_lengths.threes;
    value |= BigInt(threes_index);

    const fours_index = possible_values.fours.indexOf(state.fours);
    if (fours_index === -1) {
        return null;
    }
    value <<= bit_lengths.fours;
    value |= BigInt(fours_index);

    const fives_index = possible_values.fives.indexOf(state.fives);
    if (fives_index === -1) {
        return null;
    }
    value <<= bit_lengths.fives;
    value |= BigInt(fives_index);

    const sixes_index = possible_values.sixes.indexOf(state.sixes);
    if (sixes_index === -1) {
        return null;
    }
    value <<= bit_lengths.sixes;
    value |= BigInt(sixes_index);

    const one_pair_index = possible_values.one_pair.indexOf(state.one_pair);
    if (one_pair_index === -1) {
        return null;
    }
    value <<= bit_lengths.one_pair;
    value |= BigInt(one_pair_index);

    const two_pairs_index = possible_values.two_pairs.indexOf(state.two_pairs);
    if (two_pairs_index === -1) {
        return null;
    }
    value <<= bit_lengths.two_pairs;
    value |= BigInt(two_pairs_index);

    const three_of_a_kind_index = possible_values.three_of_a_kind.indexOf(
        state.three_of_a_kind,
    );
    if (three_of_a_kind_index === -1) {
        return null;
    }
    value <<= bit_lengths.three_of_a_kind;
    value |= BigInt(three_of_a_kind_index);

    const four_of_a_kind_index = possible_values.four_of_a_kind.indexOf(
        state.four_of_a_kind,
    );
    if (four_of_a_kind_index === -1) {
        return null;
    }
    value <<= bit_lengths.four_of_a_kind;
    value |= BigInt(four_of_a_kind_index);

    const small_straight_index = possible_values.small_straight.indexOf(
        state.small_straight,
    );
    if (small_straight_index === -1) {
        return null;
    }
    value <<= bit_lengths.small_straight;
    value |= BigInt(small_straight_index);

    const large_straight_index = possible_values.large_straight.indexOf(
        state.large_straight,
    );
    if (large_straight_index === -1) {
        return null;
    }
    value <<= bit_lengths.large_straight;
    value |= BigInt(large_straight_index);

    const full_house_index = possible_values.full_house.indexOf(
        state.full_house,
    );
    if (full_house_index === -1) {
        return null;
    }
    value <<= bit_lengths.full_house;
    value |= BigInt(full_house_index);

    const chance_index = possible_values.chance.indexOf(state.chance);
    if (chance_index === -1) {
        return null;
    }
    value <<= bit_lengths.chance;
    value |= BigInt(chance_index);

    const yatzy_index = possible_values.yatzy.indexOf(state.yatzy);
    if (yatzy_index === -1) {
        return null;
    }
    value <<= bit_lengths.yatzy;
    value |= BigInt(yatzy_index);

    return value;
};

export const into_state = (value: BigInt): State | null => {
    const yatzy_index = value & (2n ** bit_lengths.yatzy - 1n);
    const yatzy = possible_values.yatzy[yatzy_index];
    if (yatzy === undefined) {
        return null;
    }
    value >>= bit_lengths.yatzy;

    const chance_index = value & (2n ** bit_lengths.chance - 1n);
    const chance = possible_values.chance[chance_index];
    if (chance === undefined) {
        return null;
    }
    value >>= bit_lengths.chance;

    const full_house_index = value & (2n ** bit_lengths.full_house - 1n);
    const full_house = possible_values.full_house[full_house_index];
    if (full_house === undefined) {
        return null;
    }
    value >>= bit_lengths.full_house;

    const large_straight_index =
        value & (2n ** bit_lengths.large_straight - 1n);
    const large_straight = possible_values.large_straight[large_straight_index];
    if (large_straight === undefined) {
        return null;
    }
    value >>= bit_lengths.large_straight;

    const small_straight_index =
        value & (2n ** bit_lengths.small_straight - 1n);
    const small_straight = possible_values.small_straight[small_straight_index];
    if (small_straight === undefined) {
        return null;
    }
    value >>= bit_lengths.small_straight;

    const four_of_a_kind_index =
        value & (2n ** bit_lengths.four_of_a_kind - 1n);
    const four_of_a_kind = possible_values.four_of_a_kind[four_of_a_kind_index];
    if (four_of_a_kind === undefined) {
        return null;
    }
    value >>= bit_lengths.four_of_a_kind;

    const three_of_a_kind_index =
        value & (2n ** bit_lengths.three_of_a_kind - 1n);
    const three_of_a_kind =
        possible_values.three_of_a_kind[three_of_a_kind_index];
    if (three_of_a_kind === undefined) {
        return null;
    }
    value >>= bit_lengths.three_of_a_kind;

    const two_pairs_index = value & (2n ** bit_lengths.two_pairs - 1n);
    const two_pairs = possible_values.two_pairs[two_pairs_index];
    if (two_pairs === undefined) {
        return null;
    }
    value >>= bit_lengths.two_pairs;

    const one_pair_index = value & (2n ** bit_lengths.one_pair - 1n);
    const one_pair = possible_values.one_pair[one_pair_index];
    if (one_pair === undefined) {
        return null;
    }
    value >>= bit_lengths.one_pair;

    const sixes_index = value & (2n ** bit_lengths.sixes - 1n);
    const sixes = possible_values.sixes[sixes_index];
    if (sixes === undefined) {
        return null;
    }
    value >>= bit_lengths.sixes;

    const fives_index = value & (2n ** bit_lengths.fives - 1n);
    const fives = possible_values.fives[fives_index];
    if (fives === undefined) {
        return null;
    }
    value >>= bit_lengths.fives;

    const fours_index = value & (2n ** bit_lengths.fours - 1n);
    const fours = possible_values.fours[fours_index];
    if (fours === undefined) {
        return null;
    }
    value >>= bit_lengths.fours;

    const threes_index = value & (2n ** bit_lengths.threes - 1n);
    const threes = possible_values.threes[threes_index];
    if (threes === undefined) {
        return null;
    }
    value >>= bit_lengths.threes;

    const twos_index = value & (2n ** bit_lengths.twos - 1n);
    const twos = possible_values.twos[twos_index];
    if (twos === undefined) {
        return null;
    }
    value >>= bit_lengths.twos;

    const ones_index = value & (2n ** bit_lengths.ones - 1n);
    const ones = possible_values.ones[ones_index];
    if (ones === undefined) {
        return null;
    }
    value >>= bit_lengths.ones;

    const rerolls_left = Number(value & (2n ** bit_lengths.rerolls_left - 1n));
    if (rerolls_left > 2) {
        return null;
    }
    value >>= bit_lengths.rerolls_left;

    const die4 = Number(value & (2n ** bit_lengths.die - 1n));
    if (die4 < 1 || die4 > 6) {
        return null;
    }
    value >>= bit_lengths.die;

    const die3 = Number(value & (2n ** bit_lengths.die - 1n));
    if (die3 < 1 || die3 > 6) {
        return null;
    }
    value >>= bit_lengths.die;

    const die2 = Number(value & (2n ** bit_lengths.die - 1n));
    if (die2 < 1 || die2 > 6) {
        return null;
    }
    value >>= bit_lengths.die;

    const die1 = Number(value & (2n ** bit_lengths.die - 1n));
    if (die1 < 1 || die1 > 6) {
        return null;
    }
    value >>= bit_lengths.die;

    const die0 = Number(value & (2n ** bit_lengths.die - 1n));
    if (die0 < 1 || die0 > 6) {
        return null;
    }
    value >>= bit_lengths.die;

    if (value !== 0n) {
        return null;
    }

    return {
        dice: [die0, die1, die2, die3, die4],
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
    };
};
