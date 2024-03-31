import type { Remote } from "comlink"
import type { XaneWorker } from "./worker"

declare global {
    interface Window {
        mina?: {
            requestAccounts(): Promise<Array<string>>

            requestNetwork(): Promise<{
                chainId: "berkeley" | string
            }>

            sendTransaction(params: {
                transaction: string
                feePayer?: {
                    fee?: number
                    memo?: string
                }
            }): Promise<{ hash: string }>

            signFields(params: { message: Array<string> }): Promise<{
                data: Array<string>
                signature: string
            }>

            on<T extends "accountsChanged" | "chainChanged">(
                eventName: T,
                handler: (
                    params: T extends "accountsChanged"
                        ? Array<string>
                        : T extends "chainChanged"
                        ? string
                        : never,
                ) => void,
            ): void

            removeAllListeners(): void
        }
    }
}
