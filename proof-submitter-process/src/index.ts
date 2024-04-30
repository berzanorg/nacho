import { stdin, stdout } from "node:process"
import { submitMergedProof } from "./submit"
import { RollupContract } from "nacho-rollup-contract"
import { Mina, PrivateKey, PublicKey } from "o1js"
import { readMergedProofFromDisk } from "./utils"

const proofDbPath = process.argv.at(2)
const txSenderPrivateKeyAsBase58 = process.argv.at(3)
const minaNodeUrl = process.argv.at(4)
const rollupContractAddress = process.argv.at(5)

if (
    proofDbPath === undefined ||
    txSenderPrivateKeyAsBase58 === undefined ||
    minaNodeUrl === undefined ||
    rollupContractAddress === undefined
) {
    process.exit(1)
}

const txSender = PrivateKey.fromBase58(txSenderPrivateKeyAsBase58)

Mina.setActiveInstance(Mina.Network(minaNodeUrl))

const rollupContractPublicKey = PublicKey.fromBase58(rollupContractAddress)

await RollupContract.compile()
const rollupContract = new RollupContract(rollupContractPublicKey)

stdout.write(new Uint8Array(new ArrayBuffer(1)))

stdin.on("data", async () => {
    const buffer = new ArrayBuffer(1)
    const array = new Uint8Array(buffer)
    let ok = false

    try {
        const mergedProof = await readMergedProofFromDisk(proofDbPath)
        await submitMergedProof(mergedProof, rollupContract, txSender)
        ok = true
    } catch {}

    array[0] = Number(ok)
    stdout.write(array)
})
