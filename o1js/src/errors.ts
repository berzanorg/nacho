export enum ProverError {
    UnmatchedFirstProofPublicInputAndMethodPublicInput = "First proof's public input doesn't match this method's public input.",
    UnmatchedSecondProofPublicInputAndFirstProofPublicOutput = "Second proof's public input doesn't match first proof's public output.",
}

export enum SmartContractError {
    UnmatchedProofPublicInputAndOnChainMerkleRoot = "The proof's public input doesn't match the on-chain Merkle root.",
}
