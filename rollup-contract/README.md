# nacho-rollup-contract

The rollup smart contract of [Nacho](https://github.com/berzanorg/nacho) built using [o1js](https://www.npmjs.com/package/o1js).

[`nacho-rollup-contract`](https://www.npmjs.com/package/nacho-rollup-contract) package is intended to be used for [Nacho](https://github.com/berzanorg/nacho) only as it is a naive implementation.

## 📦 Installation

If you are using [`npm`](https://docs.npmjs.com/cli/):

```shell
npm install nacho-rollup-contract
```

If you are using [`yarn`](https://classic.yarnpkg.com/lang/en/docs/cli/):

```shell
yarn add nacho-rollup-contract
```

If you are using [`pnpm`](https://pnpm.io/pnpm-cli):

```shell
pnpm add nacho-rollup-contract
```

## 🔍 Details

Nacho's rollup contract extends o1js's `SmartContract` by overriding existing `init` method and adding a new method called `settle`.
The method `init` is overridden to initialize the state of the contract as empty root hashes of Merkle trees that hold the rollup's state.

### 🛠️ Smart Contract Methods

-   [`settle(zkProof)`](https://github.com/berzanorg/nacho/blob/main/rollup-contractr/src/rollup-contract.ts#L15):

    Takes a [`ZkProof`](https://github.com/berzanorg/nacho/blob/main/proof-generator/src/zk-proof.ts) as the only parameter and checks if its public input is equal to the settled one, then updates the contract's state accordingly.

## 📝 License

This project is licensed under the MIT License.

You may use the source code of this project accordingly.

## 👤 About Me

I am [**Berzan**](https://berzan.org/) and I am developing [**Nacho**](https://nacho.finance/) to express my skills in the real world.

I built this library to use inside [**Nacho**](https://nacho.finance/).
