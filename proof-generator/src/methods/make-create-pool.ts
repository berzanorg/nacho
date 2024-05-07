import { Field, Poseidon, PublicKey, SelfProof, Signature, UInt64 } from "o1js"
import {
    Balance,
    DoubleBalanceWitness,
    Liquidity,
    Pool,
    SingleLiquidityWitness,
    SinglePoolWitness,
    StateRoots,
} from "nacho-common-o1js"

export const makeCreatePool = async (
    stateRoots: StateRoots,
    earlierProof: SelfProof<StateRoots, StateRoots>,
    singlePoolWitness: SinglePoolWitness,
    singleLiquidityWitness: SingleLiquidityWitness,
    doubleBalanceWitness: DoubleBalanceWitness,
    baseTokenId: Field,
    quoteTokenId: Field,
    userAddress: PublicKey,
    userLiquidityBaseTokenAmount: UInt64,
    userLiquidityQuoteTokenAmount: UInt64,
    userBalanceBaseTokenAmount: UInt64,
    userBalanceQuoteTokenAmount: UInt64,
    userSignature: Signature,
): Promise<StateRoots> => {
    stateRoots.assertEquals(earlierProof.publicOutput)
    earlierProof.verify()

    // The signature message is base and quote tokens IDs and liquidity amounts.
    // NOTE: A compromised signature might be used to make multiple unwanted transactions.
    userSignature
        .verify(userAddress, [
            baseTokenId,
            quoteTokenId,
            userLiquidityBaseTokenAmount.value,
            userLiquidityQuoteTokenAmount.value,
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

    // NOTE: We don't have to check overflow because both numbers are less than 2^64 and the target can store up to 2^254.
    const totalLiquidityPoints = userLiquidityBaseTokenAmount.value.mul(
        userLiquidityQuoteTokenAmount.value,
    )

    const userLiquidity = new Liquidity({
        provider: userAddress,
        baseTokenId,
        quoteTokenId,
        points: totalLiquidityPoints,
    })

    const pool = new Pool({
        baseTokenId,
        quoteTokenId,
        baseTokenAmount: userLiquidityBaseTokenAmount,
        quoteTokenAmount: userLiquidityQuoteTokenAmount,
        totalLiquidityPoints,
    })

    // NOTE: We make sure that this double Merkle witness is correct to prevent mistaken root calculation.
    doubleBalanceWitness.isCorrect().assertTrue()

    stateRoots.pools.assertEquals(singlePoolWitness.calculateRoot(Field(0)))
    stateRoots.liquidities.assertEquals(singleLiquidityWitness.calculateRoot(Field(0)))
    stateRoots.balances.assertEquals(
        doubleBalanceWitness.calculateRoot(
            Poseidon.hash(userBaseTokenBalance.toFields()),
            Poseidon.hash(userQuoteTokenBalance.toFields()),
        ),
    )

    userLiquidityBaseTokenAmount.equals(UInt64.zero).assertFalse()
    userLiquidityQuoteTokenAmount.equals(UInt64.zero).assertFalse()

    // NOTE: We don't have to check if the user has enough balance because it throws an error if there is an underflow.
    userBaseTokenBalance.tokenAmount = userBalanceBaseTokenAmount.sub(userLiquidityBaseTokenAmount)
    userQuoteTokenBalance.tokenAmount = userBalanceQuoteTokenAmount.sub(
        userLiquidityQuoteTokenAmount,
    )

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
