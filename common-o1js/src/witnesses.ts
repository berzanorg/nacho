import {
    BALANCES_TREE_HEIGHT,
    BURNS_TREE_HEIGHT,
    LIQUIDITIES_TREE_HEIGHT,
    POOLS_TREE_HEIGHT,
    WITHDRAWALS_TREE_HEIGHT,
} from "./constants"
import { DoubleWitness } from "./double-witness"
import { SingleWitness } from "./single-witness"

export class DoubleBalanceWitness extends DoubleWitness(BALANCES_TREE_HEIGHT) {}
export class SingleBalanceWitness extends SingleWitness(BALANCES_TREE_HEIGHT) {}
export class SingleBurnWitness extends SingleWitness(BURNS_TREE_HEIGHT) {}
export class SingleLiquidityWitness extends SingleWitness(LIQUIDITIES_TREE_HEIGHT) {}
export class SinglePoolWitness extends SingleWitness(POOLS_TREE_HEIGHT) {}
export class SingleWithdrawalWitness extends SingleWitness(WITHDRAWALS_TREE_HEIGHT) {}
