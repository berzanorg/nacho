import { Bool, Field, Poseidon, PublicKey, SelfProof, UInt64 } from "o1js"
import { Balance, Deposit, SingleBalanceWitness, StateRoots, choose } from "nacho-common-o1js"

export const depositTokens = async (
    stateRoots: StateRoots,
    earlierProof: SelfProof<StateRoots, StateRoots>,
    singleBalanceWitness: SingleBalanceWitness,
    currentDepositsMerkleListHash: Field,
    expectedDepositsMerkleListHash: Field,
    userAddress: PublicKey,
    tokenId: Field,
    userDepositTokenAmount: UInt64,
    userBalanceTokenAmount: UInt64,
): Promise<StateRoots> => {
    stateRoots.assertEquals(earlierProof.publicOutput)
    earlierProof.verify()

    const userBalance = new Balance({
        owner: userAddress,
        tokenId,
        tokenAmount: userBalanceTokenAmount,
    })

    const balancesRootIfFirstDeposit = singleBalanceWitness.calculateRoot(Field(0))

    const isUsersFirstDeposit = stateRoots.balances.equals(balancesRootIfFirstDeposit)

    Bool.or(
        stateRoots.balances.equals(balancesRootIfFirstDeposit),
        stateRoots.balances.equals(
            singleBalanceWitness.calculateRoot(Poseidon.hash(userBalance.toFields())),
        ),
    ).assertTrue()

    const userDeposit = new Deposit({
        depositor: userAddress,
        tokenId,
        tokenAmount: userDepositTokenAmount,
    })

    // NOTE: A malicious party can submit false Merkle list hashes and make a deposit.
    expectedDepositsMerkleListHash.assertEquals(
        Poseidon.hash([currentDepositsMerkleListHash, Poseidon.hash(userDeposit.toFields())]),
    )

    choose(isUsersFirstDeposit, userBalanceTokenAmount.value, Field(0)).assertEquals(0)

    userBalance.tokenAmount = userBalanceTokenAmount.add(userDepositTokenAmount)

    stateRoots.balances = singleBalanceWitness.calculateRoot(Poseidon.hash(userBalance.toFields()))

    return stateRoots
}
