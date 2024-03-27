import { Field, Poseidon, Provable, PublicKey, SelfProof, Signature, UInt64 } from "o1js"
import {
    Balance,
    DoubleBalanceWitness,
    Liquidity,
    Pool,
    SingleLiquidityWitness,
    SinglePoolWitness,
    StateRoots,
    normalDiv,
} from "nacho-common-o1js"

export const makeRemoveLiquidity = (
    stateRoots: StateRoots,
    earlierProof: SelfProof<StateRoots, StateRoots>,
    singlePoolWitness: SinglePoolWitness,
    singleLiquidityWitness: SingleLiquidityWitness,
    doubleBalanceWitness: DoubleBalanceWitness,
    baseTokenId: Field,
    quoteTokenId: Field,
    userAddress: PublicKey,
    userLiquidityPoints: Field,
    userBalanceBaseTokenAmount: UInt64,
    userBalanceQuoteTokenAmount: UInt64,
    poolBaseTokenAmount: UInt64,
    poolQuoteTokenAmount: UInt64,
    poolTotalLiquidityPoints: Field,
    userLiquidityPointsToRemove: Field,
    userBaseTokenAmountLimitToRemove: UInt64,
    userQuoteTokenAmountLimitToRemove: UInt64,
    userSignature: Signature,
): StateRoots => {
    stateRoots.assertEquals(earlierProof.publicOutput)
    earlierProof.verify()

    // The signature message is base and quote tokens IDs and limits and liquidity points.
    // NOTE: A compromised signature might be used to make multiple unwanted transactions.
    userSignature
        .verify(userAddress, [
            baseTokenId,
            quoteTokenId,
            userBaseTokenAmountLimitToRemove.value,
            userQuoteTokenAmountLimitToRemove.value,
            userLiquidityPointsToRemove,
        ])
        .assertTrue()

    const userBaseTokenBalance = new Balance({
        owner: userAddress,
        tokenId: baseTokenId,
        tokenAmount: userBalanceBaseTokenAmount,
    })

    const userQuoteTokenBalance = new Balance({
        owner: userAddress,
        tokenId: quoteTokenId,
        tokenAmount: userBalanceQuoteTokenAmount,
    })

    const userLiquidity = new Liquidity({
        provider: userAddress,
        baseTokenId,
        quoteTokenId,
        points: userLiquidityPoints,
    })

    const pool = new Pool({
        baseTokenId,
        quoteTokenId,
        baseTokenAmount: poolBaseTokenAmount,
        quoteTokenAmount: poolQuoteTokenAmount,
        totalLiquidityPoints: poolTotalLiquidityPoints,
    })

    // NOTE: We make sure that this double Merkle witness is correct to prevent mistaken root calculation.
    doubleBalanceWitness.isCorrect().assertTrue()

    stateRoots.pools.assertEquals(singlePoolWitness.calculateRoot(Poseidon.hash(pool.toFields())))
    stateRoots.liquidities.assertEquals(
        singleLiquidityWitness.calculateRoot(Poseidon.hash(userLiquidity.toFields())),
    )
    stateRoots.balances.assertEquals(
        doubleBalanceWitness.calculateRoot(
            Poseidon.hash(userBaseTokenBalance.toFields()),
            Poseidon.hash(userQuoteTokenBalance.toFields()),
        ),
    )

    userLiquidityPointsToRemove.equals(Field(0)).assertFalse()
    poolTotalLiquidityPoints.equals(Field(0)).assertFalse()
    poolBaseTokenAmount.equals(UInt64.zero).assertFalse()

    // NOTE: We don't have to check overflow because the base token amount in a pool is always less than 2^64,
    // the total liquidity points in a pool is always less than 2^128 and can never be equal to zero.
    // Plus the base token amount in the pool is never equal to zero.
    // And `UInt64.from` checks overflows.
    const baseTokenAmountToRemove = UInt64.from(
        normalDiv(
            poolBaseTokenAmount.value.mul(userLiquidityPointsToRemove),
            poolTotalLiquidityPoints,
        ),
    )
    // NOTE: We don't have to check overflow because the quote token amount in a pool is always less than 2^64,
    // the total liquidity points in a pool is always less than 2^128 and can never be equal to zero.
    // Plus the base token amount in the pool is never equal to zero.
    // And `UInt64.from` checks overflows.
    const quoteTokenAmountToRemove = UInt64.from(
        normalDiv(
            poolQuoteTokenAmount.value.mul(userLiquidityPointsToRemove),
            poolTotalLiquidityPoints,
        ),
    )

    baseTokenAmountToRemove.assertGreaterThanOrEqual(userBaseTokenAmountLimitToRemove)
    quoteTokenAmountToRemove.assertLessThanOrEqual(userQuoteTokenAmountLimitToRemove)

    // NOTE: We don't have to check if the user has enough liquidity points because it throws an error if there is an underflow.
    userLiquidity.points = userLiquidityPoints.sub(userLiquidityPointsToRemove)
    // NOTE: We don't have to check if the pool has enough liquidity points because it throws an error if there is an underflow.
    pool.totalLiquidityPoints = poolTotalLiquidityPoints.sub(userLiquidityPointsToRemove)
    // NOTE: We don't have to check if the pool has enough balance because it throws an error if there is an underflow.
    pool.baseTokenAmount = poolBaseTokenAmount.sub(baseTokenAmountToRemove)
    // NOTE: We don't have to check if the pool has enough balance because it throws an error if there is an underflow.
    pool.quoteTokenAmount = poolQuoteTokenAmount.sub(quoteTokenAmountToRemove)
    userBaseTokenBalance.tokenAmount = userBalanceBaseTokenAmount.add(baseTokenAmountToRemove)
    userQuoteTokenBalance.tokenAmount = userBalanceQuoteTokenAmount.add(quoteTokenAmountToRemove)

    stateRoots.pools = singlePoolWitness.calculateRoot(Poseidon.hash(pool.toFields()))
    stateRoots.liquidities = singleLiquidityWitness.calculateRoot(
        Poseidon.hash(userLiquidity.toFields()),
    )
    stateRoots.balances = doubleBalanceWitness.calculateRoot(
        Poseidon.hash(userBaseTokenBalance.toFields()),
        Poseidon.hash(userQuoteTokenBalance.toFields()),
    )

    return stateRoots
}
