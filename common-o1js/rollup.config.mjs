import { dts } from "rollup-plugin-dts"

export default [
    {
        input: "build/compiled/index.js",
        output: {
            file: "build/index.js",
            format: "es",
        },
    },
    {
        input: "build/compiled/index.d.ts",
        output: {
            file: "build/index.d.ts",
            format: "es",
        },
        plugins: [dts()],
    },
]
