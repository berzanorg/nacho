export type Deposit = {
    depositor: string
    tokenId: bigint
    tokenAmount: bigint
}

export type Withdraw = {
    withdrawer: string
    tokenId: bigint
    tokenAmount: bigint
}
