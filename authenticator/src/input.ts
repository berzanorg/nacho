export type Input =
    | {
          kind: "burnTokens"
          signature: string
          address: string
          tokenId: bigint
          tokenAmount: bigint
      }
    | {
          kind: "createPool"
          signature: string
          address: string
          baseTokenId: bigint
          quoteTokenId: bigint
          baseTokenAmount: bigint
          quoteTokenAmount: bigint
      }
    | {
          kind: "provideLiquidity"
          signature: string
          address: string
          baseTokenId: bigint
          quoteTokenId: bigint
          baseTokenAmount: bigint
          quoteTokenAmountLimit: bigint
      }
    | {
          kind: "removeLiquidity"
          signature: string
          address: string
          baseTokenId: bigint
          quoteTokenId: bigint
          baseTokenAmountLimit: bigint
          quoteTokenAmountLimit: bigint
          liquidityPoints: bigint
      }
    | {
          kind: "buyTokens"
          signature: string
          address: string
          baseTokenId: bigint
          quoteTokenId: bigint
          baseTokenAmount: bigint
          quoteTokenAmountLimit: bigint
      }
    | {
          kind: "sellTokens"
          signature: string
          address: string
          baseTokenId: bigint
          quoteTokenId: bigint
          baseTokenAmountLimit: bigint
          quoteTokenAmount: bigint
      }
    | {
          kind: "mistaken"
      }
