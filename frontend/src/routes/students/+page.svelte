<svelte:head>
	<title>Students</title>
</svelte:head>

<script>
    import NavMenu from "../components/NavMenu.svelte";
</script>

<body class="bg-slate-50">
    <main> <div class="flex">
        <NavMenu/>
        <div>
            <h2 class="font-black text-2xl mt-7 mx-10 text-slate-900">STUDENTS</h2>
            <div class="mt-5 mx-12 flex flex-wrap items-stretch">
                <img class="ml-5 mt-3.5 absolute" src="/images/icons/@.png" alt=""/>
                <input 
                    type="text" 
                    placeholder="Search for student" 
                    class="border-0 px-3 py-3 bg-white rounded-full text-sm shadow outline-none focus:outline-none pl-12 w-full"
                >
            </div>
            {#each Array(5) as _}
                <div class="border shadow-lg rounded-md p-4 max-w-sm w-full mx-16 mt-10 bg-white">
                    <div class="animate-pulse flex space-x-4">
                        <div class="rounded-full bg-slate-700 h-10 w-10"></div>
                        <div class="flex-1 space-y-6 py-1">
                            <div class="h-2 bg-slate-700 rounded"></div>
                            <div class="space-y-3">
                                <div class="grid grid-cols-3 gap-4">
                                    <div class="h-2 bg-slate-700 rounded col-span-2"></div>
                                    <div class="h-2 bg-slate-700 rounded col-span-1"></div>
                                </div>
                                <div class="h-2 bg-slate-700 rounded"></div>
                            </div>
                        </div>
                    </div>
                </div>
            {/each}
        </div>

    </div> </main>
</body>