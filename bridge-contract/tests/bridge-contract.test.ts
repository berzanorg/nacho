import { describe, it } from "node:test"
import assert from "assert"
import { AccountUpdate, Field, MerkleTree, Mina, Poseidon, PrivateKey, UInt64 } from "o1js"
import { BridgeContract } from "../src/bridge-contract.js"
import { TokenContract } from "token-contract"
import { DEPOSITS_TREE_HEIGHT, Deposit, SingleDepositWitness } from "common-o1js"

describe("bridge contract", async () => {
    const LocalBlockchain = Mina.LocalBlockchain({ proofsEnabled: false })
    Mina.setActiveInstance(LocalBlockchain)

    const user1 = LocalBlockchain.testAccounts[0]
    const user2 = LocalBlockchain.testAccounts[1]
    const bridgePrivateKey = PrivateKey.random()
    const tokenPrivateKey = PrivateKey.random()
    const bridgeContract = new BridgeContract(bridgePrivateKey.toPublicKey())
    const tokenContract = new TokenContract(tokenPrivateKey.toPublicKey())

    const depositsList: Array<Deposit> = []

    await BridgeContract.compile()
    await TokenContract.compile()

    it("deploys bridge and token contracts", async () => {
        const deployTx = await Mina.transaction(user1.publicKey, () => {
            bridgeContract.deploy()
            tokenContract.deploy()
            AccountUpdate.fundNewAccount(user1.publicKey)
            AccountUpdate.fundNewAccount(user1.publicKey)
        })

        deployTx.sign([user1.privateKey, bridgePrivateKey, tokenPrivateKey])
        await deployTx.prove()
        await deployTx.send()
    })

    it("mints tokens", async () => {
        const mintTx = await Mina.transaction(user2.publicKey, () => {
            tokenContract.mint(UInt64.MAXINT(), user2.publicKey)
            AccountUpdate.fundNewAccount(user2.publicKey)
        })

        mintTx.sign([user2.privateKey])
        await mintTx.prove()
        await mintTx.send()

        const user2Balance = Mina.getBalance(user2.publicKey, tokenContract.deriveTokenId())
        user2Balance.assertEquals(UInt64.MAXINT())
    })

    it("transfers tokens", async () => {
        const transferTx = await Mina.transaction(user2.publicKey, () => {
            tokenContract.transfer(user2.publicKey, user1.publicKey, UInt64.from(1000))
            AccountUpdate.fundNewAccount(user2.publicKey)
        })

        transferTx.sign([user2.privateKey])
        await transferTx.prove()
        await transferTx.send()

        const user1Balance = Mina.getBalance(user1.publicKey, tokenContract.deriveTokenId())
        user1Balance.assertEquals(UInt64.from(1000))

        const user2Balance = Mina.getBalance(user2.publicKey, tokenContract.deriveTokenId())
        user2Balance.assertEquals(UInt64.MAXINT().sub(1000))
    })

    it("deposits tokens to bridge", async () => {
        const depositTx = await Mina.transaction(user1.publicKey, () => {
            bridgeContract.depositTokens(tokenContract.address, UInt64.from(1000))
            AccountUpdate.fundNewAccount(user1.publicKey)
        })

        depositTx.sign([user1.privateKey])
        await depositTx.prove()
        await depositTx.send()

        const user1Balance = Mina.getBalance(user1.publicKey, tokenContract.deriveTokenId())
        user1Balance.assertEquals(UInt64.from(0))

        const bridgeBalance = Mina.getBalance(
            bridgePrivateKey.toPublicKey(),
            tokenContract.deriveTokenId(),
        )
        bridgeBalance.assertEquals(UInt64.from(1000))
    })

    it("emits events", async () => {
        const events = await bridgeContract.fetchEvents()

        assert.deepEqual(events.length, 1)
        assert.deepEqual(
            events[0].event.data,
            new Deposit({
                depositor: user1.publicKey,
                tokenId: tokenContract.deriveTokenId(),
                tokenAmount: UInt64.from(1000),
            }),
        )

        depositsList.push(events[0].event.data as unknown as Deposit)
        const hash = Poseidon.hash(depositsList[0].toFields())
        console.log(hash.toBigInt(), "hash")
        console.log(Poseidon.hash([Field(0), hash]).toBigInt())
    })

    it("rolls actions up", async () => {
        const rollTx = await Mina.transaction(user2.publicKey, () => {
            bridgeContract.rollActions()
        })
        rollTx.sign([user2.privateKey])
        await rollTx.prove()
        await rollTx.send()
    })

    it("updates deposits list root correctly", async () => {
        let root = Field(0)

        for (const deposit of depositsList) {
            const hash = Poseidon.hash(deposit.toFields())
            root = Poseidon.hash([root, hash])
        }

        bridgeContract.depositsMerkleListRoot.requireEquals(root)
    })

    it("deposits tokens to bridge", async () => {
        const depositTx = await Mina.transaction(user2.publicKey, () => {
            bridgeContract.depositTokens(tokenContract.address, UInt64.from(2000))
        })

        depositTx.sign([user2.privateKey])
        await depositTx.prove()
        await depositTx.send()

        const user2Balance = Mina.getBalance(user2.publicKey, tokenContract.deriveTokenId())
        user2Balance.assertEquals(UInt64.MAXINT().sub(3000))

        const bridgeBalance = Mina.getBalance(
            bridgePrivateKey.toPublicKey(),
            tokenContract.deriveTokenId(),
        )
        bridgeBalance.assertEquals(UInt64.from(3000))
    })

    it("deposits tokens to bridge", async () => {
        const depositTx = await Mina.transaction(user2.publicKey, () => {
            bridgeContract.depositTokens(tokenContract.address, UInt64.from(3000))
        })

        depositTx.sign([user2.privateKey])
        await depositTx.prove()
        await depositTx.send()

        const user2Balance = Mina.getBalance(user2.publicKey, tokenContract.deriveTokenId())
        user2Balance.assertEquals(UInt64.MAXINT().sub(6000))

        const bridgeBalance = Mina.getBalance(
            bridgePrivateKey.toPublicKey(),
            tokenContract.deriveTokenId(),
        )
        bridgeBalance.assertEquals(UInt64.from(6000))
    })

    it("rolls actions up", async () => {
        const rollTx = await Mina.transaction(user2.publicKey, () => {
            bridgeContract.rollActions()
        })
        rollTx.sign([user2.privateKey])
        await rollTx.prove()
        await rollTx.send()
    })

    it("updates deposits list root correctly", async () => {
        let root = Field(0)

        const deposits = (await bridgeContract.fetchEvents())
            .map((a) => a.event.data as unknown as Deposit)
            .reverse()

        for (const deposit of deposits) {
            console.log(JSON.stringify(deposit))
            const hash = Poseidon.hash(deposit.toFields())
            root = Poseidon.hash([root, hash])
        }

        console.log(root.toBigInt())
        console.log(bridgeContract.depositsMerkleListRoot.get().toBigInt())
        bridgeContract.depositsMerkleListRoot.requireEquals(root)
    })
})
