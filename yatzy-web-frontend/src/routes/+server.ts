import { redirect } from "@sveltejs/kit";

import { base } from "$app/paths";

import { validators } from "$lib/game.ts";
import { from_state } from "$lib/state.ts";
import type { Choice, Dice, Score } from "$lib/types.ts";

import type { RequestHandler } from "./$types.d.ts";

export const GET: RequestHandler = async ({ fetch, request, url }) => {
    console.log("new request from", request.headers.get("X-Forwarded-For"));

    const query = url.searchParams;
    const dice_str = query.getAll("dice").join("");
    const rerolls_left_str = query.get("rerolls_left");
    const ones_str = query.get("ones") ?? "";
    const twos_str = query.get("twos") ?? "";
    const threes_str = query.get("threes") ?? "";
    const fours_str = query.get("fours") ?? "";
    const fives_str = query.get("fives") ?? "";
    const sixes_str = query.get("sixes") ?? "";
    const one_pair_str = query.get("one_pair") ?? "";
    const two_pairs_str = query.get("two_pairs") ?? "";
    const three_of_a_kind_str = query.get("three_of_a_kind") ?? "";
    const four_of_a_kind_str = query.get("four_of_a_kind") ?? "";
    const small_straight_str = query.get("small_straight") ?? "";
    const large_straight_str = query.get("large_straight") ?? "";
    const full_house_str = query.get("full_house") ?? "";
    const chance_str = query.get("chance") ?? "";
    const yatzy_str = query.get("yatzy") ?? "";

    let dice: Dice;
    if (validators.dice(dice_str)) {
        dice = [...dice_str].map(Number);
    } else {
        dice = [1, 2, 3, 4, 5];
    }

    let rerolls_left: number;
    if (
        rerolls_left_str !== null &&
        validators.rerolls_left(rerolls_left_str)
    ) {
        rerolls_left = Number(rerolls_left_str);
    } else {
        rerolls_left = 2;
    }

    let ones: Score = null;
    if (ones_str && validators.ones(ones_str)) {
        ones = Number(ones_str);
    }

    let twos: Score = null;
    if (twos_str && validators.twos(twos_str)) {
        twos = Number(twos_str);
    }

    let threes: Score = null;
    if (threes_str && validators.threes(threes_str)) {
        threes = Number(threes_str);
    }

    let fours: Score = null;
    if (fours_str && validators.fours(fours_str)) {
        fours = Number(fours_str);
    }

    let fives: Score = null;
    if (fives_str && validators.fives(fives_str)) {
        fives = Number(fives_str);
    }

    let sixes: Score = null;
    if (sixes_str && validators.sixes(sixes_str)) {
        sixes = Number(sixes_str);
    }

    let one_pair: Score = null;
    if (one_pair_str && validators.one_pair(one_pair_str)) {
        one_pair = Number(one_pair_str);
    }

    let two_pairs: Score = null;
    if (two_pairs_str && validators.two_pairs(two_pairs_str)) {
        two_pairs = Number(two_pairs_str);
    }

    let three_of_a_kind: Score = null;
    if (
        three_of_a_kind_str &&
        validators.three_of_a_kind(three_of_a_kind_str)
    ) {
        three_of_a_kind = Number(three_of_a_kind_str);
    }

    let four_of_a_kind: Score = null;
    if (four_of_a_kind_str && validators.four_of_a_kind(four_of_a_kind_str)) {
        four_of_a_kind = Number(four_of_a_kind_str);
    }

    let small_straight: Score = null;
    if (small_straight_str && validators.small_straight(small_straight_str)) {
        small_straight = Number(small_straight_str);
    }

    let large_straight: Score = null;
    if (large_straight_str && validators.large_straight(large_straight_str)) {
        large_straight = Number(large_straight_str);
    }

    let full_house: Score = null;
    if (full_house_str && validators.full_house(full_house_str)) {
        full_house = Number(full_house_str);
    }

    let chance: Score = null;
    if (chance_str && validators.chance(chance_str)) {
        chance = Number(chance_str);
    }

    let yatzy: Score = null;
    if (yatzy_str && validators.yatzy(yatzy_str)) {
        yatzy = Number(yatzy_str);
    }

    const state = {
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
    };

    redirect(307, `${base}/${from_state(state).toString(16)}`);
};
