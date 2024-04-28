import typescript from "@rollup/plugin-typescript"

const config = [
    {
        input: "build/compiled/index.js",
        output: {
            file: "build/proof-merger-process.mjs",
            format: "es",
        },
        plugins: [typescript()],
    },
]
export default config
