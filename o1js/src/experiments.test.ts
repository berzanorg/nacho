import assert from "node:assert"
import { describe } from "node:test"
import { Field, MerkleTree, Poseidon } from "o1js"

const swap = (b: boolean, x: Field, y: Field): [Field, Field] => (b ? [y, x] : [x, y])
const choose = (b: boolean, x: Field, y: Field): Field => (b ? x : y)

type Sibling = {
    value: Field
    isLeft: boolean
}

class WitnessX1<H extends number> {
    siblingsX1: Array<Sibling>
    height: H

    constructor(siblingsX1: Array<Sibling>, height: H) {
        this.siblingsX1 = siblingsX1
        this.height = height
    }

    root(valueX1: Field): Field {
        let rootX1 = valueX1

        for (let i = 0; i < this.height - 1; i++) {
            rootX1 = Poseidon.hash(
                swap(this.siblingsX1[i].isLeft, rootX1, this.siblingsX1[i].value),
            )
        }

        return rootX1
    }
}

class WitnessX2<H extends number> {
    siblingsX1: Array<Sibling>
    siblingsX2: Array<Sibling>
    x1AndX2AreSiblingsAt: Array<boolean>
    height: H

    constructor(siblingsX1: Array<Sibling>, siblingsX2: Array<Sibling>, height: H) {
        this.siblingsX1 = siblingsX1
        this.siblingsX2 = siblingsX2
        this.height = height

        this.x1AndX2AreSiblingsAt = new Array(this.height - 1).fill(false)

        let areX1AndX2SiblingsAt: number = this.height

        for (let i = this.height - 2; i >= 0; i--) {
            if (siblingsX1[i].isLeft !== siblingsX2[i].isLeft) {
                areX1AndX2SiblingsAt = i
                break
            }
        }

        if (areX1AndX2SiblingsAt !== this.height) {
            this.x1AndX2AreSiblingsAt[areX1AndX2SiblingsAt] = true
        }
    }

    root(valueX1: Field, valueX2: Field): Field {
        let rootX1 = valueX1
        let rootX2 = valueX2

        for (let i = 0; i < this.height - 1; i++) {
            const siblingX1 = choose(this.x1AndX2AreSiblingsAt[i], rootX2, this.siblingsX1[i].value)

            rootX1 = Poseidon.hash(swap(this.siblingsX1[i].isLeft, rootX1, siblingX1))
            rootX2 = Poseidon.hash(
                swap(this.siblingsX2[i].isLeft, rootX2, this.siblingsX2[i].value),
            )
        }

        return rootX1
    }
}

class WitnessX3<H extends number> {
    siblingsX1: Array<Sibling>
    siblingsX2: Array<Sibling>
    siblingsX3: Array<Sibling>
    x1AndX2AreSiblingsAt: Array<boolean>
    x2AndX3AreSiblingsAt: Array<boolean>
    height: H

    constructor(
        siblingsX1: Array<Sibling>,
        siblingsX2: Array<Sibling>,
        siblingsX3: Array<Sibling>,
        height: H,
    ) {
        this.siblingsX1 = siblingsX1
        this.siblingsX2 = siblingsX2
        this.siblingsX3 = siblingsX3
        this.height = height

        this.x1AndX2AreSiblingsAt = new Array(this.height - 1).fill(false)
        this.x2AndX3AreSiblingsAt = new Array(this.height - 1).fill(false)

        let areX1AndX2SiblingsAt: number = this.height
        let areX2AndX3SiblingsAt: number = this.height

        for (let i = this.height - 2; i >= 0; i--) {
            if (siblingsX1[i].isLeft !== siblingsX2[i].isLeft) {
                areX1AndX2SiblingsAt = i
                break
            }
        }

        for (let i = this.height - 2; i >= 0; i--) {
            if (siblingsX2[i].isLeft !== siblingsX3[i].isLeft) {
                areX2AndX3SiblingsAt = i
                break
            }
        }

        if (areX1AndX2SiblingsAt !== this.height) {
            this.x1AndX2AreSiblingsAt[areX1AndX2SiblingsAt] = true
        }

        if (areX2AndX3SiblingsAt !== this.height) {
            this.x2AndX3AreSiblingsAt[areX2AndX3SiblingsAt] = true
        }
    }

    root(valueX1: Field, valueX2: Field, valueX3: Field): Field {
        let rootX1 = valueX1
        let rootX2 = valueX2
        let rootX3 = valueX3

        for (let i = 0; i < this.height - 1; i++) {
            const siblingX1 = choose(this.x1AndX2AreSiblingsAt[i], rootX2, this.siblingsX1[i].value)
            const siblingX2 = choose(this.x2AndX3AreSiblingsAt[i], rootX3, this.siblingsX2[i].value)

            rootX1 = Poseidon.hash(swap(this.siblingsX1[i].isLeft, rootX1, siblingX1))
            rootX2 = Poseidon.hash(swap(this.siblingsX2[i].isLeft, rootX2, siblingX2))
            rootX3 = Poseidon.hash(
                swap(this.siblingsX3[i].isLeft, rootX3, this.siblingsX3[i].value),
            )
        }

        return rootX1
    }
}

class WitnessX4<H extends number> {
    siblingsX1: Array<Sibling>
    siblingsX2: Array<Sibling>
    siblingsX3: Array<Sibling>
    siblingsX4: Array<Sibling>
    x1AndX2AreSiblingsAt: Array<boolean>
    x2AndX3AreSiblingsAt: Array<boolean>
    x3AndX4AreSiblingsAt: Array<boolean>
    height: H

