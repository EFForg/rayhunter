<script lang="ts">
    import { Manifest, ManifestEntry } from '$lib/manifest.svelte';
    import TableRow from './ManifestTableRow.svelte';
    import Card from './ManifestCard.svelte';
    interface Props {
        entries: ManifestEntry[];
        server_is_recording: boolean;
    }
    let { entries, server_is_recording }: Props = $props();
</script>

<!--For larger screens we use a table-->
<table class="hidden table-auto text-left lg:table">
    <thead>
        <tr class="bg-gray-100 drop-shadow">
            <th class="p-2" scope="col">ID</th>
            <th class="p-2" scope="col">Started</th>
            <th class="p-2" scope="col">Last Message</th>
            <th class="p-2" scope="col">Size</th>
            <th class="p-2" scope="col">PCAP</th>
            <th class="p-2" scope="col">QMDL</th>
            <th class="p-2" scope="col">ZIP</th>
            <th class="p-2" scope="col">Analysis</th>
            <th class="p-2" scope="col"></th>
        </tr>
    </thead>
    <tbody>
        {#each entries as entry, i}
            <TableRow {entry} current={false} {i} />
        {/each}
    </tbody>
</table>
<!--For smaller screens we use cards-->
<div class="lg:hidden flex flex-col gap-4">
    {#each entries as entry}
        <Card {entry} current={false} {server_is_recording} />
    {/each}
</div>
