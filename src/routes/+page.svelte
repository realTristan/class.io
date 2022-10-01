<svelte:head>
	<title>class.io</title>
</svelte:head>

<script>
    import NewSubmissions from "./components/NewSubmissions.svelte";
    import StudentStats from "./components/StudentStats.svelte"
</script>

<body class="bg-slate-50">
    <main>
        <StudentStats/>
        <NewSubmissions/>   
    </main>
</body>