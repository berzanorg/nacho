import { describe, it } from "node:test"
import { AccountUpdate, Mina, PrivateKey, UInt64 } from "o1js"
import { TokenContract } from "../src/token-contract.js"

describe("token contract", async () => {
    const LocalBlockchain = Mina.LocalBlockchain({ proofsEnabled: false })
    Mina.setActiveInstance(LocalBlockchain)

    const user1 = LocalBlockchain.testAccounts[0]
    const user2 = LocalBlockchain.testAccounts[1]
    const tokenPrivateKey = PrivateKey.random()
    const tokenContract = new TokenContract(tokenPrivateKey.toPublicKey())

    await TokenContract.compile()

    it("deploys token contract", async () => {
        const deployTx = await Mina.transaction(user1.publicKey, () => {
            tokenContract.deploy()
            AccountUpdate.fundNewAccount(user1.publicKey)
        })

        deployTx.sign([user1.privateKey, tokenPrivateKey])
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
        const mintTx = await Mina.transaction(user2.publicKey, () => {
            tokenContract.transfer(user2.publicKey, user1.publicKey, UInt64.from(1000))
            AccountUpdate.fundNewAccount(user2.publicKey)
        })

        mintTx.sign([user2.privateKey])
        await mintTx.prove()
        await mintTx.send()

        const user1Balance = Mina.getBalance(user1.publicKey, tokenContract.deriveTokenId())
        user1Balance.assertEquals(UInt64.from(1000))

        const user2Balance = Mina.getBalance(user2.publicKey, tokenContract.deriveTokenId())
        user2Balance.assertEquals(UInt64.MAXINT().sub(1000))
    })
})
