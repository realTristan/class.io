<script>
    // Imports
    import TableBody from "./TableBody.svelte"
    import TableHead from "./TableHead.svelte"
    import TableTitle from "./TableTitle.svelte"
    
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
<div class="mb-6 mt-10 ml-10 px-4" style="width: 700px;">
    <div class="flex-col break-words bg-white shadow-lg rounded w-full">
        <TableTitle/>
        
        <!-- Table Contents -->
        <div class="block overflow-x-auto w-full">
            <table class="items-center w-full bg-transparent border-collapse mb-6">
                <TableHead/>
                <TableBody DATA={DATA}/>
            </table>
        </div>
    </div>
</div>