export type Input = FetchDepositedEvents | FetchWithdrawnEvents | MistakenInput

export type FetchDepositedEvents = {
    kind: "FetchDepositedEvents"
    fromBlock: number
}
export type FetchWithdrawnEvents = {
    kind: "FetchWithdrawnEvents"
    fromBlock: number
}

export type MistakenInput = {
    kind: "MistakenInput"
}
