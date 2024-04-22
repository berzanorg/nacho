import { StateRoots } from "nacho-common-o1js"
import { proofGenerator } from "nacho-proof-generator"
import { Field } from "o1js"
import { MergeProofs } from "./input.js"
import { readProofFromDisk } from "./utils.js"

export const mergeProofs = async (params: MergeProofs, proofDbPath: string) => {
    const proof = await proofGenerator.mergeProofs(
        new StateRoots({
            balances: Field(params.state_roots.balances),
            liquidities: Field(params.state_roots.liquidities),
            pools: Field(params.state_roots.pools),
            burns: Field(params.state_roots.burns),
        }),
        await readProofFromDisk(proofDbPath, params.first_proof_index),
        await readProofFromDisk(proofDbPath, params.second_proof_index),
    )

    return proof
}
