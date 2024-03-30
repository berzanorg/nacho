import { RPC_SERVER_URL } from "./constants.js"
import { parseGetBalances } from "./parse/parse-get-balances.js"
import { parseGetBurns } from "./parse/parse-get-burns.js"
import { parseGetLiquidities } from "./parse/parse-get-liquidities.js"
import { parseGetPools } from "./parse/parse-get-pools.js"
import { parseTxStatus } from "./parse/parse-get-tx-status.js"
import { Balance } from "./types/balance.js"
import { Burn } from "./types/burn.js"
import { Liquidity } from "./types/liquidity.js"
import { BridgeWitnesses } from "./types/bridge-witnesses.js"
import { Pool } from "./types/pool.js"
import { TxStatus } from "./types/tx-status.js"
import { unparseGetTxStatus } from "./unparse/unparse-get-tx-status.js"
import { unparseGetTotalTxCount } from "./unparse/unparse-get-total-tx-count.js"
import { parseGetTotalTxCount } from "./parse/parse-get-total-tx-count.js"
import { unparseGetBalances } from "./unparse/unparse-get-balances.js"
import { unparseGetPools } from "./unparse/unparse-get-pools.js"
import { unparseGetLiquidities } from "./unparse/unparse-get-liquidities.js"
import { unparseGetBurns } from "./unparse/unparse-get-burns.js"
import { unparseGetBridgeWitnesses } from "./unparse/unparse-get-bridge-witnesses.js"
import { parseGetBridgeWitnesses } from "./parse/parse-get-bridge-witnesses.js"
import { unparseBurnTokens } from "./unparse/unparse-burn-tokens.js"
import { parseTxId } from "./parse/parse-tx-id.js"
import { unparseCreatePool } from "./unparse/unparse-create-pool.js"
import { unparseProvideLiquidity } from "./unparse/unparse-provide-liquidity.js"
import { unparseRemoveLiquidity } from "./unparse/unparse-remove-liquidity.js"
import { unparseBuyTokens } from "./unparse/unparse-buy-tokens.js"
import { unparseSellTokens } from "./unparse/unparse-sell-tokens.js"

declare function setInterval(handler: TimerHandler, timeout?: number, ...arguments: any[]): number

interface ClientReadMethods {
    getTotalTxCount: () => Promise<number>

    getTxStatus: (txId: number) => Promise<TxStatus>

    getBalances: (address: string) => Promise<Array<Balance>>

    getPools: () => Promise<Array<Pool>>

    getLiquidities: (address: string) => Promise<Array<Liquidity>>

    getBurns: (address: string) => Promise<Array<Burn>>

    getBridgeWitnesses: (burnId: number) => Promise<BridgeWitnesses>
}

interface ClientWriteMethods {
    burnTokens: (
        address: string,
        signature: string,
        tokenId: bigint,
        amount: bigint,
    ) => Promise<void>

    createPool: (
        address: string,
        signature: string,
        baseTokenId: bigint,
        quoteTokenId: bigint,
        baseTokenAmount: bigint,
        quoteTokenAmount: bigint,
    ) => Promise<void>

    provideLiquidity: (
        address: string,
        signature: string,
        baseTokenId: bigint,
        quoteTokenId: bigint,
        baseTokenAmount: bigint,
        quoteTokenAmountLimit: bigint,
    ) => Promise<void>

    removeLiquidity: (
        address: string,
        signature: string,
        baseTokenId: bigint,
        quoteTokenId: bigint,
        baseTokenAmountLimit: bigint,
        quoteTokenAmountLimit: bigint,
        liquidityPointAmount: bigint,
    ) => Promise<void>

    buyTokens: (
        address: string,
        signature: string,
        baseTokenId: bigint,
        quoteTokenId: bigint,
        baseTokenAmount: bigint,
        quoteTokenAmountLimit: bigint,
    ) => Promise<void>

    sellTokens: (
        address: string,
        signature: string,
        baseTokenId: bigint,
        quoteTokenId: bigint,
        baseTokenAmountLimit: bigint,
        quoteTokenAmount: bigint,
    ) => Promise<void>
}

export class Client implements ClientReadMethods, ClientWriteMethods {
    private static url = new URL(RPC_SERVER_URL)
    constructor() {}

    private async request(buf: ArrayBuffer): Promise<ArrayBuffer> {
        const httpResponse = await fetch(Client.url, {
            method: "POST",
            body: buf,
        })

        const responseBody = await httpResponse.arrayBuffer()

        return responseBody
    }

