<script>
    // Imports
    import TableBody from "./NewSubmissions/TableBody.svelte"
    import TableHead from "./NewSubmissions/TableHead.svelte"
    import TableTitle from "./NewSubmissions/TableTitle.svelte"
    
    // Student Data
    const DATA = [
        {
            name: "Tristan", 
            submission_hash: "4da0c3de19e7dc9845a87758c2cf6b94a275ff5b", 
            class: "Math", 
            date: "Saturday, October 1st, 2022", 
            marked: true
        },
        {
            name: "Keeleigh", 
            submission_hash: "4da0c3de19e7dc9845a87758c2cf6b94a275ff5b", 
            class: "Physics", 
            date: "Saturday, October 3rd, 2022", 
            marked: false
        },
        {
            name: "Bob", 
            submission_hash: "4da0c3de19e7dc9845a87758c2cf6b94a275ff5b", 
            class: "Chemistry", 
            date: "Saturday, October 2nd, 2022", 
            marked: true
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