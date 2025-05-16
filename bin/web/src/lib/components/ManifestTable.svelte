<script lang="ts">
    import { Manifest, ManifestEntry } from "$lib/manifest.svelte";
    import TableRow from "./ManifestTableRow.svelte";
    import Card from "./ManifestCard.svelte"
    interface Props {
        entries: ManifestEntry[];
        current_entry: ManifestEntry | undefined;
        server_is_recording: boolean;
    }
    let { entries, current_entry, server_is_recording }: Props = $props();
</script>

<!--For larger screens we use a table-->
<table class="hidden table-auto text-left lg:table">
    <thead>
        <tr class="bg-gray-100 drop-shadow">
            <th class='p-2' scope="col">Name</th>
            <th class='p-2' scope="col">Started</th>
            <th class='p-2' scope="col">Last Message</th>
            <th class='p-2' scope="col">Size (bytes)</th>
            <th class='p-2' scope="col">PCAP</th>
            <th class='p-2' scope="col">QMDL</th>
            <th class='p-2' scope="col">Analysis</th>
            <th class='p-2' scope="col"></th>
        </tr>
    </thead>
    <tbody>
        {#if current_entry !== undefined}
            <TableRow entry={current_entry} current={true} i={0} />
        {/if}
        {#each entries as entry, i}
            <TableRow {entry} current={false} {i} />
        {/each}
    </tbody>
</table>
<!--For smaller screens we use cards-->
<div class="lg:hidden flex flex-col gap-2">
    {#if current_entry !== undefined}
        <Card entry={current_entry} current={true} i={0} server_is_recording={server_is_recording}/>
    {/if}
    {#each entries as entry, i}
        <Card {entry} current={false} {i} />
    {/each}
</div>