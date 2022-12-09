<svelte:head>
	<title>Submissions</title>
</svelte:head>

<script>
    import NavMenu from "../components/NavMenu.svelte";
</script>

<body class="bg-slate-50">
    <main>
        <div class="flex">
            <NavMenu/>
            <div>
                <h2 class="font-black text-2xl mt-7 mx-10 text-slate-900">SUBMISSIONS</h2>
                <div class="mt-5 mx-12 flex flex-wrap items-stretch">
                    <img class="ml-5 mt-3.5 absolute" src="/images/icons/find_submissions.png" alt=""/>
                    <input 
                        type="text" 
                        placeholder="Search" 
                        class="border-0 px-3 py-3 bg-white rounded-full text-sm shadow outline-none focus:outline-none pl-12 w-full"
                    >
                </div>
            </div>
        </div>
    </main>
</body>