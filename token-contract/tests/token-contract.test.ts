import { describe, test } from "node:test"
import { AccountUpdate, Mina, PrivateKey, UInt64 } from "o1js"
import { TokenContract } from "../src/token-contract.js"

describe("token contract", async () => {
    const LocalBlockchain = Mina.LocalBlockchain({ proofsEnabled: false })
    Mina.setActiveInstance(LocalBlockchain)

    const [berzan, john] = LocalBlockchain.testAccounts

    const tokenKeys = (() => {
        const privateKey = PrivateKey.random()

        return {
            publicKey: privateKey.toPublicKey(),
            privateKey,
        }
    })()
    const token = new TokenContract(tokenKeys.publicKey)

    await TokenContract.compile()

    test("deploys token contract", async () => {
        const deployTx = await Mina.transaction(berzan.publicKey, async () => {
            token.deploy()
            AccountUpdate.fundNewAccount(berzan.publicKey)
        })

        await deployTx.prove()
        await deployTx.sign([berzan.privateKey, tokenKeys.privateKey]).send()
    })

    test("mints tokens", async () => {
        const mintTx = await Mina.transaction(john.publicKey, async () => {
            token.mint(john.publicKey, UInt64.MAXINT())
            AccountUpdate.fundNewAccount(john.publicKey)
        })

        await mintTx.prove()
        await mintTx.sign([john.privateKey]).send()

        const user2Balance = Mina.getBalance(john.publicKey, token.deriveTokenId())
        user2Balance.assertEquals(UInt64.MAXINT())
    })

    test("transfers tokens", async () => {
        const mintTx = await Mina.transaction(john.publicKey, async () => {
            AccountUpdate.fundNewAccount(john.publicKey)
            token.transfer(john.publicKey, berzan.publicKey, UInt64.from(1000))
        })

        mintTx.sign([john.privateKey, berzan.privateKey, tokenKeys.privateKey])
        await mintTx.prove()
        await mintTx.send()

        const user1Balance = Mina.getBalance(berzan.publicKey, token.deriveTokenId())
        user1Balance.assertEquals(UInt64.from(1000))

        const user2Balance = Mina.getBalance(john.publicKey, token.deriveTokenId())
        user2Balance.assertEquals(UInt64.MAXINT().sub(1000))
    })
})
