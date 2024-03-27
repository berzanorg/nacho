import { Field, Struct, UInt64 } from "o1js"

export class Pool extends Struct({
    baseTokenId: Field,
    quoteTokenId: Field,
    baseTokenAmount: UInt64,
    quoteTokenAmount: UInt64,
    totalLiquidityPoints: Field,
}) {
    toFields(): [Field, Field, Field, Field, Field] {
        return [
            this.baseTokenId,
            this.quoteTokenId,
            this.baseTokenAmount.value,
            this.quoteTokenAmount.value,
            this.totalLiquidityPoints,
        ]
    }
}
