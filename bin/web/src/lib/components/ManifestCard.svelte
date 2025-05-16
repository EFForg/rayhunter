<script lang="ts">
    import { ManifestEntry } from "$lib/manifest.svelte";
    import DownloadLink from '$lib/components/DownloadLink.svelte';
    import DeleteButton from "$lib/components/DeleteButton.svelte";
	import AnalysisStatus from "./AnalysisStatus.svelte";
	import AnalysisView from "./AnalysisView.svelte";
    import RecordingControls from "./RecordingControls.svelte";
    let { entry, current, i, server_is_recording }: {
        entry: ManifestEntry;
        current: boolean;
        i: number;
        server_is_recording: boolean;
    } = $props();

    // passing `undefined` as the locale uses the browser default
    const date_formatter = new Intl.DateTimeFormat(undefined, {
        timeStyle: "long",
        dateStyle: "short",
    });
    let status_row_color = $derived.by(() => {
        const num_warnings = entry.get_num_warnings();
        if (num_warnings !== undefined && num_warnings > 0) {
            return "bg-red-100";
        }
        return current ? "bg-green-100" : "bg-gray-100"
    });
    let analysis_visible = $state(false);
    function toggle_analysis_visibility() {
        analysis_visible = !analysis_visible;
    }
</script>
<div class="{status_row_color} drop-shadow p-4 flex flex-col">
    <span class="">Name: {entry.name}</span>
    <span class="">Started: {date_formatter.format(entry.start_time)}</span>
    <span class="">Last Message: {date_formatter.format(entry.last_message_time)}</span>
    <span class="">Size: {entry.qmdl_size_bytes} bytes</span>
    <span class=""><AnalysisStatus onclick={toggle_analysis_visibility} entry={entry} /></span>
    <div class="flex flex-row justify-between">
        <span class=""><DownloadLink url={entry.get_pcap_url()} text="pcap" /></span>
        <span class=""><DownloadLink url={entry.get_qmdl_url()} text="qmdl" /></span>
        {#if current}
            <RecordingControls {server_is_recording} />
        {:else}
            <DeleteButton
                prompt={`Are you sure you want to delete entry ${entry.name}?`}
                url={entry.get_delete_url()}
            />
        {/if}
    </div>
    <div class="border-b {analysis_visible ? '' : 'hidden'}">
        <div class="border-t border-dashed p-2" colspan="8">
            <AnalysisView {entry} />
        </div>
    </div>
</div>
