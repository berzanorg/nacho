import { ZkProgram } from "o1js"
import { proofGenerator } from "./proof-generator.js"

export class ZkProof extends ZkProgram.Proof(proofGenerator) {}
