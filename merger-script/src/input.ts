import { StateRoots } from "./types.js"

export type Input = MergeProofs | MistakenInput

export type MergeProofs = {
    kind: "MergeProofs"
    state_roots: StateRoots
    first_proof_index: bigint
    second_proof_index: bigint
}

export type MistakenInput = {
    kind: "MistakenInput"
}
