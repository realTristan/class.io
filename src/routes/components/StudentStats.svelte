<script>
    // Imports
    import TableBody from "./StudentStats/TableBody.svelte"
    import TableHead from "./StudentStats/TableHead.svelte"
    import TableTitle from "./StudentStats/TableTitle.svelte"
    
    // Student Data
    const DATA = [
        {
            name: "Tristan", 
            id: "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3", 
            class: "Math", 
            completion: "100%"
        },
        {
            name: "Keeleigh", 
            id: "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3", 
            class: "Physics", 
            completion: "60%"
        },
        {
            name: "Bob", 
            id: "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3", 
            class: "Chemistry", 
            completion: "30%"
        }
    ];
</script>

<!-- Table Card -->
<div class="m-10 px-4">
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