export type StateRoots = {
    balances: bigint
    liquidities: bigint
    pools: bigint
    burns: bigint
}

export type Sibling = {
    value: bigint
    isLeft: boolean
}

export type SingleBalanceWitness = {
    siblings: Array<Sibling>
}

export type DoubleBalanceWitness = {
    siblingsX1: Array<Sibling>
    siblingsX2: Array<Sibling>
    siblingsAt: Array<boolean>
}

export type SingleBurnWitness = {
    siblings: Array<Sibling>
}

export type SinglePoolWitness = {
    siblings: Array<Sibling>
}

export type SingleLiquidityWitness = {
    siblings: Array<Sibling>
}
