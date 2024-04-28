import { Bool, Field, Poseidon, Provable, PublicKey, SelfProof, Signature, UInt64 } from "o1js"
import {
    Balance,
    DoubleBalanceWitness,
    Liquidity,
    Pool,
    SingleLiquidityWitness,
    SinglePoolWitness,
    StateRoots,
    choose,
    normalDiv,
} from "nacho-common-o1js"

export const makeProvideLiquidity = (
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
    userBaseTokenAmountToProvide: UInt64,
    userQuoteTokenAmountLimitToProvide: UInt64,
    userSignature: Signature,
): StateRoots => {
    stateRoots.assertEquals(earlierProof.publicOutput)
    earlierProof.verify()

    // The signature message is base and quote tokens IDs, base token amount and quote token amount limit.
    // NOTE: A compromised signature might be used to make multiple unwanted transactions.
    userSignature
        .verify(userAddress, [
            baseTokenId,
            quoteTokenId,
            userBaseTokenAmountToProvide.value,
            userQuoteTokenAmountLimitToProvide.value,
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
    const liquiditiesRootIfFirstProviding = singleLiquidityWitness.calculateRoot(Field(0))
    const isFirstProviding = stateRoots.liquidities.equals(liquiditiesRootIfFirstProviding)

    Bool.or(
        stateRoots.liquidities.equals(liquiditiesRootIfFirstProviding),
        stateRoots.liquidities.equals(
            singleLiquidityWitness.calculateRoot(Poseidon.hash(userLiquidity.toFields())),
        ),
    ).assertTrue()

    stateRoots.balances.assertEquals(
        doubleBalanceWitness.calculateRoot(
            Poseidon.hash(userBaseTokenBalance.toFields()),
            Poseidon.hash(userQuoteTokenBalance.toFields()),
        ),
    )

    // NOTE: We make sure that the given user liquidity points are zero, if it is the user's first liquidity providing in this pool.
    choose(isFirstProviding, userLiquidityPoints, Field(0)).assertEquals(Field(0))

    userBaseTokenAmountToProvide.equals(UInt64.zero).assertFalse()
    poolBaseTokenAmount.equals(UInt64.zero).assertFalse()

    // NOTE: We don't have to check overflow because the total liquidity points in a pool is always less than 2^128,
    // the base token amount to provide as a liquidity is always less than 2^64 and the target can store up to 2^254.
    // Plus the base token amount in the pool is never equal to zero.
    const liquidityPointsToBeCreated = normalDiv(
        poolTotalLiquidityPoints.mul(userBaseTokenAmountToProvide.value),
        poolBaseTokenAmount.value,
    )

    // NOTE: We don't have to check overflow because both the base token amount to provide as a liquidity
    // and the quote token amount in the pool are always less than 2^64
    // and the target can store up to 2^254.
    // Plus the base token amount in the pool is never equal to zero.
    // And `UInt64.from` checks overflows.
    const quoteTokenAmountToProvide = UInt64.from(
        normalDiv(
            userBaseTokenAmountToProvide.value.mul(poolQuoteTokenAmount.value),
            poolBaseTokenAmount.value,
        ),
    )

    quoteTokenAmountToProvide.assertLessThanOrEqual(userQuoteTokenAmountLimitToProvide)

    // NOTE: We don't have to check if the user has enough balance because it throws an error if there is an underflow.
    userBaseTokenBalance.tokenAmount = userBalanceBaseTokenAmount.sub(userBaseTokenAmountToProvide)
    // NOTE: We don't have to check if the user has enough balance because it throws an error if there is an underflow.
    userQuoteTokenBalance.tokenAmount = userBalanceQuoteTokenAmount.sub(quoteTokenAmountToProvide)
    userLiquidity.points = userLiquidityPoints.add(liquidityPointsToBeCreated)
    pool.totalLiquidityPoints = poolTotalLiquidityPoints.add(liquidityPointsToBeCreated)
    pool.baseTokenAmount = poolBaseTokenAmount.add(userBaseTokenAmountToProvide)
    pool.quoteTokenAmount = poolQuoteTokenAmount.add(quoteTokenAmountToProvide)

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
