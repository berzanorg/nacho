import typescript from "@rollup/plugin-typescript"

const config = [
    {
        input: "build/compiled/index.js",
        output: {
            file: "build/proof-submitter-process.mjs",
            format: "es",
        },
        plugins: [typescript()],
    },
]
export default config
