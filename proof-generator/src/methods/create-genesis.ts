import { StateRoots } from "nacho-common-o1js"

export const createGenesis = (stateRoots: StateRoots): StateRoots => {
    stateRoots.assertEquals(StateRoots.empty())

    return stateRoots
}
