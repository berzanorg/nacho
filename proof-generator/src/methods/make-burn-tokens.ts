import { Field, Poseidon, PublicKey, SelfProof, Signature, UInt64 } from "o1js"
import {
    Balance,
    Burn,
    SingleBalanceWitness,
    SingleBurnWitness,
    StateRoots,
} from "nacho-common-o1js"

export const makeBurnTokens = (
    stateRoots: StateRoots,
    earlierProof: SelfProof<StateRoots, StateRoots>,
    singleBalanceWitness: SingleBalanceWitness,
    singleBurnWitness: SingleBurnWitness,
    userAddress: PublicKey,
    tokenId: Field,
    userBurnTokenAmount: UInt64,
    userBalanceTokenAmount: UInt64,
    userSignature: Signature,
): StateRoots => {
    stateRoots.assertEquals(earlierProof.publicOutput)
    earlierProof.verify()

    // The signature message is a token ID and burn amount.
    // NOTE: A compromised signature might be used to make multiple unwanted transactions.
    userSignature.verify(userAddress, [tokenId, userBurnTokenAmount.value]).assertTrue()

    const userBalance = new Balance({
        owner: userAddress,
        tokenId,
        tokenAmount: userBalanceTokenAmount,
    })

    const userBurn = new Burn({
        burner: userAddress,
        tokenId,
        tokenAmount: userBurnTokenAmount,
    })

    stateRoots.balances.assertEquals(
        singleBalanceWitness.calculateRoot(Poseidon.hash(userBalance.toFields())),
    )

    stateRoots.burns.assertEquals(singleBurnWitness.calculateRoot(Field(0)))

    // NOTE: We don't have to check if the user has enough balance because it throws an error if there is an underflow.
    userBalance.tokenAmount = userBalanceTokenAmount.sub(userBurnTokenAmount)

    stateRoots.burns = singleBurnWitness.calculateRoot(Poseidon.hash(userBurn.toFields()))
    stateRoots.balances = singleBalanceWitness.calculateRoot(Poseidon.hash(userBalance.toFields()))

    return stateRoots
}
