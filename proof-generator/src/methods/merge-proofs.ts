import { SelfProof } from "o1js"
import { StateRoots } from "nacho-common-o1js"

export const mergeProofs = (
    stateRoots: StateRoots,
    currentProof: SelfProof<StateRoots, StateRoots>,
    laterProof: SelfProof<StateRoots, StateRoots>,
): StateRoots => {
    stateRoots.assertEquals(currentProof.publicInput)
    currentProof.publicOutput.assertEquals(laterProof.publicInput)

    currentProof.verify()
    laterProof.verify()

    return stateRoots
}
