import { describe, it } from "node:test"
import { Bool, Field, Signature, UInt64 } from "o1js"
import { addOnePerMilFee, normalDiv } from "nacho-common-o1js"
import { proofGenerator } from "../src/proof-generator.js"
import { createStateUtil, generateKeypair } from "./utils.js"
import assert from "node:assert"

describe("proof generator", async () => {
    await proofGenerator.compile()

    const stateUtil = createStateUtil()
    const john = generateKeypair()
    const minaTokenId = Field(1)
    const usdcTokenId = Field(2)

    it("generates create genesis proof", async () => {
        const proof = await proofGenerator.createGenesis(stateUtil.stateRoots)

        assert.deepEqual(proof.publicInput, stateUtil.stateRoots)
        assert.deepEqual(proof.publicOutput, stateUtil.stateRoots)
        stateUtil.pushProof(proof)
    })

    it("generates deposit tokens proof", async () => {
        const tokenAmount = UInt64.from(45_000_000)
        const currentBalance = UInt64.from(0)
        const isUsersFirstDeposit = Bool(true)

        const proof = await proofGenerator.depositTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSingleBalanceWitness(0n),
            stateUtil.currentDepositsRoot,
            stateUtil.getExpectedDepositsRoot(john.publicKey, minaTokenId, tokenAmount),
            john.publicKey,
            minaTokenId,
            tokenAmount,
            currentBalance,
            isUsersFirstDeposit,
        )

        proof.publicInput.assertEquals(stateUtil.stateRoots)

        stateUtil.setBalance(0n, john.publicKey, minaTokenId, tokenAmount)
        stateUtil.pushDeposit(john.publicKey, minaTokenId, tokenAmount)

        proof.publicOutput.assertEquals(stateUtil.stateRoots)
        stateUtil.pushProof(proof)
    })

    it("generates make burn tokens proof", async () => {
        const burnAmount = UInt64.from(3_000_000)
        const currentBalance = UInt64.from(45_000_000)
        const userSignature = Signature.create(john.privateKey, [minaTokenId, burnAmount.value])

        const proof = await proofGenerator.makeBurnTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSingleBalanceWitness(0n),
            stateUtil.getSingleBurnWitness(0n),
            john.publicKey,
            minaTokenId,
            burnAmount,
            currentBalance,
            userSignature,
        )

        proof.publicInput.assertEquals(stateUtil.stateRoots)

        stateUtil.setBalance(0n, john.publicKey, minaTokenId, currentBalance.sub(burnAmount))
        stateUtil.setBurn(0n, john.publicKey, minaTokenId, burnAmount)

        proof.publicOutput.assertEquals(stateUtil.stateRoots)
        stateUtil.pushProof(proof)
    })

    it("generates deposit tokens proof one more time", async () => {
        const tokenAmount = UInt64.from(100_000_000)
        const currentBalance = UInt64.from(0)
        const isUsersFirstDeposit = Bool(true)

        const proof = await proofGenerator.depositTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSingleBalanceWitness(1n),
            stateUtil.currentDepositsRoot,
            stateUtil.getExpectedDepositsRoot(john.publicKey, usdcTokenId, tokenAmount),
            john.publicKey,
            usdcTokenId,
            tokenAmount,
            currentBalance,
            isUsersFirstDeposit,
        )

        proof.publicInput.assertEquals(stateUtil.stateRoots)

        stateUtil.setBalance(1n, john.publicKey, usdcTokenId, tokenAmount)
        stateUtil.pushDeposit(john.publicKey, usdcTokenId, tokenAmount)

        proof.publicOutput.assertEquals(stateUtil.stateRoots)
        stateUtil.pushProof(proof)
    })

    it("generates make create pool proof", async () => {
        const currentMinaBalance = UInt64.from(42_000_000)
        const currentUsdcBalance = UInt64.from(100_000_000)
        const initialMinaLiquidity = UInt64.from(32_000_000)
        const initialUsdcLiquidity = UInt64.from(50_000_000)
        const userSignature = Signature.create(john.privateKey, [
            minaTokenId,
            usdcTokenId,
            initialMinaLiquidity.value,
            initialUsdcLiquidity.value,
        ])

        const proof = await proofGenerator.makeCreatePool(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSinglePoolWitness(0n),
            stateUtil.getSingleLiqudityWitness(0n),
            stateUtil.getDoubleBalanceWitness(0n, 1n),
            minaTokenId,
            usdcTokenId,
            john.publicKey,
            initialMinaLiquidity,
            initialUsdcLiquidity,
            currentMinaBalance,
            currentUsdcBalance,
            userSignature,
        )

        proof.publicInput.assertEquals(stateUtil.stateRoots)

        stateUtil.setPool(
            0n,
            minaTokenId,
            usdcTokenId,
            initialMinaLiquidity,
            initialUsdcLiquidity,
            initialMinaLiquidity.value.mul(initialUsdcLiquidity.value),
        )

        stateUtil.setLiquidity(
            0n,
            john.publicKey,
            minaTokenId,
            usdcTokenId,
            initialMinaLiquidity.value.mul(initialUsdcLiquidity.value),
        )

        stateUtil.setBalance(
            0n,
            john.publicKey,
            minaTokenId,
            currentMinaBalance.sub(initialMinaLiquidity),
        )
        stateUtil.setBalance(
            1n,
            john.publicKey,
            usdcTokenId,
            currentUsdcBalance.sub(initialUsdcLiquidity),
        )

        proof.publicOutput.assertEquals(stateUtil.stateRoots)
        stateUtil.pushProof(proof)
    })

    it("generates make provide liquidity proof", async () => {
        const currentMinaBalance = UInt64.from(10_000_000)
        const currentUsdcBalance = UInt64.from(50_000_000)
        const currentMinaLiquidity = UInt64.from(32_000_000)
        const currentUsdcLiquidity = UInt64.from(50_000_000)
        const currentLiquidityPoints = currentMinaLiquidity.value.mul(currentUsdcLiquidity.value)
        const baseTokenAmountToProvide = UInt64.from(5_000_000)
        const quoteTokenAmountLimitToProvide = UInt64.from(10_000_000)
        const isFirstProviding = Bool(false)
        const userSignature = Signature.create(john.privateKey, [
            minaTokenId,
            usdcTokenId,
            baseTokenAmountToProvide.value,
            quoteTokenAmountLimitToProvide.value,
        ])

        const proof = await proofGenerator.makeProvideLiquidity(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSinglePoolWitness(0n),
            stateUtil.getSingleLiqudityWitness(0n),
            stateUtil.getDoubleBalanceWitness(0n, 1n),
            minaTokenId,
            usdcTokenId,
            john.publicKey,
            currentLiquidityPoints,
            currentMinaBalance,
            currentUsdcBalance,
            currentMinaLiquidity,
            currentUsdcLiquidity,
            currentLiquidityPoints,
            baseTokenAmountToProvide,
            quoteTokenAmountLimitToProvide,
            isFirstProviding,
            userSignature,
        )

        proof.publicInput.assertEquals(stateUtil.stateRoots)

        const liquidityPointsToBeCreated = normalDiv(
            currentLiquidityPoints.mul(baseTokenAmountToProvide.value),
            currentMinaLiquidity.value,
        )

        const quoteTokenAmountToProvide = UInt64.from(
            normalDiv(
                baseTokenAmountToProvide.value.mul(currentUsdcLiquidity.value),
                currentMinaLiquidity.value,
            ),
        )

        stateUtil.setPool(
            0n,
            minaTokenId,
            usdcTokenId,
            currentMinaLiquidity.add(baseTokenAmountToProvide),
            currentUsdcLiquidity.add(quoteTokenAmountToProvide),
            currentLiquidityPoints.add(liquidityPointsToBeCreated),
        )

        stateUtil.setLiquidity(
            0n,
            john.publicKey,
            minaTokenId,
            usdcTokenId,
            currentLiquidityPoints.add(liquidityPointsToBeCreated),
        )

        stateUtil.setBalance(
            0n,
            john.publicKey,
            minaTokenId,
            currentMinaBalance.sub(baseTokenAmountToProvide),
        )

        stateUtil.setBalance(
            1n,
            john.publicKey,
            usdcTokenId,
            currentUsdcBalance.sub(quoteTokenAmountToProvide),
        )

        proof.publicOutput.assertEquals(stateUtil.stateRoots)
        stateUtil.pushProof(proof)
    })

    it("generates make remove liquidity proof", async () => {
        const currentMinaBalance = UInt64.from(5_000_000)
        const currentUsdcBalance = UInt64.from(42_187_500)
        const currentMinaLiquidity = UInt64.from(37_000_000)
        const currentUsdcLiquidity = UInt64.from(57_812_500n)
        const currentLiquidityPoints = Field(1_850_000_000_000_000n)
        const baseTokenAmountLimitToProvide = UInt64.from(5_000_000)
        const quoteTokenAmountLimitToProvide = UInt64.from(7_812_500)
        const liquidityPointsToRemove = Field(250_000_000_000_000n)
        const userSignature = Signature.create(john.privateKey, [
            minaTokenId,
            usdcTokenId,
            baseTokenAmountLimitToProvide.value,
            quoteTokenAmountLimitToProvide.value,
            liquidityPointsToRemove,
        ])

        const proof = await proofGenerator.makeRemoveLiquidity(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSinglePoolWitness(0n),
            stateUtil.getSingleLiqudityWitness(0n),
            stateUtil.getDoubleBalanceWitness(0n, 1n),
            minaTokenId,
            usdcTokenId,
            john.publicKey,
            currentLiquidityPoints,
            currentMinaBalance,
            currentUsdcBalance,
            currentMinaLiquidity,
            currentUsdcLiquidity,
            currentLiquidityPoints,
            liquidityPointsToRemove,
            baseTokenAmountLimitToProvide,
            quoteTokenAmountLimitToProvide,
            userSignature,
        )

        proof.publicInput.assertEquals(stateUtil.stateRoots)

        const newMinaBalance = UInt64.from(10_000_000)
        const newUsdcBalance = UInt64.from(50_000_000)
        const newMinaLiquidity = UInt64.from(32_000_000)
        const newUsdcLiquidity = UInt64.from(50_000_000)
        const newLiquidityPoints = newMinaLiquidity.value.mul(newUsdcLiquidity.value)

        stateUtil.setPool(
            0n,
            minaTokenId,
            usdcTokenId,
            newMinaLiquidity,
            newUsdcLiquidity,
            newLiquidityPoints,
        )

        stateUtil.setLiquidity(0n, john.publicKey, minaTokenId, usdcTokenId, newLiquidityPoints)

        stateUtil.setBalance(0n, john.publicKey, minaTokenId, newMinaBalance)
        stateUtil.setBalance(1n, john.publicKey, usdcTokenId, newUsdcBalance)

        proof.publicOutput.assertEquals(stateUtil.stateRoots)
        stateUtil.pushProof(proof)
    })

    it("generates make buy tokens proof", async () => {
        const currentMinaBalance = UInt64.from(10_000_000)
        const currentUsdcBalance = UInt64.from(50_000_000)
        const currentMinaLiquidity = UInt64.from(32_000_000)
        const currentUsdcLiquidity = UInt64.from(50_000_000)
        const currentLiquidityPoints = currentMinaLiquidity.value.mul(currentUsdcLiquidity.value)
        const baseTokenAmountToSwap = UInt64.from(1_000_000)
        const quoteTokenAmountLimitToSwap = UInt64.from(2_000_000)
        const userSignature = Signature.create(john.privateKey, [
            minaTokenId,
            usdcTokenId,
            baseTokenAmountToSwap.value,
            quoteTokenAmountLimitToSwap.value,
        ])

        const proof = await proofGenerator.makeBuyTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSinglePoolWitness(0n),
            stateUtil.getDoubleBalanceWitness(0n, 1n),
            john.publicKey,
            minaTokenId,
            usdcTokenId,
            currentMinaBalance,
            currentUsdcBalance,
            currentMinaLiquidity,
            currentUsdcLiquidity,
            currentLiquidityPoints,
            baseTokenAmountToSwap,
            quoteTokenAmountLimitToSwap,
            userSignature,
        )

        proof.publicInput.assertEquals(stateUtil.stateRoots)

        const k = currentMinaLiquidity.value.mul(currentUsdcLiquidity.value)

        const newMinaLiquidity = currentMinaLiquidity.sub(baseTokenAmountToSwap)

        newMinaLiquidity.equals(UInt64.zero).assertFalse()

        const newUsdcLiquidity = UInt64.from(normalDiv(k, newMinaLiquidity.value))

        const usdcAmountToSwap = newUsdcLiquidity.sub(currentUsdcLiquidity)

        const quoteTokenAmountToSwapPlusFee = addOnePerMilFee(usdcAmountToSwap)

        stateUtil.setPool(
            0n,
            minaTokenId,
            usdcTokenId,
            currentMinaLiquidity.sub(baseTokenAmountToSwap),
            currentUsdcLiquidity.add(quoteTokenAmountToSwapPlusFee),
            currentLiquidityPoints,
        )

        stateUtil.setBalance(
            0n,
            john.publicKey,
            minaTokenId,
            currentMinaBalance.add(baseTokenAmountToSwap),
        )

        stateUtil.setBalance(
            1n,
            john.publicKey,
            usdcTokenId,
            currentUsdcBalance.sub(quoteTokenAmountToSwapPlusFee),
        )

        proof.publicOutput.assertEquals(stateUtil.stateRoots)
        stateUtil.pushProof(proof)
    })

    it("generates make sell tokens proof", async () => {
        const currentMinaBalance = UInt64.from(11_000_000)
        const currentUsdcBalance = UInt64.from(48_385_485n)
        const currentMinaLiquidity = UInt64.from(31_000_000)
        const currentUsdcLiquidity = UInt64.from(51_614_515n)
        const currentLiquidityPoints = Field(1600000000000000n)
        const baseTokenAmountLimitToSwap = UInt64.from(1_000_000)
        const quoteTokenAmountToSwap = UInt64.from(1_614_515)
        const userSignature = Signature.create(john.privateKey, [
            minaTokenId,
            usdcTokenId,
            baseTokenAmountLimitToSwap.value,
            quoteTokenAmountToSwap.value,
        ])

        const proof = await proofGenerator.makeSellTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSinglePoolWitness(0n),
            stateUtil.getDoubleBalanceWitness(0n, 1n),
            john.publicKey,
            minaTokenId,
            usdcTokenId,
            currentMinaBalance,
            currentUsdcBalance,
            currentMinaLiquidity,
            currentUsdcLiquidity,
            currentLiquidityPoints,
            baseTokenAmountLimitToSwap,
            quoteTokenAmountToSwap,
            userSignature,
        )

        proof.publicInput.assertEquals(stateUtil.stateRoots)

        const k = currentMinaLiquidity.value.mul(currentUsdcLiquidity.value)

        const newUsdcLiquidity = currentUsdcLiquidity.sub(quoteTokenAmountToSwap)

        const newMinaLiquidity = UInt64.from(normalDiv(k, newUsdcLiquidity.value))

        const baseTokenAmountToSwap = newMinaLiquidity.sub(currentMinaLiquidity)

        const baseTokenAmountToSwapPlusFee = addOnePerMilFee(baseTokenAmountToSwap)

        stateUtil.setPool(
            0n,
            minaTokenId,
            usdcTokenId,
            currentMinaLiquidity.add(baseTokenAmountToSwapPlusFee),
            currentUsdcLiquidity.sub(quoteTokenAmountToSwap),
            currentLiquidityPoints,
        )

        stateUtil.setBalance(
            0n,
            john.publicKey,
            minaTokenId,
            currentMinaBalance.sub(baseTokenAmountToSwapPlusFee),
        )

        stateUtil.setBalance(
            1n,
            john.publicKey,
            usdcTokenId,
            currentUsdcBalance.add(quoteTokenAmountToSwap),
        )

        proof.publicOutput.assertEquals(stateUtil.stateRoots)
        stateUtil.pushProof(proof)
    })
})
