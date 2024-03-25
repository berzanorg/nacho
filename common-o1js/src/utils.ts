import { Bool, Field, UInt64 } from "o1js"

/**
 * Swaps the order of `x` and `y` if `c` is true.
 * Leaves them in the same order if `c` is false.
 */
export const putInOrder = (c: Bool, x: Field, y: Field): [Field, Field] => {
    const m = c.toField().mul(x.sub(y))
    const a = x.sub(m)
    const b = y.add(m)
    return [a, b]
}

/**
 * Returns `x` if `c` is true.
 * Returns `y` if `c` is false.
 */
export const choose = (c: Bool, x: Field, y: Field): Field => {
    const m = c.toField()
    return x.mul(m).add(y.mul(Field(1).sub(m)))
}
