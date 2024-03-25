import {
    Field,
    State,
    TokenContract as BaseTokenContract,
    UInt64,
    method,
    state,
    AccountUpdateForest,
    PublicKey,
} from "o1js"

export class TokenContract extends BaseTokenContract {
    @state(Field) bl = State<Field>()

    @method mint(amount: UInt64, receiver: PublicKey) {
        this.internal.mint({
            address: receiver,
            amount,
        })
    }

    @method approveBase(forest: AccountUpdateForest) {
        this.checkZeroBalanceChange(forest)
    }
}
