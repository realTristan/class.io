<svelte:head>
	<title>class.io</title>
</svelte:head>

<script>
    import LoginMenu from './components/LoginMenu.svelte';
</script>

<!-- h-screen bg-[url(images/bg.png)]-->
<body 
  class="bg-slate-800"
  style="background-size: cover; background-repeat: no-repeat; background-position: center center;"
>
    <main>
        <div class="flex justify-center items-center">
            <LoginMenu/>
        </div>
    </main>
</body>