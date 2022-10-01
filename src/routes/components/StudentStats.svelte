<script>
    // Imports
    import TableBody from "./StudentStats/TableBody.svelte"
    import TableHead from "./StudentStats/TableHead.svelte"
    import TableTitle from "./StudentStats/TableTitle.svelte"
    
    // Student Data
    const DATA = [
        {name: "Tristan", class: "Math", completion: "100%"},
        {name: "Keeleigh", class: "Physics", completion: "60%"},
        {name: "Ella", class: "Chemistry", completion: "30%"}
    ];
</script>

<!-- Table Card -->
<div class="m-10 px-4 xl:w-3/5">
    <div class="flex-col break-words bg-white shadow-lg rounded w-full">
        <TableTitle/>
        
        <!-- Table Contents -->
        <div class="block overflow-x-auto w-full">
            <table class="items-center w-full bg-transparent border-collapse">
                <TableHead/>
                <TableBody DATA={DATA}/>
            </table>
        </div>
    </div>
</div>