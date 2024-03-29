import { createStateUtil, proofGenerator } from "nacho-proof-generator"
import { describe, it } from "node:test"
import { RollupContract } from "../src/index.js"
import { AccountUpdate, Bool, Field, Mina, UInt64 } from "o1js"
import { generateKeypair } from "./utils.js"
import { StateRoots } from "nacho-common-o1js"
import assert from "assert"

describe("rollup contract", async () => {
    await proofGenerator.compile()
    await RollupContract.compile()

    const LocalBlockchain = Mina.LocalBlockchain({ proofsEnabled: false })
    Mina.setActiveInstance(LocalBlockchain)

    const stateUtil = createStateUtil()
    const john = LocalBlockchain.testAccounts[0]
    const rollupContractKeypair = generateKeypair()
    const rollupContract = new RollupContract(rollupContractKeypair.publicKey)

    it("deploys rollup contract", async () => {
        const tx = await Mina.transaction(john.publicKey, () => {
            rollupContract.deploy()
            AccountUpdate.fundNewAccount(john.publicKey)
        })

        tx.sign([john.privateKey, rollupContractKeypair.privateKey])
        await tx.prove()
        await tx.send()

        rollupContract.stateRoots.getAndRequireEquals().assertEquals(StateRoots.empty())
    })

    it("generates create genesis proof", async () => {
        const proof = await proofGenerator.createGenesis(stateUtil.stateRoots)

        stateUtil.pushProof(proof)
    })

    it("generates deposit tokens proof", async () => {
        const minaTokenId = Field(1)
        const tokenAmount = UInt64.from(42)
        const currentBalance = UInt64.zero
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

        stateUtil.pushProof(proof)
    })

    it("merges proofs", async () => {
        const proof = await proofGenerator.mergeProofs(
            stateUtil.stateRoots,
            stateUtil.proofs[0],
            stateUtil.proofs[1],
        )

        stateUtil.pushProof(proof)
    })

    it("settles proofs to rollup contract", async () => {
        const tx = await Mina.transaction(john.publicKey, () => {
            rollupContract.settle(stateUtil.proofs[2])
        })

        tx.sign([john.privateKey])
        await tx.prove()
        await tx.send()

        stateUtil.setBalance(0n, john.publicKey, Field(0), UInt64.from(42))
        rollupContract.stateRoots.getAndRequireEquals().assertEquals(stateUtil.stateRoots)
    })

    it("doesn't settle old proofs to rollup contract", async () => {
        try {
            const tx = await Mina.transaction(john.publicKey, () => {
                rollupContract.settle(stateUtil.proofs[0])
            })
            tx.sign([john.privateKey])
            await tx.prove()
            await tx.send()
        } catch (error) {
            assert.equal(
                (error as Error).message,
                'Transaction failed with errors:\n- [[],[["Account_app_state_precondition_unsatisfied",0]]]',
            )
        }
    })
})
