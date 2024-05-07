import { Field, PublicKey, Struct, UInt64 } from "o1js"

/** The provable data structure that represents a user's balance of a single token. */
export class Balance extends Struct({
    owner: PublicKey,
    tokenId: Field,
    tokenAmount: UInt64,
}) {
    toFields(): [Field, Field, Field, Field] {
        return [this.owner.x, this.owner.isOdd.toField(), this.tokenId, this.tokenAmount.value]
    }
}
