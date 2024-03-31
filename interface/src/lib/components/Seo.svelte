<script lang="ts">
    import { page } from "$app/stores"

    const title = "Nacho by Berzan"
    const description = "Nacho is a zk rollup where you can bridge and swap tokens."
    const url = `https://nacho.berzan.org${$page.url.pathname}`
    const keywords = "berzan, nacho, zk rollup, zk bridge, zk dex"
    const twitter = "@berzanorg"
    const image = "https://nacho.berzan.org/og.png"
    const imageWidth = "1200"
    const imageHeight = "630"
</script>

<svelte:head>
    <title>{title}</title>
    <meta name="description" content={description} />
    <link rel="canonical" href={url} />
    <meta name="keywords" content={keywords} />
    <meta property="og:locale" content="en" />
    <meta property="og:type" content="website" />
    <meta property="og:url" content={url} />
    <meta property="og:site_name" content="Nacho by Berzan" />
    <meta property="og:title" content={title} />
    <meta property="og:description" content={description} />
    <meta property="og:image" content={image} />
    <meta property="og:image:width" content={imageWidth} />
    <meta property="og:image:height" content={imageHeight} />
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:site" content={twitter} />
    <meta name="twitter:creator" content={twitter} />
    <meta name="twitter:title" content={title} />
    <meta name="twitter:url" content={url} />
    <meta name="twitter:description" content={description} />
    <meta name="twitter:image:src" content={image} />
</svelte:head>
