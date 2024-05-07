import { Field, PublicKey, Struct, UInt64 } from "o1js"

/** The provable data structure that represents a user's burnings of a single token inside the layer 2 network. */
export class Burn extends Struct({
    burner: PublicKey,
    tokenId: Field,
    tokenAmount: UInt64,
}) {
    toFields(): [Field, Field, Field, Field] {
        return [this.burner.x, this.burner.isOdd.toField(), this.tokenId, this.tokenAmount.value]
    }
}
