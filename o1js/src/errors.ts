export enum ProverError {
    UnmatchedFirstProofPublicInputAndMethodPublicInput = "First proof's public input doesn't match method's public input.",
    UnmatchedSecondProofPublicInputAndFirstProofPublicOutput = "Second proof's public input doesn't match first proof's public output.",
    UnmathcedEarlierProofPublicInputAndMethodPublicInput = "Earlier proof's public input doesn't match method's public input.",
    UnmatchedBalanceWdbIndexAndBalanceWitnessIndex = "Balance's witness DB index doesn't match balance's witness index.",
    UnmatchedBurnWdbIndexAndBurnWitnessIndex = "Burn's witness DB index doesn't match burn's witness index.",
    UnmatchedPairWdbIndexAndPairWitnessIndex = "Pair's witness DB index doesn't match pair's witness index.",
    UnmatchedLiquidityWdbIndexAndLiquidityWitnessIndex = "Liquidity's witness DB index doesn't match liquidity's witness index.",
    UnmatchedBalanceWitnessRootAndMethodPublicInput = "Calculated balance witness root doesn't match method's public input.",
    UnmatchedBurnWitnessRootAndMethodPublicInput = "Calculated burn witness root doesn't match method's public input.",
    UnmatchedPairWitnessRootAndMethodPublicInput = "Calculated pair witness root doesn't match method's public input.",
    UnmatchedLiquidityWitnessRootAndMethodPublicInput = "Calculated liquidity witness root doesn't match method's public input.",
    UnmatchedBaseTokenBalanceTokenIdAndPairBaseTokenId = "Base token balance's token id doesn't match pair's base token id.",
    UnmatchedQuoteTokenBalanceTokenIdAndPairQuoteTokenId = "Quote token balance's token id doesn't match pair's quote token id.",
    UnmatchedBaseTokenBalanceTokenIdAndLiquidityBaseTokenId = "Base token balance's token id doesn't match liquidity's base token id.",
    UnmatchedQuoteTokenBalanceTokenIdAndLiquidityQuoteTokenId = "Quote token balance's token id doesn't match liquidity's quote token id.",
    UnmatchedBalanceTokenIdAndBurnTokenId = "Balance's token id doesn't match burn's token id.",
}

export enum SmartContractError {
    UnmatchedProofPublicInputAndOnChainMerkleRoot = "The proof's public input doesn't match the on-chain Merkle root.",
}
