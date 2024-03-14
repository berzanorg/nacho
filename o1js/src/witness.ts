import { Field, UInt64, Provable, MerkleWitness, Struct, Poseidon } from "o1js"

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
    /**
     * The index of the leaf.
     */
    get index(): Field {
        return this[0]
    }

    /**
     * Calculates a Merkle root using the given value.
     */
    calculateRoot(value: Field): Field {
        let index = UInt64.from(this[0])

        let root = value

        for (let i = 1; i < 42; i++) {
            const sibling = (this as unknown as Record<number, Field>)[i]

            const isLeft = index.mod(2).equals(UInt64.zero)

            let m = isLeft.toField().mul(root.sub(sibling))
            const left = sibling.add(m)
            const right = root.sub(m)

            root = Poseidon.hash([left, right])

            index = index.div(2)
        }

        return root
    }
}

function calculateRoot1(witness: Witness, value: Field): Field {
    let index = UInt64.from(witness[0])

    let root = value

    for (let i = 1; i < 42; i++) {
        const sibling = (witness as unknown as Record<number, Field>)[i]

        const isLeft = index.mod(2).equals(UInt64.zero)

        let m = isLeft.toField().mul(root.sub(sibling))
        const left = sibling.add(m)
        const right = root.sub(m)

        root = Poseidon.hash([left, right])

        index = index.div(2)
    }

    return root
}

function calculateRoot2(witness1: Witness, value1: Field, witness2: Witness, value2: Field): Field {
    // TODO: Implement the logic for calculating a single root using two witnesses.
    return Field(0)
}

function calculateRoot3(
    witness1: Witness,
    value1: Field,
    witness2: Witness,
    value2: Field,
    witness3: Witness,
    value3: Field,
): Field {
    // TODO: Implement the logic for calculating a single root using two witnesses.
    return Field(0)
}

function calculateRoot4(
    witness1: Witness,
    value1: Field,
    witness2: Witness,
    value2: Field,
    witness3: Witness,
    value3: Field,
    witness4: Witness,
    value4: Field,
): Field {
    // TODO: Implement the logic for calculating a single root using two witnesses.
    return Field(0)
}
