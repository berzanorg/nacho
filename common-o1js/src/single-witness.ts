import { Bool, Field, Poseidon, Provable, Struct } from "o1js"
import { Sibling } from "./types"
import { choose, putInOrder } from "./utils"

export const SingleWitness = (height: number) => {
    if (height < 2) {
        throw Error("A Merkle tree must have a height of 2 at least.")
    }

    return class SingleWitnessWithGivenHeight extends Struct({
        siblingValues: Provable.Array(Field, height - 1),
        siblingIsLefts: Provable.Array(Bool, height - 1),
    }) {
        constructor(siblings: Array<Sibling>) {
            if (siblings.length !== height - 1) {
                throw Error(
                    `${siblings.length} which is the count of the siblings doesn't match ${
                        height - 1
                    } which is the expected count.`,
                )
            }

            super({
                siblingValues: siblings.map((sibling) => Field(sibling.value)),
                siblingIsLefts: siblings.map((sibling) => Bool(sibling.isLeft)),
            })
        }

        calculateRoot(value: Field): Field {
            let root = value

            for (let i = 0; i < height - 1; i++) {
                root = Poseidon.hash(
                    putInOrder(this.siblingIsLefts[i], root, this.siblingValues[i]),
                )
            }

            return root
        }

        calculateIndex(): Field {
            let index = Field(0)
            let powerOfTwo = Field(1)

            for (let i = 0; i < height - 1; i++) {
                index = choose(this.siblingIsLefts[i], index.add(powerOfTwo), index)
                powerOfTwo = powerOfTwo.mul(2)
            }

            return index
        }
    }
}
