<script lang="ts">
    import { ManifestEntry } from "$lib/manifest";
    import DownloadLink from '$lib/components/DownloadLink.svelte';
    let { entry, current }: {
        entry: ManifestEntry;
        current: boolean;
    } = $props();

    let row_class = current ? "current" : "";
</script>

<tr>
    <th scope='row'>{entry.name}</th>
    <td>{entry.start_time}</td>
    <td>{entry.last_message_time}</td>
    <td>{entry.qmdl_size_bytes}</td>
    <td><DownloadLink url={entry.getPcapUrl()} text="pcap" /></td>
    <td><DownloadLink url={entry.getQmdlUrl()} text="qmdl" /></td>
    <td>N/A</td>
</tr>

<style>
    th {
        @apply font-bold p-2 border-b bg-blue-100;
    }

    td {
        @apply p-2 border-b;
    }

    tr {
        @apply even:bg-gray-100;
    }
</style>
