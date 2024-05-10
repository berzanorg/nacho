import { BridgeContract, SafeContract } from "nacho-bridge-contract"
import { proofGenerator } from "nacho-proof-generator"
import { RollupContract } from "nacho-rollup-contract"
import { TokenContract } from "nacho-token-contract"
import { AccountUpdate, Lightnet, Mina, PrivateKey, UInt64, fetchAccount } from "o1js"

type Setup =
    | {
          isTest: true
      }
    | {
          isTest: false
          minaGraphqlEndpointUrl: string
          lightnetAccountManagerUrl: string
      }

const TX_FEE = 1_000_000_000

export const setup = async (props: Setup) => {
    let sender = PrivateKey.random()

    if (props.isTest) {
        const LocalBlockchain = await Mina.LocalBlockchain()

        sender = LocalBlockchain.testAccounts[0].key

        Mina.setActiveInstance(LocalBlockchain)
    } else {
        const Remote = Mina.Network({
            mina: props.minaGraphqlEndpointUrl,
            lightnetAccountManager: props.lightnetAccountManagerUrl,
        })

        sender = (
            await Lightnet.acquireKeyPair({
                isRegularAccount: true,
            })
        ).privateKey

        Mina.setActiveInstance(Remote)
    }

    const abcPrivkey = PrivateKey.random()
    const xyzPrivkey = PrivateKey.random()
    const rollupPrivkey = PrivateKey.random()
    const bridgePrivkey = PrivateKey.random()

    const abcToken = new TokenContract(abcPrivkey.toPublicKey())
    const xyzToken = new TokenContract(xyzPrivkey.toPublicKey())
    const rollup = new RollupContract(rollupPrivkey.toPublicKey())
    const bridge = new BridgeContract(bridgePrivkey.toPublicKey())
    const abcSafe = new SafeContract(bridgePrivkey.toPublicKey(), abcToken.deriveTokenId())
    const xyzSafe = new SafeContract(bridgePrivkey.toPublicKey(), xyzToken.deriveTokenId())

    await TokenContract.compile()
    await proofGenerator.compile()
    await RollupContract.compile()
    await SafeContract.compile()
    await BridgeContract.compile()

    await fetchAccount({
        publicKey: sender.toPublicKey(),
    })

    const txFirst = await Mina.transaction(
        {
            sender: sender.toPublicKey(),
            fee: TX_FEE,
            nonce: 0,
        },
        async () => {
            await abcToken.deploy()
            await xyzToken.deploy()
            await rollup.deploy()
            await bridge.deploy()

            AccountUpdate.fundNewAccount(sender.toPublicKey(), 4)
        },
    )
    txFirst.sign([sender, abcPrivkey, xyzPrivkey, rollupPrivkey, bridgePrivkey])
    await txFirst.prove()
    await txFirst.send()

    const txSecond = await Mina.transaction(
        {
            sender: sender.toPublicKey(),
            fee: TX_FEE,
            nonce: 1,
        },
        async () => {
            await abcSafe.deploy()
            await xyzSafe.deploy()

            await abcToken.approveAccountUpdate(abcSafe.self)
            await xyzToken.approveAccountUpdate(xyzSafe.self)
            await bridge.initRollupContractAddress(rollup.address)

            AccountUpdate.fundNewAccount(sender.toPublicKey(), 2)
        },
    )
    txSecond.sign([sender, bridgePrivkey])
    await txSecond.prove()
    await txSecond.send()

    const txThird = await Mina.transaction(
        {
            sender: sender.toPublicKey(),
            fee: TX_FEE,
            nonce: 2,
        },
        async () => {
            await abcToken.mint(bridgePrivkey.toPublicKey(), UInt64.MAXINT())
            await xyzToken.mint(bridgePrivkey.toPublicKey(), UInt64.MAXINT())
        },
    )
    txThird.sign([sender])
    await txThird.prove()
    await txThird.send()
}
