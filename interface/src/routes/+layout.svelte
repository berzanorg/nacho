<script>
    import Footer from "$lib/components/Footer.svelte"
    import Header from "$lib/components/Header.svelte"
    import Main from "$lib/components/Main.svelte"
    import Seo from "$lib/components/Seo.svelte"
    import "../app.css"
</script>

<Seo />

<Header />
<Main>
    <slot />
</Main>
<Footer />
