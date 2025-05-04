<script lang="ts">
    interface Props {
        name: string;
        pattern: string;
        placeholder: string;
        title: string;
        validator: (value: string, type: "input" | "query") => boolean;
        value: number | null;
    }

    let {
        name,
        pattern,
        placeholder,
        title,
        validator,
        value = $bindable(null),
    }: Props = $props();

    let input = $state(value === null ? "" : String(value));

    const get = () => input;

    const set = (v: string) => {
        input = v;
        if (validator(v)) {
            if (v === "") {
                value = null;
            } else {
                value = Number(v);
            }
        }
    };
</script>

<input
    bind:value={get, set}
    inputmode="numeric"
    {name}
    {pattern}
    {placeholder}
    {title}
/>

<style lang="scss">
    input {
        width: 100%;
        padding: 0;
        background: none;
        border: none;
        border-bottom: 1px solid white;
        outline: none;
        text-align: center;
        font-family: inherit;
        font-size: inherit;
        color: inherit;

        &:invalid {
            color: #de7c4c;
        }
    }
</style>
