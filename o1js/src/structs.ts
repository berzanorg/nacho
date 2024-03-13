import { Field, PublicKey, Struct, UInt64 } from "o1js"

/**
 * Each data structure is stored in the state database of the rollup.
 *
 * Witness database indexes are stored along with the data.
 *
 * Each data structure has a fixed size and can be used in provable code.
 */

/**
 * The data structure that represents a user's balance for a specific token.
 */
export class Balance extends Struct({
    wdbIndex: Field,
    tokenId: Field,
    tokenAmount: UInt64,
    tokenOwner: PublicKey,
}) {}

/**
 * The data structure that represents a pair of two specific tokens.
 */
export class Pair extends Struct({
    wdbIndex: Field,
    baseTokenId: Field,
    quoteTokenId: Field,
    baseTokenAmount: UInt64,
    quoteTokenAmount: UInt64,
}) {}

/**
 * The data structure that represents a user's liquidity in a pair of two specific tokens.
 */
export class Liqudity extends Struct({
    wdbIndex: Field,
    baseTokenId: Field,
    quoteTokenId: Field,
    points: UInt64,
}) {}
