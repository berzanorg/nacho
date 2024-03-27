import { State, method, state, SmartContract } from "o1js"
import { StateRoots } from "nacho-common-o1js"
import { ZkProof } from "nacho-proof-generator"

export class RollupContract extends SmartContract {
    @state(StateRoots) stateRoots = State<StateRoots>()

    init() {
        super.init()
        this.stateRoots.set(StateRoots.empty())
    }

    @method settle(zkProof: ZkProof) {
        this.stateRoots.requireEquals(zkProof.publicInput)

        zkProof.verify()

        this.stateRoots.set(zkProof.publicOutput)
    }
}
