import { Sibling } from "./siblings.js"

export interface BridgeWitnesses {
    burnWitness: Array<Sibling>
    withdrawWitness: Array<Sibling>
}
