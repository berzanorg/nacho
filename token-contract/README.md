# nacho-token-contract

The token smart contract built for [Nacho](https://github.com/berzanorg/nacho) using o1js.

[`nacho-token-contract`](https://www.npmjs.com/package/nacho-token-contract) package is intended to be used for [Nacho](https://github.com/berzanorg/nacho) only as it is a naive implementation.

## 📦 Installation

If you are using [`npm`](https://docs.npmjs.com/cli/):

```shell
npm install nacho-token-contract
```

If you are using [`yarn`](https://classic.yarnpkg.com/lang/en/docs/cli/):

```shell
yarn add nacho-token-contract
```

If you are using [`pnpm`](https://pnpm.io/pnpm-cli):

```shell
pnpm add nacho-token-contract
```

## 🔍 Details

Nacho's token contract extends o1js's existing token contract by implementing `approveBase` method's logic and adding a new method called `mint`.

### 🛠️ Smart Contract Methods

-   `mint(to, amount)`:

    Mints the given token amount to the given receiver with no checks.

-   `transfer(from, to, amount)`:

    Transfers the given token amount from the given address to the given address.

## 📝 License

This project is licensed under the MIT License.

You may use the source code of this project accordingly.

## 👤 About Me

I am [**Berzan**](https://berzan.org/) and I am developing [**Nacho**](https://nacho.finance/) to express my skills in the real world.

I built this library to use inside [**Nacho**](https://nacho.finance/).
