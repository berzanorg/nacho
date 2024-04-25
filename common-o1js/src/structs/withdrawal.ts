import { Field, PublicKey, Struct, UInt64 } from "o1js"

export class Withdrawal extends Struct({
    withdrawer: PublicKey,
    tokenId: Field,
    tokenAmount: UInt64,
}) {
    toFields(): [Field, Field, Field, Field] {
        return [
            this.withdrawer.x,
            this.withdrawer.isOdd.toField(),
            this.tokenId,
            this.tokenAmount.value,
        ]
    }
}
