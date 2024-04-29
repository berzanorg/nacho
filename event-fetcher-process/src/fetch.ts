import { BridgeContract } from "nacho-bridge-contract"
import { Deposit, Withdrawal } from "nacho-common-o1js"
import { UInt32 } from "o1js"

export const fetchDepositedEvents = async (fromBlock: number, bridgeContract: BridgeContract) => {
    const allEvents = await bridgeContract.fetchEvents(UInt32.from(fromBlock))

    const last_fetched_block = allEvents
        .filter(({ type }) => type === "deposited")
        .map((e) => Number(e.blockHeight.toBigint()))
        .reduce((a, b) => (a > b ? a : b), 0)

    const events = allEvents
        .filter(({ type }) => type === "deposited")
        .map(({ event }) => event.data as unknown as Deposit)

    return { last_fetched_block, events }
}

export const fetchWithdrawnEvents = async (fromBlock: number, bridgeContract: BridgeContract) => {
    const allEvents = await bridgeContract.fetchEvents(UInt32.from(fromBlock))

    const last_fetched_block = allEvents
        .filter(({ type }) => type === "withdrawn")
        .map((e) => Number(e.blockHeight.toBigint()))
        .reduce((a, b) => (a > b ? a : b), 0)

    const events = allEvents
        .filter(({ type }) => type === "withdrawn")
        .map(({ event }) => event.data as unknown as Withdrawal)

    return { last_fetched_block, events }
}
