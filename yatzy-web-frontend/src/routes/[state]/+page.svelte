<script lang="ts">
    import "@fontsource-variable/outfit";
    import { untrack } from "svelte";

    import { replaceState } from "$app/navigation";
    import { base } from "$app/paths";

    import { points, possible_scores, validators } from "$lib/game.ts";
    import { from_state } from "$lib/state.ts";
    import type { Dice, Die, RerollsLeft, Score } from "$lib/types.ts";

    import DieIcon from "$lib/components/DieIcon.svelte";
    import ScoreField from "$lib/components/ScoreField.svelte";
    import SelectDie from "$lib/components/SelectDie.svelte";

    import type { PageProps } from "./$types";

    let { data }: PageProps = $props();

    let dice: Dice = $state(data.dice ?? [1, 2, 3, 4, 5]);
    let dice_inputs = $state([...dice]);
    let dice_string_sorted = $derived(dice.toSorted().join());

    let rerolls_left: RerollsLeft = $state(data.rerolls_left ?? 2);
    let ones: Score = $state(data.ones ?? null);
    let twos: Score = $state(data.twos ?? null);
    let threes: Score = $state(data.threes ?? null);
    let fours: Score = $state(data.fours ?? null);
    let fives: Score = $state(data.fives ?? null);
    let sixes: Score = $state(data.sixes ?? null);
    let one_pair: Score = $state(data.one_pair ?? null);
    let two_pairs: Score = $state(data.two_pairs ?? null);
    let three_of_a_kind: Score = $state(data.three_of_a_kind ?? null);
    let four_of_a_kind: Score = $state(data.four_of_a_kind ?? null);
    let small_straight: Score = $state(data.small_straight ?? null);
    let large_straight: Score = $state(data.large_straight ?? null);
    let full_house: Score = $state(data.full_house ?? null);
    let chance: Score = $state(data.chance ?? null);
    let yatzy: Score = $state(data.yatzy ?? null);

    let upper_section_total = $derived(
        (ones ?? 0) +
            (twos ?? 0) +
            (threes ?? 0) +
            (fours ?? 0) +
            (fives ?? 0) +
            (sixes ?? 0),
    );

    let bonus = $derived(upper_section_total >= 63 ? 50 : 0);

    let total = $derived(
        upper_section_total +
            bonus +
            (one_pair ?? 0) +
            (two_pairs ?? 0) +
            (three_of_a_kind ?? 0) +
            (four_of_a_kind ?? 0) +
            (small_straight ?? 0) +
            (large_straight ?? 0) +
            (full_house ?? 0) +
            (chance ?? 0) +
            (yatzy ?? 0),
    );

    let round = $derived(
        (ones === null ? 0 : 1) +
            (twos === null ? 0 : 1) +
            (threes === null ? 0 : 1) +
            (fours === null ? 0 : 1) +
            (fives === null ? 0 : 1) +
            (sixes === null ? 0 : 1) +
            (one_pair === null ? 0 : 1) +
            (two_pairs === null ? 0 : 1) +
            (three_of_a_kind === null ? 0 : 1) +
            (four_of_a_kind === null ? 0 : 1) +
            (small_straight === null ? 0 : 1) +
            (large_straight === null ? 0 : 1) +
            (full_house === null ? 0 : 1) +
            (chance === null ? 0 : 1) +
            (yatzy === null ? 0 : 1),
    );
    let game_ended = $derived(round === 15);

    let show_placeholder = $state(false);
    const available_points = $derived({
        ones: points.ones(dice),
        twos: points.twos(dice),
        threes: points.threes(dice),
        fours: points.fours(dice),
        fives: points.fives(dice),
        sixes: points.sixes(dice),
        one_pair: points.one_pair(dice),
        two_pairs: points.two_pairs(dice),
        three_of_a_kind: points.three_of_a_kind(dice),
        four_of_a_kind: points.four_of_a_kind(dice),
        small_straight: points.small_straight(dice),
        large_straight: points.large_straight(dice),
        full_house: points.full_house(dice),
        chance: points.chance(dice),
        yatzy: points.yatzy(dice),
    });

    const combo_values: Record<Choice, number> = {
        ones: 0,
        twos: 1,
        threes: 2,
        fours: 3,
        fives: 4,
        sixes: 5,
        one_pair: 6,
        two_pairs: 7,
        three_of_a_kind: 8,
        four_of_a_kind: 9,
        small_straight: 10,
        large_straight: 11,
        full_house: 12,
        chance: 13,
        yatzy: 14,
    };
    const choice_value = (choice: Choice): number => {
        if (typeof choice === "string") {
            return combo_values[choice];
        } else {
            let value = choice[0] * 100;
            if (choice.length > 1) {
                value += choice[1] * 1_000;
            }
            if (choice.length > 2) {
                value += choice[2] * 10_000;
            }
            if (choice.length > 3) {
                value += choice[3] * 100_000;
            }
            if (choice.length > 4) {
                value += choice[4] * 1_000_000;
            }
            return value;
        }
    };
    const choice_sort = (a: Choice, b: Choice) =>
        choice_value(a) - choice_value(b);

    let fetch_abort_controller: AbortController | null = $state(null);
    let fetch_timeout: number | null = $state(null);
    let best_choices: Choice[] | "error" | null = $state(data.choices ?? null);

    if (Array.isArray(best_choices)) {
        best_choices.sort(choice_sort);
    }

    const fetch_best_choices = async (url: string) => {
        if (fetch_abort_controller) {
            fetch_abort_controller.abort();
        }

        fetch_abort_controller = new AbortController();

        let data: unknown;
        try {
            const response = await fetch(url, {
                signal: fetch_abort_controller.signal,
            });
            data = await response.json();
        } catch (error) {
            if (
                !(error instanceof DOMException) ||
                error.name !== "AbortError"
            ) {
                console.error("failed to fetch from API:", error);
                best_choices = "error";
                fetch_abort_controller = null;
                fetch_timeout = null;
            }
            return;
        }

        if (Array.isArray(data)) {
            best_choices = [];
            for (const item of data) {
                if (typeof item === "object" && item !== null) {
                    if (item.choice === "select_combo") {
                        best_choices.push(item.combo);
                    } else if (item.choice === "reroll") {
                        best_choices.push(item.dice);
                    } else {
                        best_choices = "error";
                        break;
                    }
                } else {
                    best_choices = "error";
                    break;
                }
            }
        } else {
            best_choices = "error";
        }

        if (Array.isArray(best_choices)) {
            best_choices.sort(choice_sort);
        }

        fetch_abort_controller = null;
        fetch_timeout = null;
    };

    const query = $derived.by(() => {
        const query = new URLSearchParams();
        query.append("dice", dice_string_sorted);
        query.append("rerolls_left", String(rerolls_left));
        query.append("ones", ones === null ? "empty" : String(ones));
        query.append("twos", twos === null ? "empty" : String(twos));
        query.append("threes", threes === null ? "empty" : String(threes));
        query.append("fours", fours === null ? "empty" : String(fours));
        query.append("fives", fives === null ? "empty" : String(fives));
        query.append("sixes", sixes === null ? "empty" : String(sixes));
        query.append(
            "one_pair",
            one_pair === null ? "empty" : String(one_pair),
        );
        query.append(
            "two_pairs",
            two_pairs === null ? "empty" : String(two_pairs),
        );
        query.append(
            "three_of_a_kind",
            three_of_a_kind === null ? "empty" : String(three_of_a_kind),
        );
        query.append(
            "four_of_a_kind",
            four_of_a_kind === null ? "empty" : String(four_of_a_kind),
        );
        query.append(
            "small_straight",
            small_straight === null ? "empty" : String(small_straight),
        );
        query.append(
            "large_straight",
            large_straight === null ? "empty" : String(large_straight),
        );
        query.append(
            "full_house",
            full_house === null ? "empty" : String(full_house),
        );
        query.append("chance", chance === null ? "empty" : String(chance));
        query.append("yatzy", yatzy === null ? "empty" : String(yatzy));
        return query.toString();
    });

    const short_link = $derived.by(() => {
        const value = from_state({
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
        });

        if (value === null) {
            return null;
        }
        return `${base}/${value.toString(16)}`;
    });

    const score_field_title = (scores: number[]): string => {
        const parts = [];

        if (scores.length === 0) {
            return "";
        }
        if (scores.length === 1) {
            return String(scores[0]);
        }

        let i = 0;

        while (i < scores.length) {
            let j = i;
            for (; j < scores.length; j += 1) {
                if (
                    j === scores.length - 1 ||
                    scores[j + 1] !== scores[j] + 1
                ) {
                    break;
                }
            }
            if (j === i) {
                parts.push(String(scores[i]));
            } else if (j === i + 1) {
                parts.push(String(scores[i]));
                parts.push(String(scores[j]));
            } else {
                parts.push(`${scores[i]}\u2013${scores[j]}`);
            }
            i = j + 1;
        }

        if (parts.length === 0) {
            return "";
        } else if (parts.length === 1) {
            return parts[0];
        } else {
            return `${parts.slice(0, -1).join(", ")} tai ${parts[parts.length - 1]}`;
        }
    };

    $effect(() => {
        if (short_link) {
            try {
                replaceState(short_link, null);
            } catch {}
        }
    });

    let fetch_effect_first_run = true;
    $effect(() => {
        const query_dependency = query;
        if (fetch_effect_first_run) {
            fetch_effect_first_run = false;
            return;
        }

        if (game_ended) {
            return;
        }

        const timeout_id = untrack(() => fetch_timeout);
        if (timeout_id !== null) {
            clearTimeout(timeout_id);
        }
        fetch_timeout = setTimeout(
            fetch_best_choices,
            500,
            `${base}/api?${query_dependency}`,
        );
    });
