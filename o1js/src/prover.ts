import { Field, Poseidon, SelfProof, UInt64, ZkProgram } from "o1js"
import { ProverError } from "./errors.js"
import { Balance, Burn, Liquidity, Pair } from "./structs.js"
import { Witness } from "./witness.js"

/**
 * The zk program that is used to generate proofs.
 *
 * The public input and output represent the Merkle root of the rollup's state.
 *
 * The merkle root of the rollup's state is constructed from the witness database.
 */
export const prover = ZkProgram({
    name: "prover",
    publicInput: Field,
    publicOutput: Field,
    methods: {
        /**
         * The method that does no opeartion.
         *
         * It returns the public input as the public output without any modifications.
         */
        noOperation: {
            privateInputs: [],
            method(root: Field) {
                return root
            },
        },

        /**
         * The method that merges two proofs.
         *
         * It checks if the first proof's public input matches this method's public input
         * and the second proof's public input matches the first proof's public output,
         * then verifies both of the proofs, lastly returns the second proof's public output.
         */
        mergeProofs: {
            privateInputs: [SelfProof, SelfProof],
            method(
                root: Field,
                firstProof: SelfProof<Field, Field>,
                secondProof: SelfProof<Field, Field>,
            ) {
                firstProof.publicInput.assertEquals(
                    root,
                    ProverError.UnmatchedFirstProofPublicInputAndMethodPublicInput,
                )

                secondProof.publicInput.assertEquals(
                    firstProof.publicOutput,
                    ProverError.UnmatchedSecondProofPublicInputAndFirstProofPublicOutput,
                )

                firstProof.verify()
                secondProof.verify()

                return secondProof.publicOutput
            },
        },

        /**
         * The method that mints the given amount of tokens.
         */
        mint: {
            privateInputs: [SelfProof],
            method(root: Field, earlierProof: SelfProof<Field, Field>) {
                earlierProof.publicInput.assertEquals(
                    root,
                    ProverError.UnmathcedEarlierProofPublicInputAndMethodPublicInput,
                )

                earlierProof.verify()

                // TODO: Implement the logic and calculate the updated root.

                return Field(0)
            },
        },

        /**
         * The method that burns the given amount of tokens from the given balance.
         */
        burn: {
            privateInputs: [SelfProof, Balance, Witness, Burn, Witness, UInt64],
            method(
                root: Field,
                earlierProof: SelfProof<Field, Field>,
                balance: Balance,
                balanceWitness: Witness,
                burn: Burn,
                burnWitness: Witness,
                tokenAmoutToBurn: UInt64,
            ) {
                earlierProof.publicInput.assertEquals(
                    root,
                    ProverError.UnmathcedEarlierProofPublicInputAndMethodPublicInput,
                )

                balanceWitness[0].assertEquals(
                    balance.wdbIndex,
                    ProverError.UnmatchedBalanceWdbIndexAndBalanceWitnessIndex,
                )

                burnWitness[0].assertEquals(
                    burn.wdbIndex,
                    ProverError.UnmatchedBurnWdbIndexAndBurnWitnessIndex,
                )

                balanceWitness
                    .calculateRoot(Poseidon.hash(balance.toFields()))
                    .assertEquals(root, ProverError.UnmatchedBalanceWitnessRootAndMethodPublicInput)

                burnWitness
                    .calculateRoot(Field(0))
                    .assertEquals(root, ProverError.UnmatchedBurnWitnessRootAndMethodPublicInput)

                balance.tokenId.assertEquals(
                    burn.tokenId,
                    ProverError.UnmatchedBalanceTokenIdAndBurnTokenId,
                )

                earlierProof.verify()

                // TODO: Implement the logic and calculate the updated root.

                return Field(0)
            },
        },

        /**
         * The method that creates a pair of two specific tokens using the given liquidity.
         */
        createPair: {
            privateInputs: [
                SelfProof,
                Balance,
                Witness,
                Balance,
                Witness,
                Pair,
                Witness,
                Liquidity,
                Witness,
            ],
            method(
                root: Field,
                earlierProof: SelfProof<Field, Field>,
                balanceBase: Balance,
                balanceBaseWitness: Witness,
                balanceQuote: Balance,
                balanceQuoteWitness: Witness,
                pair: Pair,
                pairWitness: Witness,
                liquidity: Liquidity,
                liquidityWitness: Witness,
            ) {
                earlierProof.publicInput.assertEquals(
                    root,
                    ProverError.UnmathcedEarlierProofPublicInputAndMethodPublicInput,
                )

                balanceBaseWitness[0].assertEquals(
                    balanceBase.wdbIndex,
                    ProverError.UnmatchedBalanceWdbIndexAndBalanceWitnessIndex,
                )

                balanceQuoteWitness[0].assertEquals(
                    balanceQuote.wdbIndex,
                    ProverError.UnmatchedBalanceWdbIndexAndBalanceWitnessIndex,
                )

                pairWitness[0].assertEquals(
                    pair.wdbIndex,
                    ProverError.UnmatchedPairWdbIndexAndPairWitnessIndex,
                )

                liquidityWitness[0].assertEquals(
                    liquidity.wdbIndex,
                    ProverError.UnmatchedLiquidityWdbIndexAndLiquidityWitnessIndex,
                )

                balanceBaseWitness
                    .calculateRoot(Poseidon.hash(balanceBase.toFields()))
                    .assertEquals(root, ProverError.UnmatchedBalanceWitnessRootAndMethodPublicInput)

                balanceQuoteWitness
                    .calculateRoot(Poseidon.hash(balanceQuote.toFields()))
                    .assertEquals(root, ProverError.UnmatchedBalanceWitnessRootAndMethodPublicInput)

                pairWitness
                    .calculateRoot(Field(0))
                    .assertEquals(root, ProverError.UnmatchedPairWitnessRootAndMethodPublicInput)

                liquidityWitness
                    .calculateRoot(Field(0))
                    .assertEquals(
                        root,
                        ProverError.UnmatchedLiquidityWitnessRootAndMethodPublicInput,
                    )

                balanceBase.tokenId.assertEquals(
                    pair.baseTokenId,
                    ProverError.UnmatchedBaseTokenBalanceTokenIdAndPairBaseTokenId,
                )

                balanceQuote.tokenId.assertEquals(
                    pair.quoteTokenId,
                    ProverError.UnmatchedQuoteTokenBalanceTokenIdAndPairQuoteTokenId,
                )

                balanceBase.tokenId.assertEquals(
                    liquidity.baseTokenId,
                    ProverError.UnmatchedBaseTokenBalanceTokenIdAndLiquidityBaseTokenId,
                )

                balanceQuote.tokenId.assertEquals(
                    liquidity.quoteTokenId,
                    ProverError.UnmatchedQuoteTokenBalanceTokenIdAndLiquidityQuoteTokenId,
                )

                earlierProof.verify()

                // TODO: Implement the logic and calculate the updated root.

                return Field(0)
            },
        },

        /**
         * The method that add the given amount of liquidity to the pair of two specific tokens.
         */
        addLiquidity: {
            privateInputs: [
                SelfProof,
                Balance,
                Witness,
                Balance,
                Witness,
                Pair,
                Witness,
                Liquidity,
                Witness,
                UInt64,
                UInt64,
            ],
            method(
                root: Field,
                earlierProof: SelfProof<Field, Field>,
                balanceBase: Balance,
                balanceBaseWitness: Witness,
                balanceQuote: Balance,
                balanceQuoteWitness: Witness,
                pair: Pair,
                pairWitness: Witness,
                liquidity: Liquidity,
                liquidityWitness: Witness,
                baseTokenAmounToAddAsLiquidity: UInt64,
                maxQuoteTokenAmounToAddAsLiquidity: UInt64,
            ) {
                earlierProof.publicInput.assertEquals(
                    root,
                    ProverError.UnmathcedEarlierProofPublicInputAndMethodPublicInput,
                )

                balanceBaseWitness[0].assertEquals(
                    balanceBase.wdbIndex,
                    ProverError.UnmatchedBalanceWdbIndexAndBalanceWitnessIndex,
                )

                balanceQuoteWitness[0].assertEquals(
                    balanceQuote.wdbIndex,
                    ProverError.UnmatchedBalanceWdbIndexAndBalanceWitnessIndex,
                )

                pairWitness[0].assertEquals(
                    pair.wdbIndex,
                    ProverError.UnmatchedPairWdbIndexAndPairWitnessIndex,
                )

                liquidityWitness[0].assertEquals(
                    liquidity.wdbIndex,
                    ProverError.UnmatchedLiquidityWdbIndexAndLiquidityWitnessIndex,
                )

                balanceBaseWitness
                    .calculateRoot(Poseidon.hash(balanceBase.toFields()))
                    .assertEquals(root, ProverError.UnmatchedBalanceWitnessRootAndMethodPublicInput)

                balanceQuoteWitness
                    .calculateRoot(Poseidon.hash(balanceQuote.toFields()))
                    .assertEquals(root, ProverError.UnmatchedBalanceWitnessRootAndMethodPublicInput)

                pairWitness
                    .calculateRoot(Poseidon.hash(pair.toFields()))
                    .assertEquals(root, ProverError.UnmatchedPairWitnessRootAndMethodPublicInput)

                liquidityWitness
                    .calculateRoot(Poseidon.hash(liquidity.toFields()))
                    .assertEquals(
                        root,
                        ProverError.UnmatchedLiquidityWitnessRootAndMethodPublicInput,
                    )

                balanceBase.tokenId.assertEquals(
                    pair.baseTokenId,
                    ProverError.UnmatchedBaseTokenBalanceTokenIdAndPairBaseTokenId,
                )

                balanceQuote.tokenId.assertEquals(
                    pair.quoteTokenId,
                    ProverError.UnmatchedQuoteTokenBalanceTokenIdAndPairQuoteTokenId,
                )

                balanceBase.tokenId.assertEquals(
                    liquidity.baseTokenId,
                    ProverError.UnmatchedBaseTokenBalanceTokenIdAndLiquidityBaseTokenId,
                )

                balanceQuote.tokenId.assertEquals(
                    liquidity.quoteTokenId,
                    ProverError.UnmatchedQuoteTokenBalanceTokenIdAndLiquidityQuoteTokenId,
                )

                earlierProof.verify()

                // TODO: Implement the logic and calculate the updated root.

                return Field(0)
            },
        },

        /**
         * The method that removes the given amount of liquidity from the pair of two specific tokens.
         */
        removeLiquidity: {
            privateInputs: [
                SelfProof,
                Balance,
                Witness,
                Balance,
                Witness,
                Pair,
                Witness,
                Liquidity,
                Witness,
                UInt64,
                UInt64,
                UInt64,
            ],
            method(
                root: Field,
                earlierProof: SelfProof<Field, Field>,
                balanceBase: Balance,
                balanceBaseWitness: Witness,
                balanceQuote: Balance,
                balanceQuoteWitness: Witness,
                pair: Pair,
                pairWitness: Witness,
                liquidity: Liquidity,
                liquidityWitness: Witness,
                lpPointsToRemoveAsLiquidity: UInt64,
                minBaseTokenAmountToRemoveAsLiquidity: UInt64,
                minQuoteTokenAmountToRemoveAsLiquidity: UInt64,
            ) {
                earlierProof.publicInput.assertEquals(
                    root,
                    ProverError.UnmathcedEarlierProofPublicInputAndMethodPublicInput,
                )

                balanceBaseWitness[0].assertEquals(
                    balanceBase.wdbIndex,
                    ProverError.UnmatchedBalanceWdbIndexAndBalanceWitnessIndex,
                )

                balanceQuoteWitness[0].assertEquals(
                    balanceQuote.wdbIndex,
                    ProverError.UnmatchedBalanceWdbIndexAndBalanceWitnessIndex,
                )

                pairWitness[0].assertEquals(
                    pair.wdbIndex,
                    ProverError.UnmatchedPairWdbIndexAndPairWitnessIndex,
                )

                liquidityWitness[0].assertEquals(
                    liquidity.wdbIndex,
                    ProverError.UnmatchedLiquidityWdbIndexAndLiquidityWitnessIndex,
                )

                balanceBaseWitness
                    .calculateRoot(Poseidon.hash(balanceBase.toFields()))
                    .assertEquals(root, ProverError.UnmatchedBalanceWitnessRootAndMethodPublicInput)

                balanceQuoteWitness
                    .calculateRoot(Poseidon.hash(balanceQuote.toFields()))
                    .assertEquals(root, ProverError.UnmatchedBalanceWitnessRootAndMethodPublicInput)

                pairWitness
                    .calculateRoot(Poseidon.hash(pair.toFields()))
                    .assertEquals(root, ProverError.UnmatchedPairWitnessRootAndMethodPublicInput)

                liquidityWitness
                    .calculateRoot(Poseidon.hash(liquidity.toFields()))
                    .assertEquals(
                        root,
                        ProverError.UnmatchedLiquidityWitnessRootAndMethodPublicInput,
                    )

                balanceBase.tokenId.assertEquals(
                    pair.baseTokenId,
                    ProverError.UnmatchedBaseTokenBalanceTokenIdAndPairBaseTokenId,
                )

                balanceQuote.tokenId.assertEquals(
                    pair.quoteTokenId,
                    ProverError.UnmatchedQuoteTokenBalanceTokenIdAndPairQuoteTokenId,
                )

                balanceBase.tokenId.assertEquals(
                    liquidity.baseTokenId,
                    ProverError.UnmatchedBaseTokenBalanceTokenIdAndLiquidityBaseTokenId,
                )

                balanceQuote.tokenId.assertEquals(
                    liquidity.quoteTokenId,
                    ProverError.UnmatchedQuoteTokenBalanceTokenIdAndLiquidityQuoteTokenId,
                )

                earlierProof.verify()

                // TODO: Implement the logic and calculate the updated root.

                return Field(0)
            },
        },

        /**
         * The method that buys the base token and sells the quote token from the pair of two specific tokens.
         */
        buyBase: {
            privateInputs: [
                SelfProof,
                Balance,
                Witness,
                Balance,
                Witness,
                Pair,
                Witness,
                UInt64,
                UInt64,
            ],
            method(
                root: Field,
                earlierProof: SelfProof<Field, Field>,
                balanceBase: Balance,
                balanceBaseWitness: Witness,
                balanceQuote: Balance,
                balanceQuoteWitness: Witness,
                pair: Pair,
                pairWitness: Witness,
                baseTokenAmountToBuy: UInt64,
                maxQuoteTokenAmountToSell: UInt64,
            ) {
                earlierProof.publicInput.assertEquals(
                    root,
                    ProverError.UnmathcedEarlierProofPublicInputAndMethodPublicInput,
                )

                balanceBaseWitness[0].assertEquals(
                    balanceBase.wdbIndex,
                    ProverError.UnmatchedBalanceWdbIndexAndBalanceWitnessIndex,
                )

                balanceQuoteWitness[0].assertEquals(
                    balanceQuote.wdbIndex,
                    ProverError.UnmatchedBalanceWdbIndexAndBalanceWitnessIndex,
                )

                pairWitness[0].assertEquals(
                    pair.wdbIndex,
                    ProverError.UnmatchedPairWdbIndexAndPairWitnessIndex,
                )

                balanceBaseWitness
                    .calculateRoot(Poseidon.hash(balanceBase.toFields()))
                    .assertEquals(root, ProverError.UnmatchedBalanceWitnessRootAndMethodPublicInput)

                balanceQuoteWitness
                    .calculateRoot(Poseidon.hash(balanceQuote.toFields()))
                    .assertEquals(root, ProverError.UnmatchedBalanceWitnessRootAndMethodPublicInput)

                pairWitness
                    .calculateRoot(Poseidon.hash(pair.toFields()))
                    .assertEquals(root, ProverError.UnmatchedPairWitnessRootAndMethodPublicInput)

                balanceBase.tokenId.assertEquals(
                    pair.baseTokenId,
                    ProverError.UnmatchedBaseTokenBalanceTokenIdAndPairBaseTokenId,
                )

                balanceQuote.tokenId.assertEquals(
                    pair.quoteTokenId,
                    ProverError.UnmatchedQuoteTokenBalanceTokenIdAndPairQuoteTokenId,
                )

                earlierProof.verify()

                // TODO: Implement the logic and calculate the updated root.

                return Field(0)
            },
        },

        /**
         * The method that sells the base token and buys the quote token from the pair of two specific tokens.
         */
        sellBase: {
            privateInputs: [
                SelfProof,
                Balance,
                Witness,
                Balance,
                Witness,
                Pair,
                Witness,
                UInt64,
                UInt64,
            ],
            method(
                root: Field,
                earlierProof: SelfProof<Field, Field>,
                balanceBase: Balance,
                balanceBaseWitness: Witness,
                balanceQuote: Balance,
                balanceQuoteWitness: Witness,
                pair: Pair,
                pairWitness: Witness,
                baseTokenAmountToSell: UInt64,
                minQuoteTokenAmountToBuy: UInt64,
            ) {
                earlierProof.publicInput.assertEquals(
                    root,
                    ProverError.UnmathcedEarlierProofPublicInputAndMethodPublicInput,
                )

                balanceBaseWitness[0].assertEquals(
                    balanceBase.wdbIndex,
                    ProverError.UnmatchedBalanceWdbIndexAndBalanceWitnessIndex,
                )

                balanceQuoteWitness[0].assertEquals(
                    balanceQuote.wdbIndex,
                    ProverError.UnmatchedBalanceWdbIndexAndBalanceWitnessIndex,
                )

                pairWitness[0].assertEquals(
                    pair.wdbIndex,
                    ProverError.UnmatchedPairWdbIndexAndPairWitnessIndex,
                )

                balanceBaseWitness
                    .calculateRoot(Poseidon.hash(balanceBase.toFields()))
                    .assertEquals(root, ProverError.UnmatchedBalanceWitnessRootAndMethodPublicInput)

                balanceQuoteWitness
                    .calculateRoot(Poseidon.hash(balanceQuote.toFields()))
                    .assertEquals(root, ProverError.UnmatchedBalanceWitnessRootAndMethodPublicInput)

                pairWitness
                    .calculateRoot(Poseidon.hash(pair.toFields()))
                    .assertEquals(root, ProverError.UnmatchedPairWitnessRootAndMethodPublicInput)

                balanceBase.tokenId.assertEquals(
                    pair.baseTokenId,
                    ProverError.UnmatchedBaseTokenBalanceTokenIdAndPairBaseTokenId,
                )

                balanceQuote.tokenId.assertEquals(
                    pair.quoteTokenId,
                    ProverError.UnmatchedQuoteTokenBalanceTokenIdAndPairQuoteTokenId,
                )

                earlierProof.verify()

                // TODO: Implement the logic and calculate the updated root.

                return Field(0)
            },
        },
    },
})
