import { Bool, Field, Struct } from "o1js"

/**
 * Represents the rollup's state in the most lightweight form.
 *
 * Contains the Merkle roots for each tree.
 */
export class StateRoots extends Struct({
    balances: Field,
    liquidities: Field,
    pools: Field,
    burns: Field,
}) {
    /** Returns true if this `StateRoots` is equal to `other`. */
    equals(other: StateRoots): Bool {
        return Bool(true)
            .and(this.balances.equals(other.balances))
            .and(this.liquidities.equals(other.liquidities))
            .and(this.pools.equals(other.pools))
            .and(this.burns.equals(other.burns))
    }

    /** Proves that this `StateRoots` is equal to `other`. */
    assertEquals(other: StateRoots, message?: string) {
        this.equals(other).assertTrue(message)
    }

    /** Creates an empty `StateRoots` with Merkle roots of empty trees for each tree. */
    static empty(): StateRoots {
        const ROOT_23 =
            27841935691558593279858640177961574373148122335514448527568736064618172266482n
        const ROOT_22 =
            1945127946440409282447574121167141731006841597528804291507158560727071219394n
        const ROOT_21 =
            19489292394622142448727235211662807700126173086870669586237893953121074753278n
        const ROOT_20 =
            23937279336243536139305946754911463754843381541673857352836322740025067834219n

        return new StateRoots({
            balances: Field(ROOT_23),
            liquidities: Field(ROOT_22),
            pools: Field(ROOT_21),
            burns: Field(ROOT_20),
        })
    }
}
