import {
    TokenContract as BaseTokenContract,
    UInt64,
    method,
    AccountUpdateForest,
    PublicKey,
    Provable,
} from "o1js"

/** The token contract that is used for the bridge. */
export class TokenContract extends BaseTokenContract {
    /** Mints the given token amount to the given receiver with no checks. */
    @method async mint(amount: UInt64, receiver: PublicKey) {
        this.internal.mint({
            address: receiver,
            amount,
        })
    }

    /** Approves the given account updates if the total balance change is zero. */
    @method async approveBase(forest: AccountUpdateForest) {
        this.checkZeroBalanceChange(forest)
    }
}
