import { Field, Poseidon, PublicKey, SmartContract, UInt64, method } from "o1js"
import { Burn, SingleBurnWitness, SingleWithdrawalWitness, choose } from "nacho-common-o1js"
import { RollupContract } from "nacho-rollup-contract"
import { BridgeContract } from "./bridge-contract.js"
import { TokenContract } from "nacho-token-contract"

export class SafeContract extends SmartContract {
    @method async checkAndSubBalance(
        singleWithdrawalWitness: SingleWithdrawalWitness,
        singleBurnWitness: SingleBurnWitness,
        tokenContractPublicKey: PublicKey,
        totalWithdrawAmount: UInt64,
        totalBurnAmount: UInt64,
    ) {
        // NOTE: We require that both burn and withdraw leaves point to the same index.
        singleWithdrawalWitness.calculateIndex().assertEquals(singleBurnWitness.calculateIndex())

        const bridgeContract = new BridgeContract(this.address)
        const withdrawalsMerkleTreeRoot =
            bridgeContract.withdrawalsMerkleTreeRoot.getAndRequireEquals()
        const rollupContractPublicKey = bridgeContract.rollupContractPublicKey.getAndRequireEquals()

        const rollupContract = new RollupContract(rollupContractPublicKey)
        const tokenContract = new TokenContract(tokenContractPublicKey)

        const stateRoots = rollupContract.stateRoots.getAndRequireEquals()
        const tokenId = tokenContract.deriveTokenId()

        const burn = new Burn({
            burner: this.sender.getAndRequireSignature(),
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

        // We don't have to check if burned amount is greater than withdrawn amount as it throws an underflow error if it isn't.
        const amount = totalBurnAmount.sub(totalWithdrawAmount)

        this.balance.subInPlace(amount)
    }
}
