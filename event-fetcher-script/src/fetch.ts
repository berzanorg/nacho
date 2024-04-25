import { BridgeContract } from "nacho-bridge-contract"
import { Deposit, Withdrawal } from "nacho-common-o1js"
import { UInt32 } from "o1js"

export const fetchDepositedEvents = async (fromBlock: number, bridgeContract: BridgeContract) => {
    const allEvents = await bridgeContract.fetchEvents(UInt32.from(fromBlock))

    const depositedEvents = allEvents
        .filter(({ event, type }) => type === "deposited")
        .map(({ event }) => event.data as unknown as Deposit)

    return depositedEvents
}

export const fetchWithdrawnEvents = async (fromBlock: number, bridgeContract: BridgeContract) => {
    const events = await bridgeContract.fetchEvents(UInt32.from(fromBlock))

    const withdrawnEvents = events
        .filter(({ event, type }) => type === "withdrawn")
        .map(({ event }) => event.data as unknown as Withdrawal)

    return withdrawnEvents
}
