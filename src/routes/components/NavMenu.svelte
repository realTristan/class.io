<script>
    const NAV_MENU_ITEMS_1 = ["home", "dashboard", "settings"]
    const NAV_MENU_ITEMS_2 = ["classes", "submissions", "students"]
</script>

<!-- Navigation Menu -->
<div class="bg-white h-screen w-56 justify-between items-center shadow-lg">
    <div class="flex mt-9 mb-7 ml-6 items-center">
        <img class="rounded-full w-9 h-9 bg-slate-200" src="https://picsum.photos/500/500" alt="">
        <h2 class="font-bold text-sm text-slate-600 ml-3">Tristan Simpson</h2>
    </div>
    
    <div class="w-50 mx-6 h-px bg-slate-200"></div>
    <h2 class="font-bold text-xs mt-5 mb-3 ml-6 text-slate-500">ADMIN</h2>
    <ul class="md:flex-col md:min-w-full flex flex-col list-none">
        {#each NAV_MENU_ITEMS_1 as item}
            <li class="items-center hover:bg-slate-50 mx-4 rounded">
                <a href="/{item}" class="text-xs py-3 uppercase font-bold text-slate-600 flex mx-4">
                    <img class="mr-3" src="/images/{item}.png" alt=""/>
                    {item}
                </a>
            </li>
        {/each}
    </ul>
    <h2 class="font-bold text-xs mt-5 mb-3 ml-6 text-slate-500">CLASSES</h2>
    <ul class="md:flex-col md:min-w-full flex flex-col list-none">
        {#each NAV_MENU_ITEMS_2 as item}
            <li class="items-center hover:bg-slate-50 mx-4 rounded">
                <a href="/{item}" class="text-xs py-3 uppercase font-bold text-slate-600 flex mx-4">
                    <img class="mr-3" src="/images/{item}.png" alt=""/>
                    {item}
                </a>
            </li>
        {/each}
    </ul>
</div>