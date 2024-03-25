import { SingleWitness } from "./single-witness.js"

const BALANCES_TREE_HEIGHT = 20
const LIQUIDITIES_TREE_HEIGHT = 19
const PAIRS_TREE_HEIGHT = 18
const BURNS_TREE_HEIGHT = 17
export const DEPOSITS_TREE_HEIGHT = 16

export class SingleBalanceWitness extends SingleWitness(BALANCES_TREE_HEIGHT) {}
export class SingleDepositWitness extends SingleWitness(DEPOSITS_TREE_HEIGHT) {}
