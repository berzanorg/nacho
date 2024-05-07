import { State, method, state, SmartContract } from "o1js"
import { StateRoots } from "nacho-common-o1js"
import { ZkProof } from "nacho-proof-generator"

/** The rollup contract that stores the state roots which can be updated by submitting valid zk proofs. */
export class RollupContract extends SmartContract {
    @state(StateRoots) stateRoots = State<StateRoots>()

    init() {
        super.init()
        this.stateRoots.set(StateRoots.empty())
    }

    /** Verifies the given zk proof and updates the state roots if it's valid. */
    @method async settle(zkProof: ZkProof) {
        this.stateRoots.getAndRequireEquals().assertEquals(zkProof.publicInput)

        zkProof.verify()

        this.stateRoots.set(zkProof.publicOutput)
    }
}
