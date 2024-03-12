import { Field, SmartContract, State, method, state, Permissions } from "o1js"
import { Proof } from "./proof.js"
import { SmartContractError } from "./errors.js"

/**
 * The smart contract that resides on Mina Protocol.
 *
 * It stores the Merkle root of the rollup's state.
 *
 * It allows the Merkle root to be updated by verifying proofs.
 */
export class Contract extends SmartContract {
    /**
     * The state that stores the Merkle root of the rollup's state.
     *
     * The Merkle root is constructed by the witness database.
     */
    @state(Field) root = State<Field>()

    /**
     * The method that updates the on-chain state of the rollup.
     *
     * It takes a proof, checks if the proof's public input matches the Merkle root stored on-chain,
     * verifies the proof and updates the Merkle root with the proof's public output.
     */
    @method updateState(proof: Proof) {
        const root = this.root.getAndRequireEquals()

        proof.publicInput.assertEquals(
            root,
            SmartContractError.UnmatchedProofPublicInputAndOnChainMerkleRoot,
        )

        proof.verify()

        this.root.set(proof.publicOutput)
    }

    /**
     * The method that is called during the first deployment.
     *
     * It sets all the account permissions to impossible except `editState` and `access` permissions.
     *
     * It also sets the Merkle root stored on-chain as the root of an empty Merkle tree with 42 as the height.
     */
    init() {
        super.init()

        this.account.permissions.set({
            ...Permissions.allImpossible(),
            editState: Permissions.proof(),
            access: Permissions.proof(),
        })

        this.root.set(Field("0x1b2d2ee01d57dcf55e8f31af6842433f17d768c262ca1d8c3dd9b6c97f41b369"))
    }
}
