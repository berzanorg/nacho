import { Field, PublicKey, Struct, UInt64 } from "o1js"

export class Burn extends Struct({
    burner: PublicKey,
    tokenId: Field,
    tokenAmount: UInt64,
}) {
    toFields(): [Field, Field, Field, Field] {
        return [this.burner.x, this.burner.isOdd.toField(), this.tokenId, this.tokenAmount.value]
    }
}
