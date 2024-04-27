import { StateRoots } from "./types.js"

export type Input = MergeWithPrevious | MistakenInput

export type MergeWithPrevious = {
    kind: "MergeWithPrevious"
    state_roots: StateRoots
    proof_index: number
}

export type MistakenInput = {
    kind: "MistakenInput"
}
