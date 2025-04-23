<script lang="ts">
    import { ManifestEntry } from "$lib/manifest.svelte";
    import DownloadLink from '$lib/components/DownloadLink.svelte';
    import DeleteButton from "$lib/components/DeleteButton.svelte";
	import AnalysisStatus from "./AnalysisStatus.svelte";
	import AnalysisView from "./AnalysisView.svelte";
    let { entry, current, i }: {
        entry: ManifestEntry;
        current: boolean;
        i: number
    } = $props();

    // passing `undefined` as the locale uses the browser default
    const date_formatter = new Intl.DateTimeFormat(undefined, {
        timeStyle: "long",
        dateStyle: "short",
    });
    let normal_row_color = i % 2 == 0 ? "bg-white" : "bg-gray-100";
    let row_color = current ? "bg-green-100" : normal_row_color;
    let analysis_visible = $state(false);
</script>

<tr class="{row_color}">
    <th class="font-bold p-2 bg-blue-100" scope='row'>{entry.name}</th>
    <td class="p-2">{date_formatter.format(entry.start_time)}</td>
    <td class="p-2">{date_formatter.format(entry.last_message_time)}</td>
    <td class="p-2">{entry.qmdl_size_bytes}</td>
    <td class="p-2"><DownloadLink url={entry.getPcapUrl()} text="pcap" /></td>
    <td class="p-2"><DownloadLink url={entry.getQmdlUrl()} text="qmdl" /></td>
    <td class="p-2"><AnalysisStatus onclick={() => { analysis_visible = !analysis_visible; }} entry={entry} /></td>
    {#if current}
        <td class="p-2"></td>
    {:else}
        <td class="p-2">
            <DeleteButton
                prompt={`Are you sure you want to delete entry ${entry.name}?`}
                url={entry.getDeleteUrl()}
            />
        </td>
    {/if}
</tr>
<tr class="{normal_row_color} border-b {analysis_visible ? '' : 'collapse'}">
    <td class="font-bold p-2 bg-blue-100"></td>
    <td class="border-t border-dashed p-2" colspan="7">
        <AnalysisView {entry} />
    </td>
</tr>
