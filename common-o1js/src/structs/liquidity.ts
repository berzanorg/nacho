import { Field, PublicKey, Struct, UInt64 } from "o1js"

export class Liquidity extends Struct({
    provider: PublicKey,
    baseTokenId: Field,
    quoteTokenId: Field,
    points: Field,
}) {
    toFields(): [Field, Field, Field, Field, Field] {
        return [
            this.provider.x,
            this.provider.isOdd.toField(),
            this.baseTokenId,
            this.quoteTokenId,
            this.points,
        ]
    }
}
