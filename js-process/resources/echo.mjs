import { stdin, stdout } from "node:process"

stdin.on("data", async (chunk) => {
    stdout.write(chunk)
})
