<script>
    const TITLES = ["NAME", "STUDENTS", "SUBMISSIONS", "UNITS", "STUDENT LOGIN", "WHITELIST", "CREATED"];
</script>

<!-- Table Headers -->
<thead class="thead-light">
    <tr>
        {#each TITLES as title}
            <th class="px-6 py-3 bg-blueGray-50 text-blueGray-500 whitespace-nowrap font-semibold text-left text-sm">
                {title}
            </th>
        {/each}
    </tr>
</thead>