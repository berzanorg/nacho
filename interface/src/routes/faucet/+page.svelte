<script>
    import { wallet } from "$lib/wallet.svelte"
</script>

<div class="flex flex-col gap-8">
    <h1 class="text-4xl leading-tight font-semibold">
        Nacho is a zk rollup where you can bridge and swap tokens.
    </h1>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
        <a
            class="bg-yellow-200 hover:scale-95 active:scale-105 duration-200 text-neutral-900 flex items-center justify-center h-14 sm:h-16 md:h-20 md:text-xl font-semibold text-lg"
            href="/swap"
        >
            Swap Tokens</a
        >
        <a
            class="bg-yellow-200 hover:scale-95 active:scale-105 duration-200 text-neutral-900 flex items-center justify-center h-14 sm:h-16 md:h-20 md:text-xl font-semibold text-lg"
            href="/faucet"
        >
            Use Faucet</a
        >
        <a
            class="bg-yellow-200 hover:scale-95 active:scale-105 duration-200 text-neutral-900 flex items-center justify-center h-14 sm:h-16 md:h-20 md:text-xl font-semibold text-lg"
            href="/bridge"
        >
            Use Bridge</a
        >
    </div>
</div>
