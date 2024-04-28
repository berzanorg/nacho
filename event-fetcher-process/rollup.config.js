import typescript from "@rollup/plugin-typescript"

const config = [
    {
        input: "build/compiled/index.js",
        output: {
            file: "build/event-fetcher-process.mjs",
            format: "es",
        },
        plugins: [typescript()],
    },
]
export default config
