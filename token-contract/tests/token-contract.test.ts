import { describe, test } from "node:test"
import { AccountUpdate, Mina, UInt64 } from "o1js"
import { TokenContract } from "../src/token-contract.js"

describe("token contract", async () => {
    const LocalBlockchain = await Mina.LocalBlockchain({ proofsEnabled: false })
    Mina.setActiveInstance(LocalBlockchain)

    const [berzan, john] = LocalBlockchain.testAccounts

    const tokenTestPublicKey = Mina.TestPublicKey.random(1)
    const token = new TokenContract(tokenTestPublicKey)

    await TokenContract.compile()

    test("deploys token contract", async () => {
        const deployTx = await Mina.transaction(berzan, async () => {
            await token.deploy()
            AccountUpdate.fundNewAccount(berzan)
        })

        await deployTx.prove()
        await deployTx.sign([berzan.key, tokenTestPublicKey.key]).send()
    })

    test("mints", async () => {
        const mintTx = await Mina.transaction(john, async () => {
            await token.mint(john, UInt64.MAXINT())
            AccountUpdate.fundNewAccount(john)
        })

        await mintTx.prove()
        await mintTx.sign([john.key]).send()

        const user2Balance = Mina.getBalance(john, token.deriveTokenId())
        user2Balance.assertEquals(UInt64.MAXINT())
    })

    test("transfers", async () => {
        const mintTx = await Mina.transaction(john, async () => {
            AccountUpdate.fundNewAccount(john)
            await token.transfer(john, berzan, UInt64.from(1000))
        })

        mintTx.sign([john.key, berzan.key, tokenTestPublicKey.key])
        await mintTx.prove()
        await mintTx.send()

        const user1Balance = Mina.getBalance(berzan, token.deriveTokenId())
        user1Balance.assertEquals(UInt64.from(1000))

        const user2Balance = Mina.getBalance(john, token.deriveTokenId())
        user2Balance.assertEquals(UInt64.MAXINT().sub(1000))
    })
})
