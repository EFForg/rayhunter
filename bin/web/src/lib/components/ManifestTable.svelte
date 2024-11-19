<script lang="ts">
    import { Manifest, ManifestEntry } from "$lib/manifest";
    import TableRow from "./ManifestTableRow.svelte";
    interface Props {
        manifest: Manifest;
    }
    let { manifest }: Props = $props();
</script>

<table>
    <thead>
        <tr>
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
        {#if manifest.current_entry !== undefined}
            <TableRow entry={manifest.current_entry} current={true} />
        {/if}
        {#each manifest.entries as entry}
            <TableRow entry={entry} current={false} />
        {/each}
    </tbody>
</table>

<style>
    table {
        @apply table-auto border;
    }

    th {
        @apply bg-gray-300 p-2;
    }
</style>
