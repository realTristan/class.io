<script>
    // Imports
    import TableBody from "./NewSubmissions/TableBody.svelte"
    import TableHead from "./NewSubmissions/TableHead.svelte"
    import TableTitle from "./NewSubmissions/TableTitle.svelte"
    
    // Student Data
    const DATA = [
        {name: "Tristan", class: "Math", date: "Saturday, October 1st, 2022", marked: true},
        {name: "Keeleigh", class: "Physics", date: "Saturday, October 3rd, 2022", marked: false},
        {name: "Ella", class: "Chemistry", date: "Saturday, October 2nd, 2022", marked: true}
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