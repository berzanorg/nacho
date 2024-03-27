import { DoubleWitness } from "./double-witness.js"
import { SingleWitness } from "./single-witness.js"

export const BALANCES_TREE_HEIGHT = 23
export const LIQUIDITIES_TREE_HEIGHT = 22
export const POOLS_TREE_HEIGHT = 21
export const BURNS_TREE_HEIGHT = 20
export const WITHDRAWS_TREE_HEIGHT = 19

export class SingleBalanceWitness extends SingleWitness(BALANCES_TREE_HEIGHT) {}
export class DoubleBalanceWitness extends DoubleWitness(BALANCES_TREE_HEIGHT) {}
export class SingleLiquidityWitness extends SingleWitness(LIQUIDITIES_TREE_HEIGHT) {}
export class SinglePoolWitness extends SingleWitness(POOLS_TREE_HEIGHT) {}
export class SingleBurnWitness extends SingleWitness(BURNS_TREE_HEIGHT) {}
export class SingleWithdrawWitness extends SingleWitness(WITHDRAWS_TREE_HEIGHT) {}
