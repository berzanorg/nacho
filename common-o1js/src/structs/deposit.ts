import { Field, PublicKey, Struct, UInt64 } from "o1js"

/** The provable data structure that represents a user's deposits of a single token from the layer 1 network. */
export class Deposit extends Struct({
    depositor: PublicKey,
    tokenId: Field,
    tokenAmount: UInt64,
}) {
    toFields(): [Field, Field, Field, Field] {
        return [
            this.depositor.x,
            this.depositor.isOdd.toField(),
            this.tokenId,
            this.tokenAmount.value,
        ]
    }
}
