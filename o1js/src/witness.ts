import { Field, UInt64, Provable, MerkleWitness, Struct, Poseidon } from "o1js"
import { choose, isSibling, swap } from "./utils"

/**
 * The provable data structure that represents a Merkle witness.
 *
 * First item is the index of the leaf and other items are the siblings.
 */
export class Witness extends Struct({
    0: Field,
    1: Field,
    2: Field,
    3: Field,
    4: Field,
    5: Field,
    6: Field,
    7: Field,
    8: Field,
    9: Field,
    10: Field,
    11: Field,
    12: Field,
    13: Field,
    14: Field,
    15: Field,
    16: Field,
    17: Field,
    18: Field,
    19: Field,
    20: Field,
    21: Field,
    22: Field,
    23: Field,
    24: Field,
    25: Field,
    26: Field,
    27: Field,
    28: Field,
    29: Field,
    30: Field,
    31: Field,
    32: Field,
    33: Field,
    34: Field,
    35: Field,
    36: Field,
    37: Field,
    38: Field,
    39: Field,
    40: Field,
    41: Field,
}) {
    static calculateRootX1(witnessX1: Witness, valueX1: Field): Field {
        let indexX1 = UInt64.from(witnessX1[0])

        let rootX1 = valueX1

        for (let i = 1; i < 42; i++) {
            const siblingX1 = (witnessX1 as unknown as Array<Field>)[i]

            const isLeftX1 = indexX1.mod(2).equals(UInt64.zero)

            const leftAndRightX1 = swap(isLeftX1, siblingX1, rootX1)
            rootX1 = Poseidon.hash(leftAndRightX1)

            indexX1 = indexX1.div(2)
        }

        return rootX1
    }

    static calculateRootX2(
        witnessX1: Witness,
        valueX1: Field,
        witnessX2: Witness,
        valueX2: Field,
    ): Field {
        let indexX1 = UInt64.from(witnessX1[0])
        let indexX2 = UInt64.from(witnessX2[0])

        let rootX1 = valueX1
        let rootX2 = valueX2

        for (let i = 1; i < 42; i++) {
            let siblingX1 = choose(
                isSibling(indexX1, indexX2),
                rootX2,
                (witnessX1 as unknown as Array<Field>)[i],
            )
            const siblingX2 = (witnessX2 as unknown as Array<Field>)[i]

            const isLeftX1 = indexX1.mod(2).equals(UInt64.zero)
            const isLeftX2 = indexX2.mod(2).equals(UInt64.zero)

            const leftAndRightX1 = swap(isLeftX1, siblingX1, rootX1)
            rootX1 = Poseidon.hash(leftAndRightX1)

            const leftAndRightX2 = swap(isLeftX2, siblingX2, rootX2)
            rootX1 = Poseidon.hash(leftAndRightX2)

            indexX1 = indexX1.div(2)
            indexX2 = indexX2.div(2)
        }

        return rootX1
    }
}
