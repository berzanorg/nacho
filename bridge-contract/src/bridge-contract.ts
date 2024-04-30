import {
    AccountUpdate,
    Field,
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
import {
    Burn,
    Deposit,
    SingleBurnWitness,
    SingleWithdrawalWitness,
    Withdrawal,
    choose,
} from "nacho-common-o1js"
import { SafeContract } from "./safe-contract.js"
import { TokenContract } from "nacho-token-contract"
import { RollupContract } from "nacho-rollup-contract"

export class BridgeContract extends SmartContract {
    reducer = Reducer({
        actionType: Field,
    })
    events = {
        deposited: Deposit,
        withdrawn: Withdrawal,
    }

    @state(Field) actionState = State<Field>()
    @state(Field) depositsMerkleListRoot = State<Field>()
    @state(Field) withdrawalsMerkleTreeRoot = State<Field>()
    @state(PublicKey) rollupContractPublicKey = State<PublicKey>()

    init() {
        super.init()

        this.account.permissions.set({
            ...Permissions.allImpossible(),
            access: Permissions.none(),
            editActionState: Permissions.proof(),
            editState: Permissions.proof(),
            incrementNonce: Permissions.proof(),
            receive: Permissions.none(),
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

    @method addMinaDeposit(amount: UInt64) {
        const au = AccountUpdate.createSigned(this.sender)

        au.send({
            to: this.address,
            amount,
        })

        const deposit = new Deposit({
            depositor: this.sender,
            tokenId: Field(1), // Mina's Token ID.
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
        singleWithdrawalWitness: SingleWithdrawalWitness,
        singleBurnWitness: SingleBurnWitness,
        tokenContractPublicKey: PublicKey,
        totalWithdrawAmount: UInt64,
        totalBurnAmount: UInt64,
    ) {
        const tokenContract = new TokenContract(tokenContractPublicKey)
        const tokenId = tokenContract.deriveTokenId()
        const safeContract = new SafeContract(this.address, tokenId)

        safeContract.checkAndSubBalance(
            singleWithdrawalWitness,
            singleBurnWitness,
            tokenContractPublicKey,
            totalWithdrawAmount,
            totalBurnAmount,
        )

        const amount = totalBurnAmount.sub(totalWithdrawAmount)

        // NOTE: We don't have to check if this.sender is accurate because `SafeContract.checkAndSubBalance` already requires it to construct correct roots.
        tokenContract.transfer(safeContract.self, this.sender, amount)

        this.withdrawalsMerkleTreeRoot.set(
            singleWithdrawalWitness.calculateRoot(
                Poseidon.hash([...this.sender.toFields(), tokenId, amount.value]),
            ),
        )

        this.emitEvent(
            "withdrawn",
            new Withdrawal({
                withdrawer: this.sender,
                tokenId,
                tokenAmount: totalBurnAmount,
            }),
        )
    }

    @method withdrawMina(
        singleWithdrawalWitness: SingleWithdrawalWitness,
        singleBurnWitness: SingleBurnWitness,
        totalWithdrawAmount: UInt64,
        totalBurnAmount: UInt64,
    ) {
        const tokenId = Field(1)

        // We don't have to check if burned amount is greater than withdrawn amount as it throws an underflow error if it isn't.
        const amount = totalBurnAmount.sub(totalWithdrawAmount)

        // NOTE: We require that both burn and withdraw leaves point to the same index.
        singleWithdrawalWitness.calculateIndex().assertEquals(singleBurnWitness.calculateIndex())

        const bridgeContract = new BridgeContract(this.address)
        const withdrawalsMerkleTreeRoot =
            bridgeContract.withdrawalsMerkleTreeRoot.getAndRequireEquals()
        const rollupContractPublicKey = bridgeContract.rollupContractPublicKey.getAndRequireEquals()

        const rollupContract = new RollupContract(rollupContractPublicKey)

        const stateRoots = rollupContract.stateRoots.getAndRequireEquals()

        const burn = new Burn({
            burner: this.sender,
            tokenId,
            tokenAmount: totalBurnAmount,
        })
        const burnHash = Poseidon.hash(burn.toFields())

        const withdrawHash = choose(
            totalWithdrawAmount.equals(UInt64.zero),
            Field(0),
            Poseidon.hash(burn.toFields()),
        )

        stateRoots.burns.assertEquals(singleBurnWitness.calculateRoot(burnHash))
        withdrawalsMerkleTreeRoot.assertEquals(singleWithdrawalWitness.calculateRoot(withdrawHash))

        this.send({
            to: this.sender,
            amount,
        })

        this.withdrawalsMerkleTreeRoot.set(
            singleWithdrawalWitness.calculateRoot(
                Poseidon.hash([...this.sender.toFields(), tokenId, amount.value]),
            ),
        )

        this.emitEvent(
            "withdrawn",
            new Withdrawal({
                withdrawer: this.sender,
                tokenId,
                tokenAmount: totalBurnAmount,
            }),
        )
    }
}
