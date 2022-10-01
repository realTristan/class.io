<script>
    export let DATA;
</script>

<!-- Table Body Data -->
<tbody>
    {#each DATA as student}
    <tr class="hover:bg-slate-50 cursor-pointer" onclick="window.location='/submission/{student.submission_hash}';">
            <!-- Student Name -->
            <th class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4 text-left">
                {student.name}
            </th>

            <!-- Student Class -->
            <td class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4">
                {student.class}
            </td>

            <!-- Date -->
            <td class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4">
                {student.date}
            </td>

            <!-- Status -->
            <td class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4">
                {#if student.marked}
                    <div class="flex">
                        <div class="rounded-full bg-green-500 w-2 h-2"></div>
                        <h4 class="ml-2">Marked</h4>
                    </div>
                {:else}
                    <div class="flex">
                        <div class="rounded-full bg-red-500 w-2 h-2"></div>
                        <h4 class="ml-2">Awaiting Mark</h4>
                    </div>
                {/if}
            </td>
        </tr>
    {/each}
</tbody>