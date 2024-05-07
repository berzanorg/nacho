import { Field, MerkleTree, Poseidon, PublicKey, UInt64 } from "o1js"
import { ZkProof } from "../src/index.js"
import {
    BALANCES_TREE_HEIGHT,
    BURNS_TREE_HEIGHT,
    DoubleBalanceWitness,
    LIQUIDITIES_TREE_HEIGHT,
    POOLS_TREE_HEIGHT,
    SingleBalanceWitness,
    SingleBurnWitness,
    SingleLiquidityWitness,
    SinglePoolWitness,
    StateRoots,
} from "nacho-common-o1js"

export const createStateUtil = () => {
    const balancesTree = new MerkleTree(BALANCES_TREE_HEIGHT)
    const poolsTree = new MerkleTree(POOLS_TREE_HEIGHT)
    const liquiditiesTree = new MerkleTree(LIQUIDITIES_TREE_HEIGHT)
    const burnsTree = new MerkleTree(BURNS_TREE_HEIGHT)
    let depositsRoot = Field(0)
    const proofs: Array<ZkProof> = []

    return {
        get proofs() {
            return proofs
        },
        get lastProof() {
            return proofs.at(-1)!
        },
        pushProof(proof: ZkProof) {
            return proofs.push(proof)
        },
        get stateRoots() {
            const stateRoots = StateRoots.empty()
            stateRoots.balances = balancesTree.getRoot()
            stateRoots.pools = poolsTree.getRoot()
            stateRoots.liquidities = liquiditiesTree.getRoot()
            stateRoots.burns = burnsTree.getRoot()
            return stateRoots
        },
        get currentDepositsRoot() {
            return depositsRoot
        },
        getExpectedDepositsRoot(userAddress: PublicKey, tokenId: Field, tokenAmount: UInt64) {
            return Poseidon.hash([
                depositsRoot,
                Poseidon.hash([...userAddress.toFields(), tokenId, tokenAmount.value]),
            ])
        },
        pushDeposit(userAddress: PublicKey, tokenId: Field, tokenAmount: UInt64) {
            depositsRoot = Poseidon.hash([
                depositsRoot,
                Poseidon.hash([...userAddress.toFields(), tokenId, tokenAmount.value]),
            ])
        },
        getSingleBalanceWitness(index: bigint): SingleBalanceWitness {
            return new SingleBalanceWitness(
                balancesTree
                    .getWitness(index)
                    .map((a) => ({ value: a.sibling.toBigInt(), isLeft: !a.isLeft })),
            )
        },
        setBalance(index: bigint, userAddress: PublicKey, tokenId: Field, tokenAmount: UInt64) {
            balancesTree.setLeaf(
                index,
                Poseidon.hash([...userAddress.toFields(), tokenId, tokenAmount.value]),
            )
        },
        getDoubleBalanceWitness(indexX1: bigint, indexX2: bigint): DoubleBalanceWitness {
            const siblingsX1 = balancesTree
                .getWitness(indexX1)
                .map((a) => ({ value: a.sibling.toBigInt(), isLeft: !a.isLeft }))

            const siblingsX2 = balancesTree
                .getWitness(indexX2)
                .map((a) => ({ value: a.sibling.toBigInt(), isLeft: !a.isLeft }))

            const siblingsAt = new Array<boolean>(BALANCES_TREE_HEIGHT - 1).fill(false)

            for (let i = BALANCES_TREE_HEIGHT - 2; i >= 0; i--) {
                if (siblingsX1[i].isLeft !== siblingsX2[i].isLeft) {
                    siblingsAt[i] = true
                }
            }

            return new DoubleBalanceWitness(siblingsX1, siblingsX2, siblingsAt)
        },
        getSingleLiqudityWitness(index: bigint): SingleLiquidityWitness {
            return new SingleLiquidityWitness(
                liquiditiesTree
                    .getWitness(index)
                    .map((a) => ({ value: a.sibling.toBigInt(), isLeft: !a.isLeft })),
            )
        },
        setLiquidity(
            index: bigint,
            provider: PublicKey,
            baseTokenId: Field,
            quoteTokenId: Field,
            points: Field,
        ) {
            liquiditiesTree.setLeaf(
                index,
                Poseidon.hash([...provider.toFields(), baseTokenId, quoteTokenId, points]),
            )
        },
        getSinglePoolWitness(index: bigint): SinglePoolWitness {
            return new SinglePoolWitness(
                poolsTree
                    .getWitness(index)
                    .map((a) => ({ value: a.sibling.toBigInt(), isLeft: !a.isLeft })),
            )
        },
        setPool(
            index: bigint,
            baseTokenId: Field,
            quoteTokenId: Field,
            baseTokenAmount: UInt64,
            quoteTokenAmount: UInt64,
            totalLiquidityPoints: Field,
        ) {
            poolsTree.setLeaf(
                index,
                Poseidon.hash([
                    baseTokenId,
                    quoteTokenId,
                    baseTokenAmount.value,
                    quoteTokenAmount.value,
                    totalLiquidityPoints,
                ]),
            )
        },
        getSingleBurnWitness(index: bigint): SingleBurnWitness {
            return new SingleBurnWitness(
                burnsTree
                    .getWitness(index)
                    .map((a) => ({ value: a.sibling.toBigInt(), isLeft: !a.isLeft })),
            )
        },
        setBurn(index: bigint, burner: PublicKey, tokenId: Field, tokenAmount: UInt64) {
            burnsTree.setLeaf(
                index,
                Poseidon.hash([...burner.toFields(), tokenId, tokenAmount.value]),
            )
        },
    }
}
