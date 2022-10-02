<svelte:head>
	<title>class.io</title>
</svelte:head>

<script>
    import { onMount } from 'svelte'
    import { fade } from 'svelte/transition'
  import LoginMenu from './components/LoginMenu.svelte';
    
    // Track when the page loaded to use a
    // fade-in animation
    let pageLoaded = false
    onMount(() => pageLoaded = true)
</script>

<!-- h-screen bg-[url(images/bg.png)]-->
<body 
  class="bg-slate-800"
  style="background-size: cover; background-repeat: no-repeat; background-position: center center;"
>
    <main>{#if pageLoaded}
        <div transition:fade="{{ duration: 700 }}">
            <div class="flex justify-center items-center">
                <LoginMenu/>
            </div>
        </div>
    {/if}</main>
</body>