</script>

<svelte:head>
    <title>Yatzy</title>
</svelte:head>

<main>
    <form
        action={base}
        id="form"
        onsubmit={(event) => {
            event.preventDefault();
        }}
    >
        <div class="current-dice-and-rerolls">
            <div>
                <div class="yesscript">
                    <div class="current-dice-and-sort">
                        <div class="current-dice">
                            {#each { length: 5 }, index}
                                <SelectDie bind:value={dice[index]} />
                            {/each}
                        </div>

                        {#if dice.join() !== dice_string_sorted}
                            <button
                                class="sort-button"
                                title="Järjestä nopat"
                                type="button"
                                onclick={() => {
                                    dice.sort();
                                }}
                            >
                                <svg
                                    width="24"
                                    height="24"
                                    stroke-width="1.5"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <path
                                        d="M14 14L2 14"
                                        stroke="currentColor"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    />
                                    <path
                                        d="M10 10H2"
                                        stroke="currentColor"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    />
                                    <path
                                        d="M6 6H2"
                                        stroke="currentColor"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    />
                                    <path
                                        d="M18 18H2"
                                        stroke="currentColor"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    />
                                    <path
                                        d="M19 14V4M19 4L22 7M19 4L16 7"
                                        stroke="currentColor"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    />
                                </svg>
                            </button>
                        {/if}
                    </div>
                </div>
                <noscript class="noscript-dice">
                    <h2>Nopat</h2>
                    <div class="noscript-dice-dice">
                        {#each { length: 5 }, index}
                            <input
                                class="noscript-dice-input"
                                name="dice"
                                type="number"
                                min="1"
                                max="6"
                                pattern="[1-6]"
                                required
                                value={dice[index]}
                            />
                        {/each}
                    </div>
                </noscript>
            </div>
            <div class="rerolls-left">
                <h2>Heittoja jäljellä</h2>
                <div class="rerolls-left-buttons yesscript">
                    {#each { length: 3 }, rerolls}
                        <button
                            class="rerolls-left-button"
                            type="button"
                            onclick={() => {
                                rerolls_left = rerolls;
                            }}
                        >
                            <span
                                class="rerolls-left-button-inner"
                                class:selected={rerolls_left === rerolls}
                            >
                                {rerolls}
                            </span>
                        </button>
                    {/each}
                </div>
                <noscript class="noscript-rerolls-left">
                    <label>
                        <input
                            type="radio"
                            name="rerolls_left"
                            required
                            value="0"
                            checked={rerolls_left === 0}
                        /> 0
                    </label>
                    <label>
                        <input
                            type="radio"
                            name="rerolls_left"
                            required
                            value="1"
                            checked={rerolls_left === 1}
                        /> 1
                    </label>
                    <label>
                        <input
                            type="radio"
                            name="rerolls_left"
                            required
                            value="2"
                            checked={rerolls_left === 2}
                        /> 2
                    </label>
                </noscript>
            </div>
        </div>

        <table class="scores">
            <tbody>
                <tr>
                    <th>Ykköset</th>
                    <td
                        ><ScoreField
                            validator={validators.ones}
                            bind:value={ones}
                            name="ones"
                            pattern={possible_scores.ones.join("|")}
                            placeholder={show_placeholder &&
                            available_points.ones
                                ? String(available_points.ones)
                                : ""}
                            title={score_field_title(possible_scores.ones)}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Kakkoset</th>
                    <td
                        ><ScoreField
                            validator={validators.twos}
                            bind:value={twos}
                            name="twos"
                            pattern={possible_scores.twos.join("|")}
                            placeholder={show_placeholder &&
                            available_points.twos
                                ? String(available_points.twos)
                                : ""}
                            title={score_field_title(possible_scores.twos)}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Kolmoset</th>
                    <td
                        ><ScoreField
                            validator={validators.threes}
                            bind:value={threes}
                            name="threes"
                            pattern={possible_scores.threes.join("|")}
                            placeholder={show_placeholder &&
                            available_points.threes
                                ? String(available_points.threes)
                                : ""}
                            title={score_field_title(possible_scores.threes)}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Neloset</th>
                    <td
                        ><ScoreField
                            validator={validators.fours}
                            bind:value={fours}
                            name="fours"
                            pattern={possible_scores.fours.join("|")}
                            placeholder={show_placeholder &&
                            available_points.fours
                                ? String(available_points.fours)
                                : ""}
                            title={score_field_title(possible_scores.fours)}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Viitoset</th>
                    <td
                        ><ScoreField
                            validator={validators.fives}
                            bind:value={fives}
                            name="fives"
                            pattern={possible_scores.fives.join("|")}
                            placeholder={show_placeholder &&
                            available_points.fives
                                ? String(available_points.fives)
                                : ""}
                            title={score_field_title(possible_scores.fives)}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Kuutoset</th>
                    <td
                        ><ScoreField
                            validator={validators.sixes}
                            bind:value={sixes}
                            name="sixes"
                            pattern={possible_scores.sixes.join("|")}
                            placeholder={show_placeholder &&
                            available_points.sixes
                                ? String(available_points.sixes)
                                : ""}
                            title={score_field_title(possible_scores.sixes)}
                        /></td
                    >
                </tr>
                <tr class="faux-score">
                    <th>Välisumma</th>
                    <td>{upper_section_total}</td>
                </tr>
                <tr class="faux-score bonus-row">
                    <th>Bonus</th>
                    <td>{upper_section_total >= 63 ? "50" : "0"}</td>
                </tr>
                <tr class="row-below-bonus">
                    <th>Pari</th>
                    <td
                        ><ScoreField
                            validator={validators.one_pair}
                            bind:value={one_pair}
                            name="one_pair"
                            pattern={possible_scores.one_pair.join("|")}
                            placeholder={show_placeholder &&
                            available_points.one_pair
                                ? String(available_points.one_pair)
                                : ""}
                            title={score_field_title(possible_scores.one_pair)}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Kaksi paria</th>
                    <td
                        ><ScoreField
                            validator={validators.two_pairs}
                            bind:value={two_pairs}
                            name="two_pairs"
                            pattern={possible_scores.two_pairs.join("|")}
                            placeholder={show_placeholder &&
                            available_points.two_pairs
                                ? String(available_points.two_pairs)
                                : ""}
                            title={score_field_title(possible_scores.two_pairs)}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Kolme samaa</th>
                    <td
                        ><ScoreField
                            validator={validators.three_of_a_kind}
                            bind:value={three_of_a_kind}
                            name="three_of_a_kind"
                            pattern={possible_scores.three_of_a_kind.join("|")}
                            placeholder={show_placeholder &&
                            available_points.three_of_a_kind
                                ? String(available_points.three_of_a_kind)
                                : ""}
                            title={score_field_title(
                                possible_scores.three_of_a_kind,
                            )}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Neljä samaa</th>
                    <td
                        ><ScoreField
                            validator={validators.four_of_a_kind}
                            bind:value={four_of_a_kind}
                            name="four_of_a_kind"
                            pattern={possible_scores.four_of_a_kind.join("|")}
                            placeholder={show_placeholder &&
                            available_points.four_of_a_kind
                                ? String(available_points.four_of_a_kind)
                                : ""}
                            title={score_field_title(
                                possible_scores.four_of_a_kind,
                            )}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Pieni suora</th>
                    <td
                        ><ScoreField
                            validator={validators.small_straight}
                            bind:value={small_straight}
                            name="small_straight"
                            pattern={possible_scores.small_straight.join("|")}
                            placeholder={show_placeholder &&
                            available_points.small_straight
                                ? String(available_points.small_straight)
                                : ""}
                            title={score_field_title(
                                possible_scores.small_straight,
                            )}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Iso suora</th>
                    <td
                        ><ScoreField
                            validator={validators.large_straight}
                            bind:value={large_straight}
                            name="large_straight"
                            pattern={possible_scores.large_straight.join("|")}
                            placeholder={show_placeholder &&
                            available_points.large_straight
                                ? String(available_points.large_straight)
                                : ""}
                            title={score_field_title(
                                possible_scores.large_straight,
                            )}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Täyskäsi</th>
                    <td
                        ><ScoreField
                            validator={validators.full_house}
                            bind:value={full_house}
                            name="full_house"
                            pattern={possible_scores.full_house.join("|")}
                            placeholder={show_placeholder &&
                            available_points.full_house
                                ? String(available_points.full_house)
                                : ""}
                            title={score_field_title(
                                possible_scores.full_house,
                            )}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Sattuma</th>
                    <td
                        ><ScoreField
                            validator={validators.chance}
                            bind:value={chance}
                            name="chance"
                            pattern={possible_scores.chance.join("|")}
                            placeholder={show_placeholder &&
                            available_points.chance
                                ? String(available_points.chance)
                                : ""}
                            title={score_field_title(possible_scores.chance)}
                        /></td
                    >
                </tr>
                <tr>
                    <th>Yatzy</th>
                    <td
                        ><ScoreField
                            validator={validators.yatzy}
                            bind:value={yatzy}
                            name="yatzy"
                            pattern={possible_scores.yatzy.join("|")}
                            placeholder={show_placeholder &&
                            available_points.yatzy
                                ? String(available_points.yatzy)
                                : ""}
                            title={score_field_title(possible_scores.yatzy)}
                        /></td
                    >
                </tr>
                <tr class="faux-score">
                    <th>Yhteensä</th>
                    <td><strong>{total}</strong></td>
                </tr>
            </tbody>
        </table>
    </form>

    <label class="placeholder-toggle">
        <input bind:checked={show_placeholder} type="checkbox" /> Näytä saatavilla
        olevat pisteet
    </label>

    <div class="best-choice-container">
        <h2 class="best-choice-heading">
            Paras siirto
            <noscript>
                <button class="refresh-button" form="form">Päivitä</button>
            </noscript>
        </h2>

        {#if fetch_timeout !== null || fetch_abort_controller}
            <p class="no-best-choices">Ladataan...</p>
        {:else if game_ended}
            <p class="no-best-choices">Peli on päättynyt</p>
        {:else if best_choices === null}
            <p class="no-best-choices">N/A</p>
        {:else if best_choices === "error"}
            <p class="no-best-choices">Virhe</p>
        {:else}
            <ul class="best-choices">
                {#each best_choices as best_choice, index}
                    <li>
                        {#if best_choice === "ones"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} ykköset
                        {:else if best_choice === "twos"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} kakkoset
                        {:else if best_choice === "threes"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} kolmoset
                        {:else if best_choice === "fours"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} neloset
                        {:else if best_choice === "fives"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} viitoset
                        {:else if best_choice === "sixes"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} kuutoset
                        {:else if best_choice === "one_pair"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} pari
                        {:else if best_choice === "two_pairs"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} kaksi
                            paria
                        {:else if best_choice === "three_of_a_kind"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} kolme
                            samaa
                        {:else if best_choice === "four_of_a_kind"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} neljä
                            samaa
                        {:else if best_choice === "small_straight"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} pieni
                            suora
                        {:else if best_choice === "large_straight"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} iso suora
                        {:else if best_choice === "full_house"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} täyskäsi
                        {:else if best_choice === "chance"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} sattuma
                        {:else if best_choice === "yatzy"}
                            {#if index === 0}Valitse{:else}tai valitse{/if} Yatzy
                        {:else}
                            <span class="best-choice-reroll">
                                <span>
                                    {#if index === 0}Heitä{:else}tai heitä{/if} uudestaan
                                </span>
                                <span class="best-choice-reroll-dice">
                                    {#each best_choice as die}
                                        <span class="best-choice-reroll-die">
                                            <DieIcon value={die} />
                                        </span>
                                    {/each}
                                </span>
                            </span>
                        {/if}
                    </li>
                {/each}
            </ul>
        {/if}
    </div>
</main>

{#if data.code_url}
    <footer>
        <a href={data.code_url}>code</a>
    </footer>
{/if}

<style lang="scss">
    :global {
        body {
            height: 98dvh;
            margin: 1dvh 1dvw;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            align-items: center;
            background: #262626;
            font-family: "Outfit Variable", sans-serif;
            color: white;
        }
    }

    a {
        color: inherit;
    }

    h2 {
        margin: 0.5rem 0;
        font-weight: 600;
    }

    th {
        text-align: left;
        padding-right: 1rem;
    }

    tr {
        height: 1rem;
    }

    tr.faux-score th {
        font-weight: 400;
    }

    th {
        text-align: left;
    }

    td {
        width: 3rem;
        text-align: center;
    }

    button {
        background: none;
        border: none;
        border-radius: 0.25rem;
        font-size: inherit;
        font-family: inherit;
        color: inherit;
        cursor: pointer;
    }

    .current-dice-and-rerolls {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }

    .current-dice-and-sort {
        display: flex;
    }

    .current-dice {
        display: flex;
        gap: 0.5rem;
    }

    .sort-button {
        position: relative;
        left: 1rem;
        width: 0;
        padding: 0;
    }

    .rerolls-left {
        display: flex;
        flex-direction: column;
    }

    .rerolls-left-buttons {
        display: flex;
        justify-content: center;
        gap: 0.5rem;
    }

    .rerolls-left-button {
        width: 3rem;
        height: 3rem;
        display: flex;
        justify-content: center;
        align-items: center;
        border-radius: 0.5rem;
        background: #111111;
        font-size: 1.5rem;
    }

    .rerolls-left-button-inner {
        width: 2.2rem;
        height: 2.2rem;
        display: flex;
        justify-content: center;
        align-items: center;
        border-radius: 0.25rem;
    }

    .rerolls-left-button-inner.selected {
        outline: 2px solid white;
    }

    .scores {
        margin: 1rem auto;
        border-collapse: collapse;

        tr {
            height: 1.6rem;
        }

        .bonus-row {
            border-bottom: 2px solid white;
        }

        .row-below-bonus * {
            padding-top: 0.25rem;
        }
    }

    .placeholder-toggle {
        display: block;
        text-align: center;
    }

    .best-choice-container {
        width: fit-content;
        margin: 0 0.5rem;
    }

    .no-best-choices {
        margin: 0;
        font-size: 1.4rem;
    }

    .best-choices {
        margin: 0;
        padding: 0;
        list-style-type: none;
        font-size: 1.4rem;
    }

    .best-choice-reroll {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 0.5rem;
        white-space-collapse: break-spaces;
    }

    .best-choice-reroll-dice {
        flex-shrink: 0;
    }

    .best-choice-reroll-die {
        display: inline-block;
        width: 2rem;
        height: 2rem;
    }

    .best-choice-heading {
        display: flex;
        gap: 1rem;
    }

    .refresh-button {
        background: #afb97a;
        color: black;
    }

    .noscript-dice {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .noscript-dice-dice {
        display: flex;
        gap: 2rem;
    }

    .noscript-dice-input {
        width: 1.75rem;
        transform: scale(1.5);
        background: none;
        border: none;
        outline: none;
        font-family: inherit;
        color: inherit;

        &:invalid {
            color: #de7c4c;
        }
    }

    .noscript-rerolls-left {
        display: flex;
        justify-content: space-between;
    }

    footer {
        padding: 1rem 0.5rem 0.5rem;
    }
</style>