    private async waitTransaction(txId: number): Promise<void> {
        let interval: number | undefined = undefined

        await new Promise<void>((resolve, reject) => {
            interval = setInterval(async () => {
                const status = await this.getTxStatus(txId)

                if (status === TxStatus.Rejected) {
                    clearInterval(interval)
                    reject()
                }

                if (status !== TxStatus.Pending) {
                    clearInterval(interval)
                    resolve()
                }
            }, 250)
        })
    }

    public async getTotalTxCount(): Promise<number> {
        const request = unparseGetTotalTxCount()
        const response = await this.request(request)
        return parseGetTotalTxCount(response)
    }

    public async getTxStatus(txId: number): Promise<TxStatus> {
        const request = unparseGetTxStatus(txId)
        const response = await this.request(request)
        return parseTxStatus(response)
    }

    public async getBalances(address: string): Promise<Array<Balance>> {
        const request = unparseGetBalances(address)
        const response = await this.request(request)
        return parseGetBalances(response)
    }

    public async getPools(): Promise<Array<Pool>> {
        const request = unparseGetPools()
        const response = await this.request(request)
        return parseGetPools(response)
    }

    public async getLiquidities(address: string): Promise<Array<Liquidity>> {
        const request = unparseGetLiquidities(address)
        const response = await this.request(request)
        return parseGetLiquidities(response)
    }

    public async getBurns(address: string): Promise<Array<Burn>> {
        const request = unparseGetBurns(address)
        const response = await this.request(request)
        return parseGetBurns(response)
    }

    public async getBridgeWitnesses(burnId: number): Promise<BridgeWitnesses> {
        const request = unparseGetBridgeWitnesses(burnId)
        const response = await this.request(request)
        return parseGetBridgeWitnesses(response)
    }

    public async burnTokens(
        address: string,
        signature: string,
        tokenId: bigint,
        amount: bigint,
    ): Promise<void> {
        const request = unparseBurnTokens(address, signature, tokenId, amount)
        const response = await this.request(request)
        const txId = parseTxId(response)
        await this.waitTransaction(txId)
    }

    public async createPool(
        address: string,
        signature: string,
        baseTokenId: bigint,
        quoteTokenId: bigint,
        baseTokenAmount: bigint,
        quoteTokenAmount: bigint,
    ): Promise<void> {
        const request = unparseCreatePool(
            address,
            signature,
            baseTokenId,
            quoteTokenId,
            baseTokenAmount,
            quoteTokenAmount,
        )
        const response = await this.request(request)
        const txId = parseTxId(response)
        await this.waitTransaction(txId)
    }

    public async provideLiquidity(
        address: string,
        signature: string,
        baseTokenId: bigint,
        quoteTokenId: bigint,
        baseTokenAmount: bigint,
        quoteTokenAmountLimit: bigint,
    ): Promise<void> {
        const request = unparseProvideLiquidity(
            address,
            signature,
            baseTokenId,
            quoteTokenId,
            baseTokenAmount,
            quoteTokenAmountLimit,
        )
        const response = await this.request(request)
        const txId = parseTxId(response)
        await this.waitTransaction(txId)
    }

    public async removeLiquidity(
        address: string,
        signature: string,
        baseTokenId: bigint,
        quoteTokenId: bigint,
        baseTokenAmountLimit: bigint,
        quoteTokenAmountLimit: bigint,
        liquidityPointAmount: bigint,
    ): Promise<void> {
        const request = unparseRemoveLiquidity(
            address,
            signature,
            baseTokenId,
            quoteTokenId,
            baseTokenAmountLimit,
            quoteTokenAmountLimit,
            liquidityPointAmount,
        )
        const response = await this.request(request)
        const txId = parseTxId(response)
        await this.waitTransaction(txId)
    }

    public async buyTokens(
        address: string,
        signature: string,
        baseTokenId: bigint,
        quoteTokenId: bigint,
        baseTokenAmount: bigint,
        quoteTokenAmountLimit: bigint,
    ): Promise<void> {
        const request = unparseBuyTokens(
            address,
            signature,
            baseTokenId,
            quoteTokenId,
            baseTokenAmount,
            quoteTokenAmountLimit,
        )
        const response = await this.request(request)
        const txId = parseTxId(response)
        await this.waitTransaction(txId)
    }

    public async sellTokens(
        address: string,
        signature: string,
        baseTokenId: bigint,
        quoteTokenId: bigint,
        baseTokenAmountLimit: bigint,
        quoteTokenAmount: bigint,
    ): Promise<void> {
        const request = unparseSellTokens(
            address,
            signature,
            baseTokenId,
            quoteTokenId,
            baseTokenAmountLimit,
            quoteTokenAmount,
        )
        const response = await this.request(request)
        const txId = parseTxId(response)
        await this.waitTransaction(txId)
    }
}
