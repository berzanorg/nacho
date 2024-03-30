import { Sibling } from "./siblings.js"

export interface BridgeWitnesses {
    burnWitness: Array<Sibling>
    withdrawalWitness: Array<Sibling>
}
