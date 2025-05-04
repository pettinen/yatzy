import { error, redirect } from "@sveltejs/kit";

import { base } from "$app/paths";

import { into_state } from "$lib/state.ts";
import type { State } from "$lib/state.ts";
import type { Choice, Die, Score } from "$lib/types.ts";

import type { PageServerLoad } from "./$types.d.ts";

const CACHE: Map<string, Choice[]> = new Map();
const MAX_CACHE_SIZE = 1000;

export const load: PageServerLoad = async ({ fetch, params, request }) => {
    if (!/^(0|[1-9a-f][\da-f]*)$/u.test(params.state)) {
        error(404);
    }

    let state_num: BigInt;
    try {
        state_num = BigInt(`0x${params.state}`);
    } catch {
        error(404);
    }

    const state = into_state(state_num);
    if (!state) {
        error(404);
    }

    if (
        state.ones === null ||
        state.twos === null ||
        state.threes === null ||
        state.fours === null ||
        state.fives === null ||
        state.sixes === null ||
        state.one_pair === null ||
        state.two_pairs === null ||
        state.three_of_a_kind === null ||
        state.four_of_a_kind === null ||
        state.small_straight === null ||
        state.large_straight === null ||
        state.full_house === null ||
        state.chance === null ||
        state.yatzy === null
    ) {
        const cache_key = [
            state.dice.join(""),
            state.rerolls_left,
            state.ones,
            state.twos,
            state.threes,
            state.fours,
            state.fives,
            state.sixes,
            state.one_pair,
            state.two_pairs,
            state.three_of_a_kind,
            state.four_of_a_kind,
            state.small_straight,
            state.large_straight,
            state.full_house,
            state.chance,
            state.yatzy,
        ].join(":");
        const cached = CACHE.get(cache_key);

        let choices: Choice[] | null = [];

        if (cached) {
            choices = cached;
            CACHE.delete(cache_key);
            CACHE.set(cache_key, choices);
        } else {
            const api_url = new URL("http://127.0.0.1:54460/");
            api_url.searchParams.append("dice", [...state.dice].join());
            api_url.searchParams.append("rerolls_left", state.rerolls_left);
            api_url.searchParams.append("ones", state.ones ?? "empty");
            api_url.searchParams.append("twos", state.twos ?? "empty");
            api_url.searchParams.append("threes", state.threes ?? "empty");
            api_url.searchParams.append("fours", state.fours ?? "empty");
            api_url.searchParams.append("fives", state.fives ?? "empty");
            api_url.searchParams.append("sixes", state.sixes ?? "empty");
            api_url.searchParams.append("one_pair", state.one_pair ?? "empty");
            api_url.searchParams.append(
                "two_pairs",
                state.two_pairs ?? "empty",
            );
            api_url.searchParams.append(
                "three_of_a_kind",
                state.three_of_a_kind ?? "empty",
            );
            api_url.searchParams.append(
                "four_of_a_kind",
                state.four_of_a_kind ?? "empty",
            );
            api_url.searchParams.append(
                "small_straight",
                state.small_straight ?? "empty",
            );
            api_url.searchParams.append(
                "large_straight",
                state.large_straight ?? "empty",
            );
            api_url.searchParams.append(
                "full_house",
                state.full_house ?? "empty",
            );
            api_url.searchParams.append("chance", state.chance ?? "empty");
            api_url.searchParams.append("yatzy", state.yatzy ?? "empty");

            try {
                const response = await fetch(api_url);
                const data = await response.json();
                if (data.errors) {
                    console.error("errors in API response:", data.errors);
                    choices = null;
                } else if (Array.isArray(data)) {
                    for (const item of data) {
                        if (typeof item === "object" && item !== null) {
                            if (item.choice === "select_combo") {
                                choices.push(item.combo);
                            } else if (item.choice === "reroll") {
                                choices.push(item.dice);
                            } else {
                                choices = null;
                                break;
                            }
                        } else {
                            choices = null;
                            break;
                        }
                    }
                } else {
                    choices = null;
                }
            } catch (error) {
                console.error("fetch from API failed:", error);
                choices = null;
            }

            if (choices) {
                CACHE.set(cache_key, choices);
                if (CACHE.size > MAX_CACHE_SIZE) {
                    const earliest_cache_key = CACHE.keys().next().value;
                    if (earliest_cache_key !== undefined) {
                        CACHE.delete(earliest_cache_key);
                    }
                }
            }
        }

        if (choices) {
            state.choices = choices;
        }
    }

    if (process.env.CODE_URLS) {
        const code_url = JSON.parse(process.env.CODE_URLS)[
            request.headers.get("X-Forwarded-Host")
        ];
        if (code_url) {
            state.code_url = code_url;
        }
    }

    return state;
};
