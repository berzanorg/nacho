import { Bool, Field, UInt64 } from "o1js"

/**
 * Whether swaps two `Field` elements or not based on the given condition.
 */
export const swap = (c: Bool, x: Field, y: Field): [Field, Field] => {
    const m = c.toField().mul(x.sub(y))
    const a = x.add(m)
    const b = y.sub(m)
    return [a, b]
}

/**
 * Checkes whether two given indexes are sibling indexes in a Merkle tree or not.
 */
export const isSibling = (x: UInt64, y: UInt64): Bool => {
    return x
        .mod(2)
        .equals(UInt64.from(2))
        .and(y.equals(x.add(1)))
        .or(
            y
                .mod(2)
                .equals(UInt64.from(2))
                .and(x.equals(y.add(1))),
        )
}

/**
 * Chooses between two `Field` elements based on the given condition.
 */
export const choose = (c: Bool, x: Field, y: Field): Field => {
    const m = c.toField()
    return x.mul(m).add(y.mul(Field(0).sub(m)))
}
