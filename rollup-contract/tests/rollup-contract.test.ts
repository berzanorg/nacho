import { createStateUtil, proofGenerator } from "nacho-proof-generator"
import { describe, test } from "node:test"
import { RollupContract } from "../src/index.js"
import { AccountUpdate, Field, Mina, UInt64 } from "o1js"
import { StateRoots } from "nacho-common-o1js"
import assert from "assert"

describe("rollup contract", async () => {
    await proofGenerator.compile()
    await RollupContract.compile()

    const LocalBlockchain = await Mina.LocalBlockchain({ proofsEnabled: false })
    Mina.setActiveInstance(LocalBlockchain)

    const stateUtil = createStateUtil()
    const [john] = LocalBlockchain.testAccounts
    const rollupTestPublicKey = Mina.TestPublicKey.random(1)
    const rollup = new RollupContract(rollupTestPublicKey)

    test("deploys rollup contract", async () => {
        const tx = await Mina.transaction(john, async () => {
            await rollup.deploy()
            AccountUpdate.fundNewAccount(john)
        })

        tx.sign([john.key, rollupTestPublicKey.key])
        await tx.prove()
        await tx.send()

        rollup.stateRoots.get().assertEquals(StateRoots.empty())
    })

    test("generates create genesis proof", async () => {
        const proof = await proofGenerator.createGenesis(stateUtil.stateRoots)

        stateUtil.pushProof(proof)
    })

    test("generates deposit tokens proof", async () => {
        const minaTokenId = Field(1)
        const tokenAmount = UInt64.from(42)
        const currentBalance = UInt64.zero

        const proof = await proofGenerator.depositTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSingleBalanceWitness(0n),
            stateUtil.currentDepositsRoot,
            stateUtil.getExpectedDepositsRoot(john, minaTokenId, tokenAmount),
            john,
            minaTokenId,
            tokenAmount,
            currentBalance,
        )

        stateUtil.pushProof(proof)
    })

    test("merges proofs", async () => {
        const proof = await proofGenerator.mergeProofs(
            stateUtil.stateRoots,
            stateUtil.proofs[0],
            stateUtil.proofs[1],
        )

        stateUtil.pushProof(proof)
    })

    test("settles proofs to rollup contract", async () => {
        const tx = await Mina.transaction(john, async () => {
            await rollup.settle(stateUtil.proofs[2])
        })

        tx.sign([john.key])
        await tx.prove()
        await tx.send()

        stateUtil.setBalance(0n, john, Field(1), UInt64.from(42))
        rollup.stateRoots.get().assertEquals(stateUtil.stateRoots)
    })

    test("doesn't settle old proofs to rollup contract", async () => {
        try {
            const tx = await Mina.transaction(john, async () => {
                await rollup.settle(stateUtil.proofs[0])
            })
            tx.sign([john.key])
            await tx.prove()
            await tx.send()
        } catch (error) {
            assert.equal((error as Error).message, "Bool.assertTrue(): false != true")
        }
    })
})
