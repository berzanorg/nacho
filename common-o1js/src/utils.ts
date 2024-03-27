import { Bool, Field, Provable, UInt64 } from "o1js"

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

/**
 * Divides `x` with `y` and returns the result.
 * Make sure that `x / y` won't result an error before calling this function.
 */
export const normalDiv = (x: Field, y: Field): Field => {
    // TODO: Add circuit check to prove.
    return Provable.witness(Field, () => new Field(x.toBigInt() / y.toBigInt()))
}

/**
 * Multiplies `x` with `1001` then divides the result with `1000`.
 * Make sure that `x * 1.001` won't result an error.
 */
export const addOnePerMilFee = (x: UInt64): UInt64 => {
    return UInt64.from(normalDiv(x.value.mul(Field(1001)), Field(1000)))
}
