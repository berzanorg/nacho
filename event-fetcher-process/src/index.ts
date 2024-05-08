import { stdin, stdout } from "node:process"
import { parseInput } from "./parse"
import { fetchDepositedEvents, fetchWithdrawnEvents } from "./fetch"
import { BridgeContract } from "nacho-bridge-contract"
import { Mina, PublicKey } from "o1js"
import { unparseError, unparseOutput } from "./output"

const minaGraphqlUrl = process.env.NACHO_MINA_GRAPHQL_URL
const minaArchiveUrl = process.env.NACHO_MINA_ARCHIVE_URL
const bridgeContractAddress = process.env.NACHO_BRIDGE_CONTRACT_ADDRESS

if (
    minaGraphqlUrl === undefined ||
    minaArchiveUrl === undefined ||
    bridgeContractAddress === undefined
) {
    process.exit(1)
}

Mina.setActiveInstance(
    Mina.Network({
        mina: minaGraphqlUrl,
        archive: minaArchiveUrl,
    }),
)

const bridgeContractPublicKey = PublicKey.fromBase58(bridgeContractAddress)

const bridgeContract = new BridgeContract(bridgeContractPublicKey)

stdin.on("data", async (chunk) => {
    try {
        const input = parseInput(chunk)

        const { events, last_fetched_block } =
            input.kind === "FetchDepositedEvents"
                ? await fetchDepositedEvents(input.fromBlock, bridgeContract)
                : input.kind === "FetchWithdrawnEvents"
                ? await fetchWithdrawnEvents(input.fromBlock, bridgeContract)
                : { events: null, last_fetched_block: null }

        if (events && last_fetched_block) {
            const buffer = unparseOutput(events, last_fetched_block)

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
