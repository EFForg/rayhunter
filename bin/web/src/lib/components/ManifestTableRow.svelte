<script lang="ts">
    import { ManifestEntry } from "$lib/manifest.svelte";
    import DownloadLink from '$lib/components/DownloadLink.svelte';
	import AnalysisStatus from "./AnalysisStatus.svelte";
    let { entry, current }: {
        entry: ManifestEntry;
        current: boolean;
    } = $props();

    // bg-gray-100
    // bg-green-300
    let row_color = current ? "bg-green-300" : "even:bg-gray-100";
</script>

<tr class="{row_color} border-b">
    <th class="font-bold p-2 border-b bg-blue-100" scope='row'>{entry.name}</th>
    <td class="p-2">{entry.start_time}</td>
    <td class="p-2">{entry.last_message_time}</td>
    <td class="p-2">{entry.qmdl_size_bytes}</td>
    <td class="p-2"><DownloadLink url={entry.getPcapUrl()} text="pcap" /></td>
    <td class="p-2"><DownloadLink url={entry.getQmdlUrl()} text="qmdl" /></td>
    <td class="p-2"><AnalysisStatus analysis_status={entry.analysis_status} entry={entry} /></td>
</tr>
