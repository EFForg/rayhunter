<script lang="ts">
    import { Manifest, ManifestEntry } from "$lib/manifest.svelte";
    import TableRow from "./ManifestTableRow.svelte";
    interface Props {
        entries: ManifestEntry[];
        current_entry: ManifestEntry | undefined;
    }
    let { entries, current_entry }: Props = $props();
</script>
<table class="table-auto text-left">
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