import { Bool, Field, PublicKey, SelfProof, Signature, UInt64, ZkProgram } from "o1js"
import {
    DoubleBalanceWitness,
    SingleBalanceWitness,
    SingleBurnWitness,
    SingleLiquidityWitness,
    SinglePoolWitness,
    StateRoots,
} from "nacho-common-o1js"
import { createGenesis } from "./methods/create-genesis.js"
import { mergeProofs } from "./methods/merge-proofs.js"
import { depositTokens } from "./methods/deposit-tokens.js"
import { makeBurnTokens } from "./methods/make-burn-tokens.js"
import { makeBuyTokens } from "./methods/make-buy-tokens.js"
import { makeSellTokens } from "./methods/make-sell-tokens.js"
import { makeProvideLiquidity } from "./methods/make-provide-liquidity.js"
import { makeRemoveLiquidity } from "./methods/make-remove-liquidity.js"
import { makeCreatePool } from "./methods/make-create-pool.js"

export const proofGenerator = ZkProgram({
    name: "proofGenerator",
    publicInput: StateRoots,
    publicOutput: StateRoots,
    methods: {
        createGenesis: {
            privateInputs: [],
            method: createGenesis,
        },
        mergeProofs: {
            privateInputs: [SelfProof<StateRoots, StateRoots>, SelfProof<StateRoots, StateRoots>],
            method: mergeProofs,
        },
        depositTokens: {
            privateInputs: [
                SelfProof<StateRoots, StateRoots>,
                SingleBalanceWitness,
                Field,
                Field,
                PublicKey,
                Field,
                UInt64,
                UInt64,
            ],
            method: depositTokens,
        },
        makeBurnTokens: {
            privateInputs: [
                SelfProof<StateRoots, StateRoots>,
                SingleBalanceWitness,
                SingleBurnWitness,
                PublicKey,
                Field,
                UInt64,
                UInt64,
                UInt64,
                Signature,
            ],
            method: makeBurnTokens,
        },
        makeCreatePool: {
            privateInputs: [
                SelfProof<StateRoots, StateRoots>,
                SinglePoolWitness,
                SingleLiquidityWitness,
                DoubleBalanceWitness,
                Field,
                Field,
                PublicKey,
                UInt64,
                UInt64,
                UInt64,
                UInt64,
                Signature,
            ],
            method: makeCreatePool,
        },
        makeProvideLiquidity: {
            privateInputs: [
                SelfProof<StateRoots, StateRoots>,
                SinglePoolWitness,
                SingleLiquidityWitness,
                DoubleBalanceWitness,
                Field,
                Field,
                PublicKey,
                Field,
                UInt64,
                UInt64,
                UInt64,
                UInt64,
                Field,
                UInt64,
                UInt64,
                Signature,
            ],
            method: makeProvideLiquidity,
        },
        makeRemoveLiquidity: {
            privateInputs: [
                SelfProof<StateRoots, StateRoots>,
                SinglePoolWitness,
                SingleLiquidityWitness,
                DoubleBalanceWitness,
                Field,
                Field,
                PublicKey,
                Field,
                UInt64,
                UInt64,
                UInt64,
                UInt64,
                Field,
                Field,
                UInt64,
                UInt64,
                Signature,
            ],
            method: makeRemoveLiquidity,
        },
        makeBuyTokens: {
            privateInputs: [
                SelfProof<StateRoots, StateRoots>,
                SinglePoolWitness,
                DoubleBalanceWitness,
                PublicKey,
                Field,
                Field,
                UInt64,
                UInt64,
                UInt64,
                UInt64,
                Field,
                UInt64,
                UInt64,
                Signature,
            ],
            method: makeBuyTokens,
        },
        makeSellTokens: {
            privateInputs: [
                SelfProof<StateRoots, StateRoots>,
                SinglePoolWitness,
                DoubleBalanceWitness,
                PublicKey,
                Field,
                Field,
                UInt64,
                UInt64,
                UInt64,
                UInt64,
                Field,
                UInt64,
                UInt64,
                Signature,
            ],
            method: makeSellTokens,
        },
    },
})
