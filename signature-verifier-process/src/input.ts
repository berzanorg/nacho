export type BurnTokens = {
    kind: "BurnTokens"
    address: string
    signature: [bigint, bigint]
    token_id: bigint
    token_amount: bigint
}

export type CreatePool = {
    kind: "CreatePool"
    address: string
    signature: [bigint, bigint]
    base_token_id: bigint
    quote_token_id: bigint
    base_token_amount: bigint
    quote_token_amount: bigint
}

export type ProvideLiquidity = {
    kind: "ProvideLiquidity"
    address: string
    signature: [bigint, bigint]
    base_token_id: bigint
    quote_token_id: bigint
    base_token_amount: bigint
    quote_token_amount_limit: bigint
}

export type RemoveLiquidity = {
    kind: "RemoveLiquidity"
    address: string
    signature: [bigint, bigint]
    base_token_id: bigint
    quote_token_id: bigint
    base_token_amount_limit: bigint
    quote_token_amount_limit: bigint
    liquidity_point_amount: bigint
}

export type BuyTokens = {
    kind: "BuyTokens"
    address: string
    signature: [bigint, bigint]
    base_token_id: bigint
    quote_token_id: bigint
    base_token_amount: bigint
    quote_token_amount_limit: bigint
}

export type SellTokens = {
    kind: "SellTokens"
    address: string
    signature: [bigint, bigint]
    base_token_id: bigint
    quote_token_id: bigint
    base_token_amount_limit: bigint
    quote_token_amount: bigint
}

export type MistakenInput = {
    kind: "MistakenInput"
}

export type Input =
    | BurnTokens
    | CreatePool
    | ProvideLiquidity
    | RemoveLiquidity
    | BuyTokens
    | SellTokens
    | MistakenInput
