import { RollupContract } from "nacho-rollup-contract"
import { ZkProof } from "nacho-proof-generator"
import { Mina, PrivateKey } from "o1js"

export const submitMergedProof = async (
    mergedProof: ZkProof,
    rollupContract: RollupContract,
    txSender: PrivateKey,
) => {
    const tx = await Mina.transaction(txSender.toPublicKey(), () => {
        rollupContract.settle(mergedProof)
    })

    tx.sign([txSender])
    await tx.prove()
    await tx.send()
}
