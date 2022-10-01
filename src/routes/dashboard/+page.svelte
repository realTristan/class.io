<svelte:head>
	<title>class.io</title>
</svelte:head>

<script>
    import ClassesCard from "./components/ClassesCard/ClassesCard.svelte";
    import NavMenu from "../components/NavMenu.svelte";
    import NewSubmissions from "./components/NewSubmissions/NewSubmissions.svelte";
    import StudentStats from "./components/StudentStats/StudentStats.svelte"
</script>

<body class="bg-slate-50">
    <main>
        <div class="flex">
            <NavMenu/>
            <div>
                <h2 class="font-black text-2xl mt-7 mx-10 text-slate-900">DASHBOARD</h2>
                <div class="flex">
                    <StudentStats/>
                    <NewSubmissions/>
                </div>
                <ClassesCard/>
            </div>
        </div>
    </main>
</body>