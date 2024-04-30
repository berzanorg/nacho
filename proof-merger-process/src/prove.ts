import { proofGenerator } from "nacho-proof-generator"
import { ContinueMerge, StartMerge } from "./input"
import { readMergedProofFromDisk, readProofFromDisk } from "./utils"

export const startMerge = async (params: StartMerge, proofsPath: string) => {
    const firstProof = await readProofFromDisk(proofsPath, BigInt(params.proof_index))
    const secondProof = await readProofFromDisk(proofsPath, BigInt(params.proof_index + 1))

    const proof = await proofGenerator.mergeProofs(firstProof.publicInput, firstProof, secondProof)

    return proof
}

export const continueMerge = async (params: ContinueMerge, proofsPath: string) => {
    const firstProof = await readMergedProofFromDisk(proofsPath)
    const secondProof = await readProofFromDisk(proofsPath, BigInt(params.proof_index))

    const proof = await proofGenerator.mergeProofs(firstProof.publicInput, firstProof, secondProof)

    return proof
}
