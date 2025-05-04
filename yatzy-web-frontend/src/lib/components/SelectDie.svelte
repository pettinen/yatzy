<script lang="ts">
    import type { Die } from "$lib/types";

    import DieIcon from "$lib/components/DieIcon.svelte";

    interface Props {
        value: Die;
    }

    let { value = $bindable() }: Props = $props();

    const increment = () => {
        if (value === 6) {
            value = 1;
        } else {
            value += 1;
        }
    };

    const decrement = () => {
        if (value === 1) {
            value = 6;
        } else {
            value -= 1;
        }
    };
</script>

<button
    aria-label={`Noppa: ${value}`}
    oncontextmenu={(event) => {
        event.preventDefault();
    }}
    onmousedown={(event) => {
        event.preventDefault();
        if (event.button === 0) {
            increment();
        } else if (event.button === 2) {
            decrement();
        }
    }}
    onwheel={(event) => {
        event.preventDefault();
        if (event.deltaY < 0) {
            increment();
        } else if (event.deltaY > 0) {
            decrement();
        }
    }}
>
    <DieIcon {value} />
</button>

<style>
    button {
        width: 3.5rem;
        height: 3.5rem;
        background: #111111;
        border: none;
        border-radius: 0.5rem;
        color: inherit;
        cursor: pointer;
    }
</style>
