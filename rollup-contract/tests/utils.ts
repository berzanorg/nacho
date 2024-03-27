import { PrivateKey } from "o1js"

export const generateKeypair = () => {
    const privateKey = PrivateKey.random()

    return {
        privateKey,
        publicKey: privateKey.toPublicKey(),
    }
}
