export type Input = StartMerge | ContinueMerge | MistakenInput

export type StartMerge = {
    kind: "StartMerge"
    proof_index: number
}

export type ContinueMerge = {
    kind: "ContinueMerge"
    proof_index: number
}

export type MistakenInput = {
    kind: "MistakenInput"
}
