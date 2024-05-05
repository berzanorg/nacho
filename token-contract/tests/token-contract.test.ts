import { describe, it } from "node:test"
import { AccountUpdate, Mina, UInt64 } from "o1js"
import { TokenContract } from "../src/token-contract.js"

describe("token contract", async () => {
    const LocalBlockchain = await Mina.LocalBlockchain({ proofsEnabled: false })
    Mina.setActiveInstance(LocalBlockchain)

    const [userAccount1, userAccount2] = LocalBlockchain.testAccounts

    const tokenAccount = Mina.TestPublicKey.random()
    const token = new TokenContract(tokenAccount)

    await TokenContract.compile()

    it("deploys token contract", async () => {
        const deployTx = await Mina.transaction(userAccount1, async () => {
            token.deploy()
            AccountUpdate.fundNewAccount(userAccount1)
        })

        await deployTx.prove()
        await deployTx.sign([userAccount1.key, tokenAccount.key]).send()
    })

    it("mints tokens", async () => {
        const mintTx = await Mina.transaction(userAccount2, async () => {
            token.mint(UInt64.MAXINT(), userAccount2)
            AccountUpdate.fundNewAccount(userAccount2)
        })

        await mintTx.prove()
        await mintTx.sign([userAccount2.key]).send()

        const user2Balance = Mina.getBalance(userAccount2, token.deriveTokenId())
        user2Balance.assertEquals(UInt64.MAXINT())
    })

    it("transfers tokens", async () => {
        const mintTx = await Mina.transaction(userAccount2, async () => {
            AccountUpdate.fundNewAccount(userAccount2)
            token.transfer(userAccount2, userAccount1, UInt64.from(1000))
        })

        mintTx.sign([userAccount2.key, userAccount1.key, tokenAccount.key])
        await mintTx.prove()
        await mintTx.send()

        const user1Balance = Mina.getBalance(userAccount1, token.deriveTokenId())
        user1Balance.assertEquals(UInt64.from(1000))

        const user2Balance = Mina.getBalance(userAccount2, token.deriveTokenId())
        user2Balance.assertEquals(UInt64.MAXINT().sub(1000))
    })
})
