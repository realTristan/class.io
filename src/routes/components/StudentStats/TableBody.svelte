<script>
    export let DATA;
</script>

<!-- Table Body Data -->
<tbody>
    {#each DATA as student}
        <tr>
            <!-- Student Name -->
            <th class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4 text-left">
                {student.name}
            </th>

            <!-- Student Class -->
            <td class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4">
                {student.class}
            </td>

            <!-- Student Work Completion -->
            <td class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4">
                <div class="flex items-center">
                    <span class="mr-2">{student.completion}</span>
                    <div class="relative w-full">
                        <div class="overflow-hidden h-2 text-xs flex rounded bg-red-400">
                        <div 
                            style="width: {student.completion};"
                            class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-green-500">
                        </div>
                    </div>
                </div>
            </td>
            
        </tr>
    {/each}
</tbody>