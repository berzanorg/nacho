import {
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
import { Burn, Deposit, SingleBurnWitness, SingleWithdrawWitness } from "nacho-common-o1js"
import { RollupContract } from "nacho-rollup-contract"
import { BridgeContract } from "./bridge-contract.js"
import { TokenContract } from "nacho-token-contract"

export class SafeContract extends SmartContract {
    @method checkAndSubBalance(
        singleWithdrawWitness: SingleWithdrawWitness,
        singleBurnWitness: SingleBurnWitness,
        tokenContractPublicKey: PublicKey,
        amount: UInt64,
    ) {
        // NOTE: We require that both burn and withdraw leaves point to the same index.
        singleWithdrawWitness.calculateIndex().assertEquals(singleBurnWitness.calculateIndex())

        const bridgeContract = new BridgeContract(this.address)
        const withdrawsMerkleTreeRoot = bridgeContract.withdrawsMerkleTreeRoot.getAndRequireEquals()
        const rollupContractPublicKey = bridgeContract.rollupContractPublicKey.getAndRequireEquals()

        const rollupContract = new RollupContract(rollupContractPublicKey)
        const tokenContract = new TokenContract(tokenContractPublicKey)

        const stateRoots = rollupContract.stateRoots.getAndRequireEquals()
        const tokenId = tokenContract.deriveTokenId()

        const burn = new Burn({
            burner: this.sender,
            tokenId,
            tokenAmount: amount,
        })
        const burnHash = Poseidon.hash(burn.toFields())

        stateRoots.burns.assertEquals(singleBurnWitness.calculateRoot(burnHash))
        withdrawsMerkleTreeRoot.assertEquals(singleWithdrawWitness.calculateRoot(Field(0)))

        this.balance.subInPlace(amount)
    }
}
