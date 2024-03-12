import { Field } from "o1js"
import assert from "node:assert"
import { describe, it } from "node:test"
import { prover } from "./prover.js"
import { Proof } from "./proof.js"
import { ProverError } from "./errors.js"

describe("Proof generation test", async () => {
    let proofs: Array<Proof> = []

    it("compiles the zk program", async () => {
        await prover.compile()
    })

    it("generates a proof", async () => {
        const proof = await prover.noOperation(Field(3))
        proofs.push(proof)
    })

    it("generates another proof", async () => {
        const proof = await prover.noOperation(Field(7))
        proofs.push(proof)
    })

    it("generates another proof", async () => {
        const proof = await prover.noOperation(Field(7))
        proofs.push(proof)
    })

    it("merges proofs", async () => {
        const proof = await prover.mergeProofs(Field(7), proofs[1], proofs[2])
        proofs.push(proof)
    })

    it("merges merged proofs", async () => {
        const proof = await prover.mergeProofs(Field(7), proofs[1], proofs[3])
    })

    it("doesn't merge proofs when the first proof's public input doesn't match the method's public input", async () => {
        try {
            const proof = await prover.mergeProofs(Field(7), proofs[0], proofs[2])
        } catch (error) {
            assert(
                (error as Error).message.startsWith(
                    ProverError.UnmatchedFirstProofPublicInputAndMethodPublicInput,
                ),
            )
        }
    })

    it("doesn't merge proofs when the second proof's public input doesn't match the first proof's public output", async () => {
        try {
            const proof = await prover.mergeProofs(Field(7), proofs[1], proofs[0])
        } catch (error) {
            assert(
                (error as Error).message.startsWith(
                    ProverError.UnmatchedSecondProofPublicInputAndFirstProofPublicOutput,
                ),
            )
        }
    })
})