    constructor(
        siblingsX1: Array<Sibling>,
        siblingsX2: Array<Sibling>,
        siblingsX3: Array<Sibling>,
        siblingsX4: Array<Sibling>,
        height: H,
    ) {
        this.siblingsX1 = siblingsX1
        this.siblingsX2 = siblingsX2
        this.siblingsX3 = siblingsX3
        this.siblingsX4 = siblingsX4
        this.height = height

        this.x1AndX2AreSiblingsAt = new Array(this.height - 1).fill(false)
        this.x2AndX3AreSiblingsAt = new Array(this.height - 1).fill(false)
        this.x3AndX4AreSiblingsAt = new Array(this.height - 1).fill(false)

        let areX1AndX2SiblingsAt: number = this.height
        let areX2AndX3SiblingsAt: number = this.height
        let areX3AndX4SiblingsAt: number = this.height

        for (let i = this.height - 2; i >= 0; i--) {
            if (siblingsX1[i].isLeft !== siblingsX2[i].isLeft) {
                areX1AndX2SiblingsAt = i
                break
            }
        }

        for (let i = this.height - 2; i >= 0; i--) {
            if (siblingsX2[i].isLeft !== siblingsX3[i].isLeft) {
                areX2AndX3SiblingsAt = i
                break
            }
        }

        for (let i = this.height - 2; i >= 0; i--) {
            if (siblingsX3[i].isLeft !== siblingsX4[i].isLeft) {
                areX3AndX4SiblingsAt = i
                break
            }
        }

        if (areX1AndX2SiblingsAt !== this.height) {
            this.x1AndX2AreSiblingsAt[areX1AndX2SiblingsAt] = true
        }

        if (areX2AndX3SiblingsAt !== this.height) {
            this.x2AndX3AreSiblingsAt[areX2AndX3SiblingsAt] = true
        }

        if (areX3AndX4SiblingsAt !== this.height) {
            this.x3AndX4AreSiblingsAt[areX3AndX4SiblingsAt] = true
        }
    }

    root(valueX1: Field, valueX2: Field, valueX3: Field, valueX4: Field): Field {
        let rootX1 = valueX1
        let rootX2 = valueX2
        let rootX3 = valueX3
        let rootX4 = valueX4

        for (let i = 0; i < this.height - 1; i++) {
            const siblingX1 = choose(this.x1AndX2AreSiblingsAt[i], rootX2, this.siblingsX1[i].value)
            const siblingX2 = choose(this.x2AndX3AreSiblingsAt[i], rootX3, this.siblingsX2[i].value)
            const siblingX3 = choose(this.x3AndX4AreSiblingsAt[i], rootX4, this.siblingsX3[i].value)

            rootX1 = Poseidon.hash(swap(this.siblingsX1[i].isLeft, rootX1, siblingX1))
            rootX2 = Poseidon.hash(swap(this.siblingsX2[i].isLeft, rootX2, siblingX2))
            rootX3 = Poseidon.hash(swap(this.siblingsX3[i].isLeft, rootX3, siblingX3))
            rootX4 = Poseidon.hash(
                swap(this.siblingsX4[i].isLeft, rootX4, this.siblingsX4[i].value),
            )
        }

        return rootX1
    }
}

describe("Experiments", async () => {
    let tree = new MerkleTree(6)

    tree.setLeaf(0n, Field(4))

    const witnessX1 = new WitnessX1(
        tree.getWitness(6n).map((a) => ({
            isLeft: !a.isLeft,
            value: a.sibling,
        })),
        6,
    )

    const witnessX2 = new WitnessX2(
        tree.getWitness(1n).map((a) => ({
            isLeft: !a.isLeft,
            value: a.sibling,
        })),
        tree.getWitness(6n).map((a) => ({
            isLeft: !a.isLeft,
            value: a.sibling,
        })),
        6,
    )

    const witnessX3 = new WitnessX3(
        tree.getWitness(1n).map((a) => ({
            isLeft: !a.isLeft,
            value: a.sibling,
        })),
        tree.getWitness(4n).map((a) => ({
            isLeft: !a.isLeft,
            value: a.sibling,
        })),
        tree.getWitness(6n).map((a) => ({
            isLeft: !a.isLeft,
            value: a.sibling,
        })),
        6,
    )

    const witnessX4 = new WitnessX4(
        tree.getWitness(1n).map((a) => ({
            isLeft: !a.isLeft,
            value: a.sibling,
        })),
        tree.getWitness(4n).map((a) => ({
            isLeft: !a.isLeft,
            value: a.sibling,
        })),
        tree.getWitness(6n).map((a) => ({
            isLeft: !a.isLeft,
            value: a.sibling,
        })),
        tree.getWitness(7n).map((a) => ({
            isLeft: !a.isLeft,
            value: a.sibling,
        })),
        6,
    )

    tree.setLeaf(6n, Field(4))

    assert.deepEqual(tree.getRoot(), witnessX1.root(Field(4)))
    assert.deepEqual(tree.getRoot(), witnessX2.root(Field(0), Field(4)))
    assert.deepEqual(tree.getRoot(), witnessX3.root(Field(0), Field(0), Field(4)))
    assert.deepEqual(tree.getRoot(), witnessX4.root(Field(0), Field(0), Field(4), Field(0)))
})
