<script>
    // Imports
    import TableBody from "./TableBody.svelte"
    import TableHead from "./TableHead.svelte"
    import TableTitle from "./TableTitle.svelte"
    
    // Class Data
    var data = [
        {
            name: "Math", 
            class_hash: "8d767bf5b72373d12f0efd4406677e9ed076f592", 
            units: "6",
            students: "84",
            created: "Saturday, October 1st, 2022",
            submissions: "24",
            whitelist: false,
            rsl: true
        },
        {
            name: "Physics", 
            class_hash: "8d767bf5b72373d12f0efd4406677e9ed076f592", 
            units: "12",
            students: "21",
            created: "Saturday, October 2nd, 2022",
            submissions: "0",
            whitelist: true,
            rsl: false
        },
        {
            name: "Chemistry", 
            class_hash: "8d767bf5b72373d12f0efd4406677e9ed076f592", 
            units: "8",
            students: "94",
            created: "Saturday, October 7th, 2022",
            submissions: "14",
            whitelist: false,
            rsl: true
        },
    ];

    // The updateData() function is used to
    // create a new class.
    function updateData() {
        // If the user already has the max amount
        // of classes, return
        if (data.length >= 7) { return; }

        // Else, update the data, inserting the new class
        data = [...data, {
            name: "New Class", 
            class_hash: "8d767bf5b72373d12f0efd4406677e9ed076f592", 
            units: "0",
            students: "0",
            created: "Saturday, October 8th, 2022",
            submissions: "0",
            whitelist: false,
            rsl: false
        }]
        window.location.href='/class/8d767bf5b72373d12f0efd4406677e9ed076f592'
    }
</script>

<!-- Table Card -->
<div class="mb-6 mt-10 ml-10 px-4">
    <div class="flex-col break-words bg-white shadow-lg rounded w-full">
        <TableTitle/>
        
        <!-- Table Contents -->
        <div class="block overflow-x-auto w-full">
            <table class="items-center w-full bg-transparent border-collapse">
                <TableHead/>
                <TableBody DATA={data}/>
            </table>
        </div>

        <!-- Create a new class button -->
        <div class="justify-center items-center text-center">
            <button
                on:click={() => updateData()}
                type="button"
                class="
                    bg-indigo-500 text-white hover:bg-indigo-600 active:bg-indigo-500 text-xs font-bold
                    my-10 rounded ease-linear transition-all duration-150 h-10 w-96"
                >
                {#if (data.length < 7)}
                    CREATE NEW
                {:else}
                    MAXIMUM CLASSES REACHED
                {/if}
            </button>
        </div>
    </div>
</div>