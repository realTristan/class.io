<svelte:head>
	<title>class.io</title>
</svelte:head>

<script>
    import NavMenu from "./components/NavMenu.svelte";
    import NewSubmissions from "./components/NewSubmissions.svelte";
    import StudentStats from "./components/StudentStats.svelte"
</script>

<body class="bg-slate-50">
    <main>
        <div class="flex">
            <NavMenu/>
            <div>
                <h2 class="font-black text-2xl mt-7 mx-10 text-slate-900">DASHBOARD</h2>
                <StudentStats/>
                <NewSubmissions/>
            </div>
        </div>
    </main>
</body>