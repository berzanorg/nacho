import { describe, it } from "node:test"
import assert from "assert"
import { AccountUpdate, Mina, Poseidon, Signature, UInt64, MerkleTree, Field } from "o1js"
import { BridgeContract } from "../src/bridge-contract.js"
import { Deposit, SingleWithdrawalWitness, WITHDRAWALS_TREE_HEIGHT } from "nacho-common-o1js"
import { RollupContract } from "nacho-rollup-contract"
import { createStateUtil, proofGenerator } from "nacho-proof-generator"
import { generateKeypair } from "./utils.js"
import { TokenContract } from "nacho-token-contract"
import { SafeContract } from "../src/safe-contract.js"

describe("bridge contract", async () => {
    await TokenContract.compile()
    await proofGenerator.compile()
    await RollupContract.compile()
    await BridgeContract.compile()
    await SafeContract.compile()

    const LocalBlockchain = Mina.LocalBlockchain({ proofsEnabled: false })
    Mina.setActiveInstance(LocalBlockchain)

    const usdcTokenContractKeypair = generateKeypair()
    const rollupContractKeypair = generateKeypair()
    const bridgeContractKeypair = generateKeypair()
    const usdcTokenContract = new TokenContract(usdcTokenContractKeypair.publicKey)
    const rollupContract = new RollupContract(rollupContractKeypair.publicKey)
    const bridgeContract = new BridgeContract(bridgeContractKeypair.publicKey)
    const usdcSafeContract = new SafeContract(
        bridgeContractKeypair.publicKey,
        usdcTokenContract.deriveTokenId(),
    )

    const stateUtil = createStateUtil()
    const john = LocalBlockchain.testAccounts[0]
    const depositsList: Array<Deposit> = []
    const withdrawalsTree = new MerkleTree(WITHDRAWALS_TREE_HEIGHT)

    it("deploys mina and usdc token contracts", async () => {
        const tx = await Mina.transaction(john.publicKey, () => {
            usdcTokenContract.deploy()
            AccountUpdate.fundNewAccount(john.publicKey)
        })

        tx.sign([john.privateKey, usdcTokenContractKeypair.privateKey])

        await tx.prove()
        await tx.send()
    })

    it("mints mina and usdc tokens", async () => {
        const tx = await Mina.transaction(john.publicKey, () => {
            usdcTokenContract.mint(UInt64.from(200_000_000), john.publicKey)
            AccountUpdate.fundNewAccount(john.publicKey)
        })

        tx.sign([john.privateKey])

        await tx.prove()
        await tx.send()

        Mina.getBalance(john.publicKey, usdcTokenContract.deriveTokenId()).assertEquals(
            UInt64.from(200_000_000),
        )
    })

    it("deploys rollup contract", async () => {
        const tx = await Mina.transaction(john.publicKey, () => {
            rollupContract.deploy()
            AccountUpdate.fundNewAccount(john.publicKey)
        })

        tx.sign([john.privateKey, rollupContractKeypair.privateKey])

        await tx.prove()
        await tx.send()
    })

    it("deploys bridge contract", async () => {
        const tx = await Mina.transaction(john.publicKey, () => {
            bridgeContract.deploy()
            usdcTokenContract.approveAccountUpdate(bridgeContract.self)
            AccountUpdate.fundNewAccount(john.publicKey)
        })

        tx.sign([john.privateKey, bridgeContractKeypair.privateKey])

        await tx.prove()
        await tx.send()
    })

    it("deploys safe contracts of bridge", async () => {
        const tx = await Mina.transaction(john.publicKey, () => {
            usdcSafeContract.deploy()
            usdcTokenContract.approveAccountUpdate(usdcSafeContract.self)
            AccountUpdate.fundNewAccount(john.publicKey)
        })

        tx.sign([john.privateKey, bridgeContractKeypair.privateKey])

        await tx.prove()
        await tx.send()
    })

    it("initializes rollup contract's public key for bridge contract", async () => {
        const tx = await Mina.transaction(john.publicKey, () => {
            bridgeContract.initRollupContractAddress(rollupContractKeypair.publicKey)
        })

        tx.sign([john.privateKey])

        await tx.prove()
        await tx.send()
    })

    it("generates create genesis proof", async () => {
        const proof = await proofGenerator.createGenesis(stateUtil.stateRoots)

        stateUtil.pushProof(proof)
    })

    it("adds deposits", async () => {
        const tx = await Mina.transaction(john.publicKey, () => {
            bridgeContract.addMinaDeposit(UInt64.from(10_000_000))
        })

        tx.sign([john.privateKey])

        await tx.prove()
        await tx.send()

        Mina.getBalance(john.publicKey).assertEquals(UInt64.from(994_990_000_000))

        Mina.getBalance(bridgeContractKeypair.publicKey).assertEquals(UInt64.from(10_000_000))
    })

    it("adds deposits one more time", async () => {
        const tx = await Mina.transaction(john.publicKey, () => {
            bridgeContract.addDeposit(usdcTokenContractKeypair.publicKey, UInt64.from(100_000_000))
        })

        tx.sign([john.privateKey])

        await tx.prove()
        await tx.send()

        Mina.getBalance(john.publicKey, usdcTokenContract.deriveTokenId()).assertEquals(
            UInt64.from(100_000_000),
        )

        Mina.getBalance(
            bridgeContractKeypair.publicKey,
            usdcTokenContract.deriveTokenId(),
        ).assertEquals(UInt64.from(100_000_000))
    })

    it("applies deposits", async () => {
        const tx = await Mina.transaction(john.publicKey, () => {
            bridgeContract.applyDeposits()
        })

        tx.sign([john.privateKey])

        await tx.prove()
        await tx.send()
    })

    it("emits correct events", async () => {
        const events = await bridgeContract.fetchEvents()
        const deposits = events.map((event) => event.event.data as unknown as Deposit).toReversed()

        assert.deepEqual(
            deposits[0],
            new Deposit({
                depositor: john.publicKey,
                tokenId: Field(1),
                tokenAmount: UInt64.from(10_000_000),
            }),
        )

        assert.deepEqual(
            deposits[1],
            new Deposit({
                depositor: john.publicKey,
                tokenId: usdcTokenContract.deriveTokenId(),
                tokenAmount: UInt64.from(100_000_000),
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
                john.publicKey,
                depositsList[0].tokenId,
                depositsList[0].tokenAmount,
            ),
            john.publicKey,
            depositsList[0].tokenId,
            depositsList[0].tokenAmount,
            UInt64.from(0),
        )

        stateUtil.setBalance(
            0n,
            john.publicKey,
            depositsList[0].tokenId,
            depositsList[0].tokenAmount,
        )
        stateUtil.pushDeposit(john.publicKey, depositsList[0].tokenId, depositsList[0].tokenAmount)

        stateUtil.pushProof(proof)
    })

    it("generates deposit tokens proof one more time", async () => {
        const proof = await proofGenerator.depositTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSingleBalanceWitness(1n),
            stateUtil.currentDepositsRoot,
            stateUtil.getExpectedDepositsRoot(
                john.publicKey,
                depositsList[1].tokenId,
                depositsList[1].tokenAmount,
            ),
            john.publicKey,
            depositsList[1].tokenId,
            depositsList[1].tokenAmount,
            UInt64.from(0),
        )

        stateUtil.setBalance(
            1n,
            john.publicKey,
            depositsList[1].tokenId,
            depositsList[1].tokenAmount,
        )
        stateUtil.pushDeposit(john.publicKey, depositsList[1].tokenId, depositsList[1].tokenAmount)

        stateUtil.pushProof(proof)
    })

    it("generates make burn tokens proof", async () => {
        const currentBalance = UInt64.from(10_000_000)
        const currentBurn = UInt64.from(0)
        const amountToBurn = UInt64.from(10_000_000)
        const userSignature = Signature.create(john.privateKey, [Field(1), amountToBurn.value])

        const proof = await proofGenerator.makeBurnTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSingleBalanceWitness(0n),
            stateUtil.getSingleBurnWitness(0n),
            john.publicKey,
            Field(1),
            currentBurn,
            currentBalance,
            amountToBurn,
            userSignature,
        )

        stateUtil.setBalance(0n, john.publicKey, Field(1), currentBalance.sub(amountToBurn))
        stateUtil.setBurn(0n, john.publicKey, Field(1), amountToBurn)

        stateUtil.pushProof(proof)
    })

    it("generates make burn tokens proof one more time", async () => {
        const currentBalance = UInt64.from(100_000_000)
        const currentBurn = UInt64.from(0)
        const amountToBurn = UInt64.from(100_000_000)
        const userSignature = Signature.create(john.privateKey, [
            usdcTokenContract.deriveTokenId(),
            amountToBurn.value,
        ])

        const proof = await proofGenerator.makeBurnTokens(
            stateUtil.stateRoots,
            stateUtil.lastProof,
            stateUtil.getSingleBalanceWitness(1n),
            stateUtil.getSingleBurnWitness(1n),
            john.publicKey,
            usdcTokenContract.deriveTokenId(),
            currentBurn,
            currentBalance,
            amountToBurn,
            userSignature,
        )

        stateUtil.setBalance(
            1n,
            john.publicKey,
            usdcTokenContract.deriveTokenId(),
            currentBalance.sub(amountToBurn),
        )
        stateUtil.setBurn(1n, john.publicKey, usdcTokenContract.deriveTokenId(), amountToBurn)

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

        const tx = await Mina.transaction(john.publicKey, () => {
            rollupContract.settle(proof)
        })

        tx.sign([john.privateKey])

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

        const tx = await Mina.transaction(john.publicKey, () => {
            bridgeContract.withdrawMina(
                singleWithdrawalWitness,
                stateUtil.getSingleBurnWitness(0n),
                totalWithdrawAmount,
                totalBurnAmount,
            )
        })

        tx.sign([john.privateKey])

        await tx.prove()
        await tx.send()

        withdrawalsTree.setLeaf(
            0n,
            Poseidon.hash([...john.publicKey.toFields(), Field(1), totalBurnAmount.value]),
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

        const tx = await Mina.transaction(john.publicKey, () => {
            bridgeContract.withdrawTokens(
                singleWithdrawalWitness,
                stateUtil.getSingleBurnWitness(1n),
                usdcTokenContract.address,
                totalWithdrawAmount,
                totalBurnAmount,
            )
        })

        tx.sign([john.privateKey])

        await tx.prove()
        await tx.send()

        Mina.getBalance(john.publicKey, usdcTokenContract.deriveTokenId()).assertEquals(
            UInt64.from(200_000_000),
        )

        Mina.getBalance(
            bridgeContractKeypair.publicKey,
            usdcTokenContract.deriveTokenId(),
        ).assertEquals(UInt64.from(0))

        withdrawalsTree.setLeaf(
            1n,
            Poseidon.hash([
                ...john.publicKey.toFields(),
                usdcTokenContract.deriveTokenId(),
                totalBurnAmount.value,
            ]),
        )
    })
})
