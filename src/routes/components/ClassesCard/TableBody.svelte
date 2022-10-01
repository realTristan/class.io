<script>
    export let DATA;
</script>

<!-- Table Body Data -->
<tbody>
    {#each DATA as course}
        <!-- student.id is the users unique user_hash-->
        <tr class="hover:bg-slate-50 cursor-pointer" onclick="window.location='/class/{course.class_hash}';">
            <!-- Course Name -->
            <th class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4 text-left">
                {course.name}
            </th>

            <!-- Course Students -->
            <td class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4 text-left">
                {course.students}
            </td>

            <!-- Course Submissions -->
            <td class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4 text-left">
                {course.submissions}
            </td>

            <!-- Course Units -->
            <td class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4 text-left">
                {course.units}
            </td>

            <!-- Require Student Login -->
            <td class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4">
                {#if course.rsl}
                    <div class="flex">
                        <div class="rounded-full bg-green-500 w-2 h-2"></div>
                        <h4 class="ml-2">Required</h4>
                    </div>
                {:else}
                    <div class="flex">
                        <div class="rounded-full bg-red-500 w-2 h-2"></div>
                        <h4 class="ml-2">Not Required</h4>
                    </div>
                {/if}
            </td>

            <!-- Student Whitelist -->
            <td class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4">
                {#if course.whitelist}
                    <div class="flex">
                        <div class="rounded-full bg-green-500 w-2 h-2"></div>
                        <h4 class="ml-2">Enabled</h4>
                    </div>
                {:else}
                    <div class="flex">
                        <div class="rounded-full bg-red-500 w-2 h-2"></div>
                        <h4 class="ml-2">Disabled</h4>
                    </div>
                {/if}
            </td>

            <!-- Course Creation Date -->
            <td class="border-t-0 px-6 align-middle border-l-0 border-r-0 text-xs whitespace-nowrap p-4 text-left">
                {course.created}
            </td>
        </tr>
    {/each}
</tbody>