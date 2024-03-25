import {
    Field,
    Poseidon,
    Provable,
    PublicKey,
    Reducer,
    SmartContract,
    State,
    UInt64,
    method,
    state,
} from "o1js"
import { Deposit } from "common-o1js"
import { TokenContract } from "token-contract"

export class BridgeContract extends SmartContract {
    reducer = Reducer({
        actionType: Field,
    })
    events = {
        deposited: Deposit,
    }

    @state(Field) actionState = State<Field>()
    @state(Field) depositsMerkleListRoot = State<Field>()

    init() {
        super.init()
        this.actionState.set(Reducer.initialActionState)
        this.depositsMerkleListRoot.set(Field(0))
    }

    @method depositTokens(tokenContractAddress: PublicKey, amount: UInt64) {
        const tokenContract = new TokenContract(tokenContractAddress)

        tokenContract.transfer(this.sender, this.address, amount)

        const deposit = new Deposit({
            depositor: this.sender,
            tokenId: tokenContract.deriveTokenId(),
            tokenAmount: amount,
        })

        this.emitEvent("deposited", deposit)

        Provable.log(this.network.timestamp.getAndRequireEquals())

        const depositHash = Poseidon.hash(deposit.toFields())

        this.reducer.dispatch(depositHash)
    }

    @method withdrawTokens() {}

    @method rollActions() {
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
}
