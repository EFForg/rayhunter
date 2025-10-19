<script lang="ts">
    import { ManifestEntry } from '$lib/manifest.svelte';
    import { AnalysisManager } from '$lib/analysisManager.svelte';
    import { screenIsLgUp } from '$lib/stores/breakpoint';
    import TableRow from './ManifestTableRow.svelte';
    import Card from './ManifestCard.svelte';
    interface Props {
        entries: ManifestEntry[];
        server_is_recording: boolean;
        manager: AnalysisManager;
    }
    let { entries, server_is_recording, manager }: Props = $props();
</script>

<!--For larger screens we use a table-->
{#if $screenIsLgUp}
    <table class="table-auto text-left table">
        <thead>
            <tr class="bg-gray-100 drop-shadow">
                <th class="p-2" scope="col">ID</th>
                <th class="p-2" scope="col">Started</th>
                <th class="p-2" scope="col">Last Message</th>
                <th class="p-2" scope="col">Size</th>
                <th class="p-2" scope="col">Download</th>
                <th class="p-2" scope="col">Analysis</th>
                <th class="p-2" scope="col"></th>
            </tr>
        </thead>
        <tbody>
            {#each entries as entry, i}
                <TableRow {entry} current={false} {i} {manager} />
            {/each}
        </tbody>
    </table>
{:else}
    <!--For smaller screens we use cards-->
    <div class="flex flex-col gap-4">
        {#each entries as entry}
            <Card {entry} current={false} {server_is_recording} {manager} />
        {/each}
    </div>
{/if}
