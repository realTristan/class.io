
<script>
    import { page } from '$app/stores';
    import { onMount } from "svelte";

    // Class data variable
    let Class = {"units": []};

    // When the site is loaded, get the class data from the backend
    // Test class hash: e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020
    onMount(async () => {
        await self.fetch("http://127.0.0.1:8080/class/" + $page.params.class)
        .then(response => response.json())
        .then(json => {
            Class = json;
            console.log(json);
        });
    })

    let Lessons = [];
</script>

<body 
  class="bg-white"
  style="background-size: cover; background-repeat: no-repeat; background-position: center center;"
>

    {#if Object.keys(Class).length == 0}
        <h2>INVALID CLASS</h2>
    {/if}

    {#if Class.rsl}
        <h2>Redirect to firebase login</h2>
    {/if}

    {#if Class.enable_whitelist}
        <h2>You are not whitelisted in this class!</h2>
    {/if}

    <div class="flex">
        <div>
             <!-- Navigation Menu -->
            <div class="flex mt-9 mb-7 ml-20">
                <h2 class="font-bold text-6xl text-slate-900 tracking-wider">{Class.class_name}</h2>
            </div>

            <!-- Gray Line Span -->
            <div class="w-96 mx-20 h-px bg-slate-200"></div>

            <!-- Units -->
            <h2 class="font-bold text-4xl mt-10 ml-20 text-slate-700 uppercase tracking-widest">Units</h2>
            {#each Class.units as Unit}
                <div on:click={() => Lessons = Unit.lessons} class="my-4 text-center items-center flex group h-24 w-96 hover:translate-x-10 duration-500 ease-in-out cursor-pointer">
                    <h2 class="font-bold text-2xl ml-24 text-slate-500 uppercase tracking-widest">{Unit.unit_name}</h2>
                </div>
            {/each}
        </div>

        <div class="mx-96 my-20">
            {#each Lessons as lesson}
                <div class="mt-20">
                    <a href={lesson.video} class="text-3xl font-black">{lesson.title}</a>
                    <h2 class="mt-2 text-base">{lesson.description}</h2>

                    <h2 class="mt-4 text-xl font-black">Work</h2>
                    <h2>{lesson.work}</h2>

                    <h2 class="mt-4 text-xl font-black">Work Solutions</h2>
                    <h2>{lesson.work_solutions}</h2>
                </div>
            {/each}
        </div>
        
    </div>
    

</body>
