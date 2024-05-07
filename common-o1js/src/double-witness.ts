import { Bool, Field, Poseidon, Provable, Struct } from "o1js"
import { Sibling } from "./types"
import { choose, putInOrder } from "./utils"

export const DoubleWitness = (height: number) => {
    if (height < 2) {
        throw Error("A Merkle tree must have a height of 2 at least.")
    }

    return class DoubleWitnessWithGivenHeight extends Struct({
        siblingValuesX1: Provable.Array(Field, height - 1),
        siblingIsLeftsX1: Provable.Array(Bool, height - 1),
        siblingValuesX2: Provable.Array(Field, height - 1),
        siblingIsLeftsX2: Provable.Array(Bool, height - 1),
        siblingsAtX1X2: Provable.Array(Bool, height - 1),
    }) {
        constructor(
            siblingsX1: Array<Sibling>,
            siblingsX2: Array<Sibling>,
            siblingsAtX1X2: Array<boolean>,
        ) {
            if (siblingsX1.length !== height - 1) {
                throw Error(
                    `${siblingsX1.length} which is the count of the siblingsX1 doesn't match ${
                        height - 1
                    } which is the expected count.`,
                )
            }

            if (siblingsX2.length !== height - 1) {
                throw Error(
                    `${siblingsX2.length} which is the count of the siblingsX2 doesn't match ${
                        height - 1
                    } which is the expected count.`,
                )
            }

            if (siblingsAtX1X2.length !== height - 1) {
                throw Error(
                    `${
                        siblingsAtX1X2.length
                    } which is the count of the siblingsAtX1X2 doesn't match ${
                        height - 1
                    } which is the expected count.`,
                )
            }

            super({
                siblingValuesX1: siblingsX1.map((sibling) => Field(sibling.value)),
                siblingIsLeftsX1: siblingsX1.map((sibling) => Bool(sibling.isLeft)),
                siblingValuesX2: siblingsX2.map((sibling) => Field(sibling.value)),
                siblingIsLeftsX2: siblingsX2.map((sibling) => Bool(sibling.isLeft)),
                siblingsAtX1X2: siblingsAtX1X2.map((siblingsAt) => Bool(siblingsAt)),
            })
        }

        isCorrect(): Bool {
            // TODO: implement correctness check
            return Bool(true)
        }

        calculateRoot(valueX1: Field, valueX2: Field): Field {
            let rootX1 = valueX1
            let rootX2 = valueX2

            for (let i = 0; i < height - 1; i++) {
                const siblingX1 = choose(this.siblingsAtX1X2[i], rootX2, this.siblingValuesX1[i])

                rootX1 = Poseidon.hash(putInOrder(this.siblingIsLeftsX1[i], rootX1, siblingX1))

                rootX2 = Poseidon.hash(
                    putInOrder(this.siblingIsLeftsX2[i], rootX2, this.siblingValuesX2[i]),
                )
            }

            return rootX1
        }

        calculateIndexes(): [Field, Field] {
            let indexX1 = Field(0)
            let indexX2 = Field(0)
            let powerOfTwo = Field(1)

            for (let i = 0; i < height - 1; i++) {
                indexX1 = choose(this.siblingIsLeftsX1[i], indexX1.add(powerOfTwo), indexX1)
                indexX2 = choose(this.siblingIsLeftsX2[i], indexX2.add(powerOfTwo), indexX2)
                powerOfTwo = powerOfTwo.mul(2)
            }

            return [indexX1, indexX2]
        }
    }
}
