<script>
    import { wallet } from "$lib/wallet.svelte"
</script>

<header class="sticky top-0 h-16 px-4 items-center flex justify-between">
    <a href="/" class="font-semibold">Nacho</a>
    <button
        class="bg-yellow-200 hover:scale-95 active:scale-105 duration-200 px-4 h-9 text-neutral-900 font-semibold"
        onclick={wallet.isConnected ? wallet.disconnect : wallet.connect}
    >
        {#if wallet.isConnected}
            Disconnect
        {:else}
            Connect
        {/if}
    </button>
</header>
