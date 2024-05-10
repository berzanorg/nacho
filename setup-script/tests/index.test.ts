import { describe, test } from "node:test"
import { setup } from "../src/setup.js"

describe("Setup script test", async () => {
    test("Setups", async () => {
        await setup({
            isTest: true,
        })
    })
})
