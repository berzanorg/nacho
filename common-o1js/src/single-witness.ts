import { Bool, CircuitValue, Field, Poseidon, arrayProp } from "o1js"
import { Sibling } from "./types.js"
import { choose, putInOrder } from "./utils.js"

class SingleWitnessBase extends CircuitValue {
    static siblingsCount: number
    siblingValues: Array<Field>
    siblingIsLefts: Array<Bool>

    siblingsCount(): number {
        return (this.constructor as any).siblingsCount
    }

    constructor(siblings: Array<Sibling>) {
        super()
        if (this.siblingsCount() !== siblings.length) {
            throw Error(
                `${
                    siblings.length
                } which is the count of the siblings doesn't match ${this.siblingsCount()} which is the expected count.`,
            )
        }
        this.siblingValues = siblings.map((sibling) => Field(sibling.value))
        this.siblingIsLefts = siblings.map((sibling) => Bool(sibling.isLeft))
    }

    calculateRoot(value: Field): Field {
        let root = value
        const s = this.siblingsCount()

        for (let i = 0; i < s; i++) {
            root = Poseidon.hash(putInOrder(this.siblingIsLefts[i], root, this.siblingValues[i]))
        }

        return root
    }

    calculateIndex(): Field {
        let index = Field(0)
        let powerOfTwo = Field(1)
        const s = this.siblingsCount()

        for (let i = 0; i < s; i++) {
            index = choose(this.siblingIsLefts[i], index.add(powerOfTwo), index)
            powerOfTwo = powerOfTwo.mul(2)
        }

        return index
    }
}

export const SingleWitness = (height: number): typeof SingleWitnessBase => {
    if (height < 2) {
        throw Error("A Merkle tree must have a height of 2 at least.")
    }
    const siblingsCount = height - 1
    class SingleWitness extends SingleWitnessBase {
        static siblingsCount = siblingsCount
    }
    arrayProp(Field, siblingsCount)(SingleWitness.prototype, "siblingValues")
    arrayProp(Bool, siblingsCount)(SingleWitness.prototype, "siblingIsLefts")
    return SingleWitness
}
