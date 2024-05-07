import { StateRoots } from "nacho-common-o1js"

export const createGenesis = async (stateRoots: StateRoots): Promise<StateRoots> => {
    stateRoots.assertEquals(StateRoots.empty())

    return stateRoots
}
