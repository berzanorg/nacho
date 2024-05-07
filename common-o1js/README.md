# nacho-common-o1js

The common [o1js](https://www.npmjs.com/package/o1js) utilities for [Nacho](https://github.com/berzanorg/nacho).

[`nacho-common-o1js`](https://www.npmjs.com/package/nacho-common-o1js) package is intended to be used for [Nacho](https://github.com/berzanorg/nacho) only as it contains naive implementations of the things Nacho needs.

## 📦 Installation

If you are using [`npm`](https://docs.npmjs.com/cli/):

```shell
npm install nacho-common-o1js
```

If you are using [`yarn`](https://classic.yarnpkg.com/lang/en/docs/cli/):

```shell
yarn add nacho-common-o1js
```

If you are using [`pnpm`](https://pnpm.io/pnpm-cli):

```shell
pnpm add nacho-common-o1js
```

## 🔍 Details

Nacho's common o1js utilities are some provable data structures, functions and constant values.

### 🧱 Provable Data Structures

-   [`Balance`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/structs/balance.ts):

    Represents a token balance of a user in the layer 2.

-   [`Burn`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/structs/burn.ts):

    Represents a token burn of a user in the layer 2.

-   [`Deposit`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/structs/deposit.ts):

    Represents a token deposit of a user in the layer 1.

-   [`DoubleBalanceWitness`](https://github.com/berzanorg/nacho/blobmain/common-o1js/src/witnesses.ts#L11):

    Represents the witness of two leaves in the Merkle tree that storesbalances.

-   [`Liquidity`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/structs/liquidity.ts):

    Represents an AMM pool liquidity of a user in the layer 2.

-   [`Pool`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/structs/pool.ts):

    Represents an AMM pool in the layer 2.

-   [`SingleBalanceWitness`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/witnesses.ts#L12):

    Represents the witness of a leaf in the Merkle tree that stores balances.

-   [`SingleBurnWitness`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/witnesses.ts#L13):

    Represents the witness of a leaf in the Merkle tree that stores burns.

-   [`SingleLiquidityWitness`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/witnesses.ts#L14):

    Represents the witness of a leaf in the Merkle tree that stores AMM liquidities.

-   [`SinglePoolWitness`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/witnesses.ts#L15):

    Represents the witness of a leaf in the Merkle tree that stores AMM pools.

-   [`SingleWithdrawalWitness`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/witnesses.ts#L16):

    Represents the witness of a leaf in the Merkle tree that stores withdrawals.

-   [`StateRoots`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/structs/state-roots.ts):

    Represents the Merkle roots of the layer 2's state in the layer 1.

-   [`Withdrawal`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/structs/withdrawal.ts):

    Represents a token withdrawal of a user in the layer 1.

### 🛠️ Provable Functions

-   [`addOnePerMilFee`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/utils.ts#L7):

    Multiplies the given number by `1.001`.

-   [`choose`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/utils.ts#L15):

    Equivalent of conditional operator.

-   [`normalDiv`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/utils.ts#L24):

    Normal division operation on field elements.

-   [`putInOrder`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/utils.ts#L33):

    Swaps the order of given elements based on given the condition.

### 🏷️ Constant Values

-   [`BALANCES_TREE_HEIGHT`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/constants.ts#L1):

    The height of the Merkle tree that stores balances which is `23`.

-   [`BURNS_TREE_HEIGHT`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/constants.ts#L2):

    The height of the Merkle tree that stores burns which is `20`.

-   [`LIQUIDITIES_TREE_HEIGHT`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/constants.ts#L3):

    The height of the Merkle tree that stores AMM liquidities which is `22`.

-   [`POOLS_TREE_HEIGHT`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/constants.ts#L4):

    The height of the Merkle tree that stores AMM pools which is `21`.

-   [`WITHDRAWALS_TREE_HEIGHT`](https://github.com/berzanorg/nacho/blob/main/common-o1js/src/constants.ts#L5):

    The height of the Merkle tree that stores withdrawals which is `19`.

## 📝 License

This project is licensed under the MIT License.

You may use the source code of this project accordingly.

## 👤 About Me

I am [**Berzan**](https://berzan.org/) and I am developing [**Nacho**](https://nacho.finance/) to express my skills in the real world.

I built this library to use inside [**Nacho**](https://nacho.finance/).
