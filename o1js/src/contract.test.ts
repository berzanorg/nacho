import { AccountUpdate, Encoding, Field, Mina, PrivateKey, PublicKey } from "o1js"
import assert from "node:assert"
import { describe, it } from "node:test"
import { prover } from "./prover.js"
import { Proof } from "./proof.js"
import { Contract } from "./contract.js"
import { SmartContractError } from "./errors.js"

describe("Proof generation test", async () => {
    let user = null as unknown as { publicKey: PublicKey; privateKey: PrivateKey }
    let contract: Contract = null as unknown as Contract
    const proofs: Array<Proof> = []

    it("setups local blockchain and test account", () => {
        const localBlockchain = Mina.LocalBlockchain({ proofsEnabled: false })
        Mina.setActiveInstance(localBlockchain)
        user = localBlockchain.testAccounts[0]
    })

    it("compiles the zk program", async () => {
        await prover.compile()
    })

    it("compiles the smart contract", async () => {
        await Contract.compile()
    })

    it("generates a proof", async () => {
        const proof = await prover.noOperation(
            Field("0x1b2d2ee01d57dcf55e8f31af6842433f17d768c262ca1d8c3dd9b6c97f41b369"),
        )
        proofs.push(proof)
    })

    it("generates another proof", async () => {
        const proof = await prover.noOperation(
            Field("0x999999999999999999999999999999999999999999"),
        )
        proofs.push(proof)
    })

    it("deploys the smart contract", async () => {
        const [seed] = Encoding.stringToFields("nacho")

        const contractPrivateKey = PrivateKey.fromFields(seed.toBits().map((f) => f.toField()))
        const contractPublicKey = contractPrivateKey.toPublicKey()
        contract = new Contract(contractPublicKey)

        const tx = await Mina.transaction(user.publicKey, () => {
            AccountUpdate.fundNewAccount(user.publicKey)
            contract.deploy()
        })

        tx.sign([user.privateKey, contractPrivateKey])

        await tx.prove()

        await tx.send()
    })

    it("updates on-chain state", async () => {
        const tx = await Mina.transaction(user.publicKey, () => {
            contract.updateState(proofs[0])
        })

        tx.sign([user.privateKey])

        await tx.prove()

        await tx.send()
    })

    it("doesn't update on-chain state when the proof is mistaken", async () => {
        try {
            const tx = await Mina.transaction(user.publicKey, () => {
                contract.updateState(proofs[1])
            })
        } catch (error) {
            assert(
                (error as Error).message.startsWith(
                    SmartContractError.UnmatchedProofPublicInputAndOnChainMerkleRoot,
                ),
            )
        }
    })
})
