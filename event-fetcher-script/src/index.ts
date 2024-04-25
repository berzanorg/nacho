import { stdin, stdout } from "node:process"
import { parseInput } from "./parse.js"
import { fetchDepositedEvents, fetchWithdrawnEvents } from "./fetch.js"
import { BridgeContract } from "nacho-bridge-contract"
import { Mina, PublicKey } from "o1js"
import { unparseError, unparseOutput } from "./output.js"

const main = async () => {
    const minaNodeUrl = process.argv.at(2)
    const bridgeContractAddress = process.argv.at(3)

    if (minaNodeUrl === undefined || bridgeContractAddress === undefined) {
        process.exit(1)
    }

    Mina.setActiveInstance(Mina.Network(minaNodeUrl))

    const bridgeContractPublicKey = PublicKey.fromBase58(bridgeContractAddress)

    await BridgeContract.compile()
    const bridgeContract = new BridgeContract(bridgeContractPublicKey)

    stdout.write(new Uint8Array(new ArrayBuffer(1)))

    stdin.on("data", async (chunk) => {
        try {
            const input = parseInput(chunk)

            const events =
                input.kind === "FetchDepositedEvents"
                    ? await fetchDepositedEvents(input.fromBlock, bridgeContract)
                    : input.kind === "FetchWithdrawnEvents"
                    ? await fetchWithdrawnEvents(input.fromBlock, bridgeContract)
                    : null

            if (events) {
                const buffer = unparseOutput(events)

                stdout.write(buffer)
            } else {
                const buffer = unparseError()

                stdout.write(buffer)
            }
        } catch (error) {
            const buffer = unparseError()

            stdout.write(buffer)
        }
    })
}

main()
