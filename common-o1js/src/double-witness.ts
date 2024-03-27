import { Bool, CircuitValue, Field, Poseidon, arrayProp } from "o1js"
import { Sibling } from "./types.js"
import { choose, putInOrder } from "./utils.js"

class DoubleWitnessBase extends CircuitValue {
    static siblingsCount: number
    siblingValuesX1: Array<Field>
    siblingIsLeftsX1: Array<Bool>
    siblingValuesX2: Array<Field>
    siblingIsLeftsX2: Array<Bool>
    siblingsAtX1X2: Array<Bool>

    siblingsCount(): number {
        return (this.constructor as any).siblingsCount
    }

    constructor(
        siblingsX1: Array<Sibling>,
        siblingsX2: Array<Sibling>,
        siblingsAtX1X2: Array<boolean>,
    ) {
        super()
        if (
            this.siblingsCount() !== siblingsX1.length ||
            this.siblingsCount() !== siblingsX2.length ||
            this.siblingsCount() !== siblingsAtX1X2.length
        ) {
            throw Error(
                `${siblingsX1.length}, ${siblingsX2.length} and ${
                    siblingsAtX1X2.length
                } which are the counts of the siblings don't match ${this.siblingsCount()} which is the expected count.`,
            )
        }
        this.siblingValuesX1 = siblingsX1.map((sibling) => Field(sibling.value))
        this.siblingIsLeftsX1 = siblingsX1.map((sibling) => Bool(sibling.isLeft))
        this.siblingValuesX2 = siblingsX2.map((sibling) => Field(sibling.value))
        this.siblingIsLeftsX2 = siblingsX2.map((sibling) => Bool(sibling.isLeft))
        this.siblingsAtX1X2 = siblingsAtX1X2.map((siblingsAt) => Bool(siblingsAt))
    }

    isCorrect(): Bool {
        // TODO: implement correctness check
        return Bool(true)
    }

    calculateRoot(valueX1: Field, valueX2: Field): Field {
        let rootX1 = valueX1
        let rootX2 = valueX2
        const s = this.siblingsCount()

        for (let i = 0; i < s; i++) {
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
        const s = this.siblingsCount()

        for (let i = 0; i < s; i++) {
            indexX1 = choose(this.siblingIsLeftsX1[i], indexX1.add(powerOfTwo), indexX1)
            indexX2 = choose(this.siblingIsLeftsX2[i], indexX2.add(powerOfTwo), indexX2)
            powerOfTwo = powerOfTwo.mul(2)
        }

        return [indexX1, indexX2]
    }
}

export const DoubleWitness = (height: number): typeof DoubleWitnessBase => {
    if (height < 2) {
        throw Error("A Merkle tree must have a height of 2 at least.")
    }
    const siblingsCount = height - 1
    class DoubleWitness extends DoubleWitnessBase {
        static siblingsCount = siblingsCount
    }
    arrayProp(Field, siblingsCount)(DoubleWitness.prototype, "siblingValuesX1")
    arrayProp(Bool, siblingsCount)(DoubleWitness.prototype, "siblingIsLeftsX1")
    arrayProp(Field, siblingsCount)(DoubleWitness.prototype, "siblingValuesX2")
    arrayProp(Bool, siblingsCount)(DoubleWitness.prototype, "siblingIsLeftsX2")
    arrayProp(Bool, siblingsCount)(DoubleWitness.prototype, "siblingsAtX1X2")
    return DoubleWitness
}
