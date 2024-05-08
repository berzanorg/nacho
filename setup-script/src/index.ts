import { BridgeContract, SafeContract } from "nacho-bridge-contract"
import { proofGenerator } from "nacho-proof-generator"
import { RollupContract } from "nacho-rollup-contract"
import { TokenContract } from "nacho-token-contract"
import { AccountUpdate, Mina, PrivateKey, UInt64, fetchAccount } from "o1js"

if (process.argv.length !== 4) throw Error("args are not given")

const minaUrl = process.argv[2]
const privKey = process.argv[3]

Mina.setActiveInstance(Mina.Network(minaUrl))

const signer = PrivateKey.fromBase58(privKey)

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
    publicKey: signer.toPublicKey(),
})

let tx = await Mina.transaction(signer.toPublicKey(), async () => {
    await abcToken.deploy()
    await xyzToken.deploy()

    AccountUpdate.fundNewAccount(signer.toPublicKey(), 2)
})
tx.sign([signer, abcPrivkey, xyzPrivkey])
await tx.prove()
await tx.send()

tx = await Mina.transaction(signer.toPublicKey(), async () => {
    await rollup.deploy()
    AccountUpdate.fundNewAccount(signer.toPublicKey(), 1)
})
tx.sign([signer, rollupPrivkey])
await tx.prove()
await tx.send()

tx = await Mina.transaction(signer.toPublicKey(), async () => {
    await bridge.deploy()
    await abcSafe.deploy()
    await xyzSafe.deploy()
    AccountUpdate.fundNewAccount(signer.toPublicKey(), 3)
    await abcToken.approveAccountUpdate(abcSafe.self)
    await xyzToken.approveAccountUpdate(xyzSafe.self)
})
tx.sign([signer, bridgePrivkey])
await tx.prove()
await tx.send()

tx = await Mina.transaction(signer.toPublicKey(), async () => {
    await abcToken.mint(bridgePrivkey.toPublicKey(), UInt64.MAXINT())
    await xyzToken.mint(bridgePrivkey.toPublicKey(), UInt64.MAXINT())
})
tx.sign([signer])
await tx.prove()
await tx.send()

tx = await Mina.transaction(signer.toPublicKey(), async () => {
    bridge.initRollupContractAddress(rollup.address)
})
tx.sign([signer])
await tx.prove()
await tx.send()

console.log(`ROLLUP PUBKEY: ${rollupPrivkey.toPublicKey().toBase58()}`)
console.log(`BRIDGE PUBKEY: ${bridgePrivkey.toPublicKey().toBase58()}`)
