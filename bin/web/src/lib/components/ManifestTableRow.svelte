<script lang="ts">
    import { ManifestEntry } from '$lib/manifest.svelte';
    import DownloadLink from '$lib/components/DownloadLink.svelte';
    import DeleteButton from '$lib/components/DeleteButton.svelte';
    import AnalysisStatus from './AnalysisStatus.svelte';
    import AnalysisView from './AnalysisView.svelte';
    let {
        entry,
        current,
        i
    }: {
        entry: ManifestEntry;
        current: boolean;
        i: number;
    } = $props();

    // passing `undefined` as the locale uses the browser default
    const date_formatter = new Intl.DateTimeFormat(undefined, {
        timeStyle: 'long',
        dateStyle: 'short'
    });
    let alternating_row_color = $derived(i % 2 == 0 ? 'bg-white' : 'bg-gray-100');
    let status_row_color = $derived.by(() => {
        const num_warnings = entry.get_num_warnings();
        if (num_warnings !== undefined && num_warnings > 0) {
            return 'bg-red-100';
        }
        return current ? 'bg-green-100' : alternating_row_color;
    });
    let analysis_visible = $state(false);
    function toggle_analysis_visibility() {
        analysis_visible = !analysis_visible;
    }
</script>

<tr class="{status_row_color} drop-shadow">
    <td class="p-2">{entry.name}</td>
    <td class="p-2">{date_formatter.format(entry.start_time)}</td>
    <td class="p-2"
        >{(entry.last_message_time && date_formatter.format(entry.last_message_time)) || 'N/A'}</td
    >
    <td class="p-2">{entry.get_readable_qmdl_size()}</td>
    <td class="p-2"><DownloadLink url={entry.get_pcap_url()} text="pcap" /></td>
    <td class="p-2"><DownloadLink url={entry.get_qmdl_url()} text="qmdl" /></td>
    <td class="p-2"><DownloadLink url={entry.get_zip_url()} text="zip" /></td>
    <td class="p-2"
        ><AnalysisStatus onclick={toggle_analysis_visibility} {entry} {analysis_visible} /></td
    >
    {#if current}
        <td class="p-2"></td>
    {:else}
        <td class="p-2">
            <DeleteButton
                prompt={`Are you sure you want to delete entry ${entry.name}?`}
                url={entry.get_delete_url()}
            />
        </td>
    {/if}
</tr>
<tr class="{alternating_row_color} border-b {analysis_visible ? '' : 'hidden'}">
    <td class="border-t border-dashed p-2" colspan="9">
        <AnalysisView {entry} />
    </td>
</tr>
