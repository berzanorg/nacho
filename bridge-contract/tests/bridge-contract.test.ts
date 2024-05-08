import { describe, it } from "node:test"
import assert from "assert"
import { AccountUpdate, Mina, Poseidon, Signature, UInt64, MerkleTree, Field, Provable } from "o1js"
import { BridgeContract } from "../src/bridge-contract.js"
import { Deposit, SingleWithdrawalWitness, WITHDRAWALS_TREE_HEIGHT } from "nacho-common-o1js"
import { RollupContract } from "nacho-rollup-contract"
import { createStateUtil, proofGenerator } from "nacho-proof-generator"
import { TokenContract } from "nacho-token-contract"
import { SafeContract } from "../src/safe-contract.js"

describe("bridge contract", async () => {
    await TokenContract.compile()
    await proofGenerator.compile()
    await RollupContract.compile()
    await SafeContract.compile()
    await BridgeContract.compile()

    const LocalBlockchain = await Mina.LocalBlockchain({ proofsEnabled: false })
    Mina.setActiveInstance(LocalBlockchain)

    const [usdcTokenTestPublicKey, rollupTestPublicKey, bridgeTestPublicKey] =
        Mina.TestPublicKey.random(3)

    const usdcTokenContract = new TokenContract(usdcTokenTestPublicKey)
    const rollupContract = new RollupContract(rollupTestPublicKey)
    const bridgeContract = new BridgeContract(bridgeTestPublicKey)
    const usdcSafeContract = new SafeContract(
        bridgeTestPublicKey,
        usdcTokenContract.deriveTokenId(),
    )

    const stateUtil = createStateUtil()
    const john = LocalBlockchain.testAccounts[0]
    const depositsList: Array<Deposit> = []
    const withdrawalsTree = new MerkleTree(WITHDRAWALS_TREE_HEIGHT)

    it("deploys usdc token contract", async () => {
        const tx = await Mina.transaction(john, async () => {
            await usdcTokenContract.deploy()
            AccountUpdate.fundNewAccount(john)
        })

        tx.sign([john.key, usdcTokenTestPublicKey.key])

        await tx.prove()
        await tx.send()
    })

    it("mints usdc tokens", async () => {
        const tx = await Mina.transaction(john, async () => {
            await usdcTokenContract.mint(john, UInt64.from(200_000_000))
            AccountUpdate.fundNewAccount(john)
        })

        tx.sign([john.key])

        await tx.prove()
        await tx.send()

        Mina.getBalance(john, usdcTokenContract.deriveTokenId()).assertEquals(
            UInt64.from(200_000_000),
        )
    })

    it("deploys rollup contract", async () => {
        const tx = await Mina.transaction(john, async () => {
            await rollupContract.deploy()
            AccountUpdate.fundNewAccount(john)
        })

        tx.sign([john.key, rollupTestPublicKey.key])

        await tx.prove()
        await tx.send()
    })

    it("deploys bridge contract", async () => {
        const tx = await Mina.transaction(john, async () => {
            await bridgeContract.deploy()
            await usdcTokenContract.approveAccountUpdate(bridgeContract.self)
            AccountUpdate.fundNewAccount(john)
        })

        tx.sign([john.key, bridgeTestPublicKey.key])

        await tx.prove()
        await tx.send()
    })

    it("deploys safe contracts of bridge", async () => {
        const tx = await Mina.transaction(john, async () => {
            await usdcSafeContract.deploy()
            await usdcTokenContract.approveAccountUpdate(usdcSafeContract.self)
            AccountUpdate.fundNewAccount(john)
        })

        tx.sign([john.key, bridgeTestPublicKey.key])

        await tx.prove()
        await tx.send()
    })

    it("initializes rollup contract's public key for bridge contract", async () => {
        const tx = await Mina.transaction(john, async () => {
            await bridgeContract.initRollupContractAddress(rollupTestPublicKey)
        })

        tx.sign([john.key])

        await tx.prove()
        await tx.send()
    })

    it("generates create genesis proof", async () => {
        const proof = await proofGenerator.createGenesis(stateUtil.stateRoots)

        stateUtil.pushProof(proof)
    })

    it("adds deposits", async () => {
        const tx = await Mina.transaction(john, async () => {
            await bridgeContract.addMinaDeposit(UInt64.from(10_000_000))
        })

        tx.sign([john.key])

        await tx.prove()
        await tx.send()

        Mina.getBalance(john).assertEquals(UInt64.from(994_990_000_000))

        Mina.getBalance(bridgeTestPublicKey).assertEquals(UInt64.from(10_000_000))
    })

    it("adds deposits one more time", async () => {
        const tx = await Mina.transaction(john, async () => {
            await bridgeContract.addDeposit(usdcTokenTestPublicKey, UInt64.from(100_000_000))
        })

        tx.sign([john.key])

        await tx.prove()
        await tx.send()

        Mina.getBalance(john, usdcTokenContract.deriveTokenId()).assertEquals(
            UInt64.from(100_000_000),
        )

        Mina.getBalance(bridgeTestPublicKey, usdcTokenContract.deriveTokenId()).assertEquals(
            UInt64.from(100_000_000),
        )
    })

    it("applies deposits", async () => {
        const tx = await Mina.transaction(john, async () => {
            await bridgeContract.applyDeposits()
        })

        tx.sign([john.key])

        await tx.prove()
        await tx.send()
    })

    it("emits correct events", async () => {
        const events = await bridgeContract.fetchEvents()
        const deposits = events.map((event) => event.event.data as unknown as Deposit).toReversed()

        const minaDepositIndex = deposits.findIndex((deposit) => deposit.tokenId.equals(1))
        const usdcDepositIndex = minaDepositIndex === 0 ? 1 : 0

        assert.deepEqual(
            deposits[usdcDepositIndex],
            new Deposit({
                depositor: john.key.toPublicKey(),
                tokenId: usdcTokenContract.deriveTokenId(),
                tokenAmount: UInt64.from(100_000_000),
            }),
        )

        assert.deepEqual(
            deposits[minaDepositIndex],
            new Deposit({
                depositor: john.key.toPublicKey(),
                tokenId: Field(1),
                tokenAmount: UInt64.from(10_000_000),
            }),
        )

        depositsList.push(...deposits)
    })

    it("generates deposit tokens proof", async () => {
        const proof = await proofGenerator.depositTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSingleBalanceWitness(0n),
            stateUtil.currentDepositsRoot,
            stateUtil.getExpectedDepositsRoot(
                john,
                depositsList[0].tokenId,
                depositsList[0].tokenAmount,
            ),
            john,
            depositsList[0].tokenId,
            depositsList[0].tokenAmount,
            UInt64.from(0),
        )

        stateUtil.setBalance(0n, john, depositsList[0].tokenId, depositsList[0].tokenAmount)
        stateUtil.pushDeposit(john, depositsList[0].tokenId, depositsList[0].tokenAmount)

        stateUtil.pushProof(proof)
    })

    it("generates deposit tokens proof one more time", async () => {
        const proof = await proofGenerator.depositTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSingleBalanceWitness(1n),
            stateUtil.currentDepositsRoot,
            stateUtil.getExpectedDepositsRoot(
                john,
                depositsList[1].tokenId,
                depositsList[1].tokenAmount,
            ),
            john,
            depositsList[1].tokenId,
            depositsList[1].tokenAmount,
            UInt64.from(0),
        )

        stateUtil.setBalance(1n, john, depositsList[1].tokenId, depositsList[1].tokenAmount)
        stateUtil.pushDeposit(john, depositsList[1].tokenId, depositsList[1].tokenAmount)

        stateUtil.pushProof(proof)
    })

    it("generates make burn tokens proof", async () => {
        const currentBalance = UInt64.from(10_000_000)
        const currentBurn = UInt64.from(0)
        const amountToBurn = UInt64.from(10_000_000)
        const userSignature = Signature.create(john.key, [Field(1), amountToBurn.value])

        const proof = await proofGenerator.makeBurnTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSingleBalanceWitness(0n),
            stateUtil.getSingleBurnWitness(0n),
            john,
            Field(1),
            currentBurn,
            currentBalance,
            amountToBurn,
            userSignature,
        )

        stateUtil.setBalance(0n, john, Field(1), currentBalance.sub(amountToBurn))
        stateUtil.setBurn(0n, john, Field(1), amountToBurn)

        stateUtil.pushProof(proof)
    })

    it("generates make burn tokens proof one more time", async () => {
        const currentBalance = UInt64.from(100_000_000)
        const currentBurn = UInt64.from(0)
        const amountToBurn = UInt64.from(100_000_000)
        const userSignature = Signature.create(john.key, [
            usdcTokenContract.deriveTokenId(),
            amountToBurn.value,
        ])

        const proof = await proofGenerator.makeBurnTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSingleBalanceWitness(1n),
            stateUtil.getSingleBurnWitness(1n),
            john,
            usdcTokenContract.deriveTokenId(),
            currentBurn,
            currentBalance,
            amountToBurn,
            userSignature,
        )

        stateUtil.setBalance(
            1n,
            john,
            usdcTokenContract.deriveTokenId(),
            currentBalance.sub(amountToBurn),
        )
        stateUtil.setBurn(1n, john, usdcTokenContract.deriveTokenId(), amountToBurn)

        stateUtil.pushProof(proof)
    })

    it("merges proofs and settles it to rollup contract", async () => {
        let proof = await proofGenerator.mergeProofs(
            stateUtil.proofs[0].publicInput,
            stateUtil.proofs[0],
            stateUtil.proofs[1],
        )

        proof = await proofGenerator.mergeProofs(
            stateUtil.proofs[0].publicInput,
            proof,
            stateUtil.proofs[2],
        )

        proof = await proofGenerator.mergeProofs(
            stateUtil.proofs[0].publicInput,
            proof,
            stateUtil.proofs[3],
        )

        proof = await proofGenerator.mergeProofs(
            stateUtil.proofs[0].publicInput,
            proof,
            stateUtil.proofs[4],
        )

        const tx = await Mina.transaction(john, async () => {
            await rollupContract.settle(proof)
        })

        tx.sign([john.key])

        await tx.prove()
        await tx.send()
    })

    it("withdraws mina", async () => {
        const totalWithdrawAmount = UInt64.from(0)
        const totalBurnAmount = UInt64.from(10_000_000)
        const singleWithdrawalWitness = new SingleWithdrawalWitness(
            withdrawalsTree
                .getWitness(0n)
                .map((a) => ({ isLeft: !a.isLeft, value: a.sibling.toBigInt() })),
        )

        const tx = await Mina.transaction(john, async () => {
            await bridgeContract.withdrawMina(
                singleWithdrawalWitness,
                stateUtil.getSingleBurnWitness(0n),
                totalWithdrawAmount,
                totalBurnAmount,
            )
        })

        tx.sign([john.key])

        await tx.prove()
        await tx.send()

        withdrawalsTree.setLeaf(
            0n,
            Poseidon.hash([...john.toFields(), Field(1), totalBurnAmount.value]),
        )
    })

    it("withdraws usdc", async () => {
        const totalWithdrawAmount = UInt64.from(0)
        const totalBurnAmount = UInt64.from(100_000_000)
        const singleWithdrawalWitness = new SingleWithdrawalWitness(
            withdrawalsTree
                .getWitness(1n)
                .map((a) => ({ isLeft: !a.isLeft, value: a.sibling.toBigInt() })),
        )

        const tx = await Mina.transaction(john, async () => {
            await bridgeContract.withdrawTokens(
                singleWithdrawalWitness,
                stateUtil.getSingleBurnWitness(1n),
                usdcTokenContract.address,
                totalWithdrawAmount,
                totalBurnAmount,
            )
        })

        tx.sign([john.key])

        await tx.prove()
        await tx.send()

        Mina.getBalance(john, usdcTokenContract.deriveTokenId()).assertEquals(
            UInt64.from(200_000_000),
        )

        Mina.getBalance(bridgeTestPublicKey, usdcTokenContract.deriveTokenId()).assertEquals(
            UInt64.from(0),
        )

        withdrawalsTree.setLeaf(
            1n,
            Poseidon.hash([
                ...john.toFields(),
                usdcTokenContract.deriveTokenId(),
                totalBurnAmount.value,
            ]),
        )
    })
})
