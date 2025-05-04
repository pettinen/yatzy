import type { Config } from "prettier";

const config: Config = {
    tabWidth: 4,
    plugins: ["prettier-plugin-svelte"],
    overrides: [
        {
            files: "*.svelte",
            options: {
                parser: "svelte",
            },
        },
    ],
};

export default config;
