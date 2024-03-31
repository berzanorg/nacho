export function createWallet() {
    let address: string | undefined = $state()

    async function connect() {
        if (!window.mina) {
            alert("Auro Wallet is not found.")
            return
        }

        const response = await window.mina.requestAccounts()
        address = response[0]

        window.mina.on("accountsChanged", (accounts) => {
            address = accounts[0]
        })
    }

    async function disconnect() {
        if (!window.mina) {
            alert("Auro Wallet is not found.")
            return
        }

        window.mina.removeAllListeners()

        address = undefined
    }

    return {
        get address() {
            return address
        },
        get isConnected() {
            return !!address
        },
        connect,
        disconnect,
    }
}

export const wallet = createWallet()
