<script lang="ts">
    import { Manifest, ManifestEntry } from "$lib/manifest.svelte";
    import TableRow from "./ManifestTableRow.svelte";
    interface Props {
        entries: ManifestEntry[];
        current_entry: ManifestEntry | undefined;
    }
    let { entries, current_entry }: Props = $props();
</script>

<table class="table-auto border">
    <thead class="p-2">
        <tr class="bg-gray-300 p-2 m-2">
            <th scope="col">Name</th>
            <th scope="col">Date Started</th>
            <th scope="col">Date of Last Message</th>
            <th scope="col">Size (bytes)</th>
            <th scope="col">PCAP</th>
            <th scope="col">QMDL</th>
            <th scope="col">Analysis Result</th>
        </tr>
    </thead>
    <tbody>
        {#if current_entry !== undefined}
            <TableRow entry={current_entry} current={true} />
        {/if}
        {#each entries as entry}
            <TableRow entry={entry} current={false} />
        {/each}
    </tbody>
</table>
