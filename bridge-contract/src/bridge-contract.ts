import {
    AccountUpdate,
    Field,
    MerkleMapWitness,
    Nullifier,
    Permissions,
    Poseidon,
    PublicKey,
    Reducer,
    SmartContract,
    State,
    UInt64,
    method,
    state,
} from "o1js"
import { Burn, Deposit, SingleBurnWitness, SingleWithdrawalWitness } from "nacho-common-o1js"
import { RollupContract } from "nacho-rollup-contract"
import { SafeContract } from "./safe-contract.js"
import { TokenContract } from "nacho-token-contract"

export class BridgeContract extends SmartContract {
    reducer = Reducer({
        actionType: Field,
    })
    events = {
        deposited: Deposit,
    }

    @state(Field) actionState = State<Field>()
    @state(Field) depositsMerkleListRoot = State<Field>()
    @state(Field) withdrawalsMerkleTreeRoot = State<Field>()
    @state(PublicKey) rollupContractPublicKey = State<PublicKey>()

    init() {
        super.init()

        this.account.permissions.set({
            ...Permissions.allImpossible(),
            access: Permissions.proof(),
            editActionState: Permissions.proof(),
            editState: Permissions.proof(),
            incrementNonce: Permissions.proof(),
            send: Permissions.proof(),
            setPermissions: Permissions.proof(),
        })

        this.actionState.set(Reducer.initialActionState)
        this.depositsMerkleListRoot.set(Field(0))
        this.withdrawalsMerkleTreeRoot.set(
            Field(25436453236035485996795240493313170211557120058262356001829805101279552630634n),
        )
        this.rollupContractPublicKey.set(PublicKey.empty())
    }

    @method initRollupContractAddress(address: PublicKey) {
        this.rollupContractPublicKey.getAndRequireEquals().assertEquals(PublicKey.empty())

        this.rollupContractPublicKey.set(address)
    }

    @method addDeposit(tokenContractAddress: PublicKey, amount: UInt64) {
        const tokenContract = new TokenContract(tokenContractAddress)

        tokenContract.transfer(this.sender, this.address, amount)

        const deposit = new Deposit({
            depositor: this.sender,
            tokenId: tokenContract.deriveTokenId(),
            tokenAmount: amount,
        })

        this.reducer.dispatch(Poseidon.hash(deposit.toFields()))

        this.emitEvent("deposited", deposit)
    }

    @method applyDeposits() {
        const actionState = this.actionState.getAndRequireEquals()
        const depositsMerkleListRoot = this.depositsMerkleListRoot.getAndRequireEquals()

        const pendingActions = this.reducer.getActions({
            fromActionState: actionState,
        })

        const { state: newDepositsMerkleListRoot, actionState: newActionState } =
            this.reducer.reduce(
                pendingActions,
                Field,
                (state: Field, action: Field) => Poseidon.hash([state, action]),
                { state: depositsMerkleListRoot, actionState },
            )

        this.actionState.set(newActionState)
        this.depositsMerkleListRoot.set(newDepositsMerkleListRoot)
    }

    @method withdrawTokens(
        singleWithdrawWitness: SingleWithdrawalWitness,
        singleBurnWitness: SingleBurnWitness,
        tokenContractPublicKey: PublicKey,
        totalWithdrawAmount: UInt64,
        totalBurnAmount: UInt64,
    ) {
        const tokenContract = new TokenContract(tokenContractPublicKey)
        const tokenId = tokenContract.deriveTokenId()
        const safeContract = new SafeContract(this.address, tokenId)

        safeContract.checkAndSubBalance(
            singleWithdrawWitness,
            singleBurnWitness,
            tokenContractPublicKey,
            totalWithdrawAmount,
            totalBurnAmount,
        )

        const amount = totalBurnAmount.sub(totalWithdrawAmount)

        // NOTE: We don't have to check if this.sender is accurate because `SafeContract.checkAndSubBalance` already requires it to construct correct roots.
        tokenContract.transfer(safeContract.self, this.sender, amount)

        this.withdrawalsMerkleTreeRoot.set(
            singleWithdrawWitness.calculateRoot(
                Poseidon.hash([...this.sender.toFields(), tokenId, amount.value]),
            ),
        )
    }
}
