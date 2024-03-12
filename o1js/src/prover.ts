import { Field, SelfProof, ZkProgram } from "o1js"
import { ProverError } from "./errors.js"

/**
 * The zk program that is used to generate proofs.
 *
 * The public input and output represent the Merkle root of the rollup's state.
 *
 * The merkle root of the rollup's state is constructed from the witness database.
 */
export const prover = ZkProgram({
    name: "prover",
    publicInput: Field,
    publicOutput: Field,
    methods: {
        /**
         * The method that does no opeartion.
         *
         * It returns the public input as the public output without any modifications.
         */
        noOperation: {
            privateInputs: [],
            method(root: Field) {
                return root
            },
        },

        /**
         * The method that merges two proofs.
         *
         * It checks if the first proof's public input matches this method's public input
         * and the second proof's public input matches the first proof's public output,
         * then verifies both of the proofs, lastly returns the second proof's public output.
         */
        mergeProofs: {
            privateInputs: [SelfProof, SelfProof],
            method(
                root: Field,
                firstProof: SelfProof<Field, Field>,
                secondProof: SelfProof<Field, Field>,
            ) {
                firstProof.publicInput.assertEquals(
                    root,
                    ProverError.UnmatchedFirstProofPublicInputAndMethodPublicInput,
                )

                secondProof.publicInput.assertEquals(
                    firstProof.publicOutput,
                    ProverError.UnmatchedSecondProofPublicInputAndFirstProofPublicOutput,
                )

                firstProof.verify()
                secondProof.verify()

                return secondProof.publicOutput
            },
        },
    },
})
