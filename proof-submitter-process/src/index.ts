import { stdin, stdout } from "node:process"
import { submitMergedProof } from "./submit"
import { RollupContract } from "nacho-rollup-contract"
import { Mina, PrivateKey, PublicKey } from "o1js"
import { readMergedProofFromDisk } from "./utils"

const proofsPath = process.env.NACHO_PROOFS_PATH
const privateKeyAsBase58 = process.env.NACHO_SUBMITTER_PRIVATE_KEY
const minaGraphqlUrl = process.env.NACHO_MINA_GRAPHQL_URL
const minaArchiveUrl = process.env.NACHO_MINA_ARCHIVE_URL
const rollupContractAddress = process.env.NACHO_ROLLUP_CONTRACT_ADDRESS

if (
    proofsPath === undefined ||
    privateKeyAsBase58 === undefined ||
    minaGraphqlUrl === undefined ||
    minaArchiveUrl === undefined ||
    rollupContractAddress === undefined
) {
    process.exit(1)
}

Mina.setActiveInstance(
    Mina.Network({
        mina: minaGraphqlUrl,
        archive: minaArchiveUrl,
    }),
)

const txSender = PrivateKey.fromBase58(privateKeyAsBase58)
const rollupContractPublicKey = PublicKey.fromBase58(rollupContractAddress)
const rollupContract = new RollupContract(rollupContractPublicKey)

await RollupContract.compile()

stdout.write(new Uint8Array(new ArrayBuffer(1)))

stdin.on("data", async () => {
    const buffer = new ArrayBuffer(1)
    const array = new Uint8Array(buffer)
    let ok = false

    try {
        const mergedProof = await readMergedProofFromDisk(proofsPath)
        await submitMergedProof(mergedProof, rollupContract, txSender)
        ok = true
    } catch {}

    array[0] = Number(ok)
    stdout.write(array)
})
