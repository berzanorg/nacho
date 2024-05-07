import { Field, Poseidon, PublicKey, SelfProof, Signature, UInt64 } from "o1js"
import {
    Balance,
    Burn,
    SingleBalanceWitness,
    SingleBurnWitness,
    StateRoots,
    choose,
} from "nacho-common-o1js"

export const makeBurnTokens = async (
    stateRoots: StateRoots,
    earlierProof: SelfProof<StateRoots, StateRoots>,
    singleBalanceWitness: SingleBalanceWitness,
    singleBurnWitness: SingleBurnWitness,
    userAddress: PublicKey,
    tokenId: Field,
    userBurnTokenAmount: UInt64,
    userBalanceTokenAmount: UInt64,
    amountToBurn: UInt64,
    userSignature: Signature,
): Promise<StateRoots> => {
    stateRoots.assertEquals(earlierProof.publicOutput)
    earlierProof.verify()

    // The signature message is a token ID and burn amount.
    // NOTE: A compromised signature might be used to make multiple unwanted transactions.
    userSignature.verify(userAddress, [tokenId, amountToBurn.value]).assertTrue()

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

    stateRoots.burns.assertEquals(
        singleBurnWitness.calculateRoot(
            choose(
                userBurnTokenAmount.equals(UInt64.zero),
                Field(0),
                Poseidon.hash(userBurn.toFields()),
            ),
        ),
    )

    // NOTE: We don't have to check if the user has enough balance because it throws an error if there is an underflow.
    userBalance.tokenAmount = userBalanceTokenAmount.sub(amountToBurn)
    userBurn.tokenAmount = userBurnTokenAmount.add(amountToBurn)

    stateRoots.balances = singleBalanceWitness.calculateRoot(Poseidon.hash(userBalance.toFields()))
    stateRoots.burns = singleBurnWitness.calculateRoot(Poseidon.hash(userBurn.toFields()))

    return stateRoots
}
