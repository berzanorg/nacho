import { BridgeContract, SafeContract } from "nacho-bridge-contract"
import { proofGenerator } from "nacho-proof-generator"
import { RollupContract } from "nacho-rollup-contract"
import { TokenContract } from "nacho-token-contract"
import { AccountUpdate, Mina, PrivateKey, UInt64, fetchAccount, Lightnet } from "o1js"
import { setup } from "./setup.js"

if (process.argv.length !== 4) throw Error("args are not given")

const minaGraphqlEndpointUrl = process.argv[2]
const lightnetAccountManagerUrl = process.argv[3]

await setup({
    isTest: false,
    minaGraphqlEndpointUrl,
    lightnetAccountManagerUrl,
})
