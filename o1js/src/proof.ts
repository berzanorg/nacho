import { ZkProgram } from "o1js"
import { prover } from "./prover.js"

/**
 * The class that represents proofs generated by `prover` zk program.
 */
export class Proof extends ZkProgram.Proof(prover) {}