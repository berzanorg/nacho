import { Field, Poseidon, PublicKey, SelfProof, Signature, UInt64 } from "o1js"
import {
    Balance,
    DoubleBalanceWitness,
    Pool,
    SinglePoolWitness,
    StateRoots,
    addOnePerMilFee,
    normalDiv,
} from "nacho-common-o1js"

export const makeBuyTokens = async (
    stateRoots: StateRoots,
    earlierProof: SelfProof<StateRoots, StateRoots>,
    singlePoolWitness: SinglePoolWitness,
    doubleBalanceWitness: DoubleBalanceWitness,
    userAddress: PublicKey,
    baseTokenId: Field,
    quoteTokenId: Field,
    userBalanceBaseTokenAmount: UInt64,
    userBalanceQuoteTokenAmount: UInt64,
    poolBaseTokenAmount: UInt64,
    poolQuoteTokenAmount: UInt64,
    poolTotalLiquidityPoints: Field,
    userBaseTokenAmountToSwap: UInt64,
    userQuoteTokenAmountLimitToSwap: UInt64,
    userSignature: Signature,
): Promise<StateRoots> => {
    stateRoots.assertEquals(earlierProof.publicOutput)
    earlierProof.verify()

    // The signature message is base and quote tokens IDs, base token amount and quote token amount limit.
    // NOTE: A compromised signature might be used to make multiple unwanted transactions.
    userSignature
        .verify(userAddress, [
            baseTokenId,
            quoteTokenId,
            userBaseTokenAmountToSwap.value,
            userQuoteTokenAmountLimitToSwap.value,
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
    stateRoots.balances.assertEquals(
        doubleBalanceWitness.calculateRoot(
            Poseidon.hash(userBaseTokenBalance.toFields()),
            Poseidon.hash(userQuoteTokenBalance.toFields()),
        ),
    )

    // NOTE: We don't have to check overflow because both numbers are less than 2^64 and the target can store up to 2^254.
    const k = poolBaseTokenAmount.value.mul(poolQuoteTokenAmount.value)

    // NOTE: We don't have to check if the base token amount of the pool is greater than the user's base token amount because it throws an error if there is an underflow.
    const newPoolBaseTokenAmount = poolBaseTokenAmount.sub(userBaseTokenAmountToSwap)

    newPoolBaseTokenAmount.equals(UInt64.zero).assertFalse()

    // NOTE: The result is always expected to be less than 2^64, `UInt64.from` throws an overflow error if it isn't.
    const newPoolQuoteTokenAmount = UInt64.fromFields([normalDiv(k, newPoolBaseTokenAmount.value)])

    // NOTE: The result is always expected to be greater than zero, it throws an underflow error if it isn't.
    const quoteTokenAmountToSwap = newPoolQuoteTokenAmount.sub(poolQuoteTokenAmount)

    const quoteTokenAmountToSwapPlusFee = addOnePerMilFee(quoteTokenAmountToSwap)
    quoteTokenAmountToSwapPlusFee.lessThanOrEqual(userQuoteTokenAmountLimitToSwap)

    userBaseTokenBalance.tokenAmount = userBalanceBaseTokenAmount.add(userBaseTokenAmountToSwap)
    // NOTE: We don't have to check if the user has enough quote token balance, because it throws underflow error if it isn't.
    userQuoteTokenBalance.tokenAmount = userBalanceQuoteTokenAmount.sub(
        quoteTokenAmountToSwapPlusFee,
    )
    // NOTE: We don't have to check if the pool has enough base token balance, because it throws underflow error if it isn't.
    pool.baseTokenAmount = poolBaseTokenAmount.sub(userBaseTokenAmountToSwap)
    pool.quoteTokenAmount = poolQuoteTokenAmount.add(quoteTokenAmountToSwapPlusFee)

    stateRoots.pools = singlePoolWitness.calculateRoot(Poseidon.hash(pool.toFields()))
    stateRoots.balances = doubleBalanceWitness.calculateRoot(
        Poseidon.hash(userBaseTokenBalance.toFields()),
        Poseidon.hash(userQuoteTokenBalance.toFields()),
    )

    return stateRoots
}
