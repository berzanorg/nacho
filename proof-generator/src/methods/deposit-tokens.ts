import { Bool, Field, Poseidon, PublicKey, SelfProof, UInt64 } from "o1js"
import { Balance, Deposit, SingleBalanceWitness, StateRoots, choose } from "nacho-common-o1js"

export const depositTokens = (
    stateRoots: StateRoots,
    earlierProof: SelfProof<StateRoots, StateRoots>,
    singleBalanceWitness: SingleBalanceWitness,
    currentDepositsMerkleListHash: Field,
    expectedDepositsMerkleListHash: Field,
    userAddress: PublicKey,
    tokenId: Field,
    userDepositTokenAmount: UInt64,
    userBalanceTokenAmount: UInt64,
    isUsersFirstDeposit: Bool,
): StateRoots => {
    stateRoots.assertEquals(earlierProof.publicOutput)
    earlierProof.verify()

    const userBalance = new Balance({
        owner: userAddress,
        tokenId,
        tokenAmount: userBalanceTokenAmount,
    })

    stateRoots.balances.assertEquals(
        singleBalanceWitness.calculateRoot(
            choose(isUsersFirstDeposit, Field(0), Poseidon.hash(userBalance.toFields())),
        ),
    )

    const userDeposit = new Deposit({
        depositor: userAddress,
        tokenId,
        tokenAmount: userDepositTokenAmount,
    })

    // NOTE: A malicious party can submit false Merkle list hashes and make a deposit.
    expectedDepositsMerkleListHash.assertEquals(
        Poseidon.hash([currentDepositsMerkleListHash, Poseidon.hash(userDeposit.toFields())]),
    )

    choose(isUsersFirstDeposit, Field(0), userBalanceTokenAmount.value).assertEquals(Field(0))

    userBalance.tokenAmount = userBalanceTokenAmount.add(userDepositTokenAmount)

    stateRoots.balances = singleBalanceWitness.calculateRoot(Poseidon.hash(userBalance.toFields()))

    return stateRoots
}
