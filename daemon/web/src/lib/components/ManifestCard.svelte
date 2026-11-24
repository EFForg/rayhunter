<script lang="ts">
    import type { SvelteSet } from 'svelte/reactivity';
    import { ManifestEntry } from '$lib/manifest.svelte';
    import { AnalysisManager } from '$lib/analysisManager.svelte';
    import DownloadLink from '$lib/components/DownloadLink.svelte';
    import DeleteButton from '$lib/components/DeleteButton.svelte';
    import AnalysisStatus from './AnalysisStatus.svelte';
    import AnalysisView from './AnalysisView.svelte';
    import RecordingControls from './RecordingControls.svelte';
    let {
        entry,
        current,
        server_is_recording,
        manager,
        selected,
    }: {
        entry: ManifestEntry;
        current: boolean;
        server_is_recording: boolean;
        manager: AnalysisManager;
        // omitted for the current recording, which can't be bulk-selected
        selected?: SvelteSet<string>;
    } = $props();

    // passing `undefined` as the locale uses the browser default
    const date_formatter = new Intl.DateTimeFormat(undefined, {
        timeStyle: 'long',
        dateStyle: 'short',
    });
    let status_row_color = $derived.by(() => {
        const num_warnings = entry.get_num_warnings();
        if (num_warnings !== undefined && num_warnings > 0) {
            return 'bg-red-100 dark:bg-red-900/30';
        }
        return current ? 'bg-green-100 dark:bg-green-900/30' : 'bg-gray-100 dark:bg-gray-800';
    });
    let status_border_color = $derived.by(() => {
        const num_warnings = entry.get_num_warnings();
        if (num_warnings !== undefined && num_warnings > 0) {
            return 'border-red-100 dark:border-transparent';
        }
        return current
            ? 'border-green-100 dark:border-transparent'
            : 'border-gray-100 dark:border-gray-800';
    });
    let analysis_visible = $state(false);
    function toggle_analysis_visibility() {
        analysis_visible = !analysis_visible;
    }
</script>

<div
    class="{status_row_color} {status_border_color} drop-shadow-sm p-4 flex flex-col gap-2 border rounded-md flex-1 overflow-x-auto overflow-y-hidden"
>
    {#if current}
        <div class="flex flex-row justify-between gap-2">
            <span class="text-xl mb-2">Current Recording</span>
            <span class=""
                ><AnalysisStatus
                    onclick={toggle_analysis_visibility}
                    {entry}
                    {analysis_visible}
                /></span
            >
        </div>
    {/if}
    <div class="flex flex-col">
        <div class="flex flex-row justify-between">
            {#if selected && !current}
                <label class="font-bold flex flex-row items-center gap-2">
                    <input
                        type="checkbox"
                        checked={selected.has(entry.name)}
                        onchange={(e) =>
                            e.currentTarget.checked
                                ? selected.add(entry.name)
                                : selected.delete(entry.name)}
                        class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 dark:border-gray-600 rounded-sm"
                    />
                    ID: {entry.name}
                </label>
            {:else}
                <span class="font-bold">ID: {entry.name}</span>
            {/if}
            {#if !current}
                <span class=""
                    ><AnalysisStatus
                        onclick={toggle_analysis_visibility}
                        {entry}
                        {analysis_visible}
                    /></span
                >
            {/if}
        </div>
        <span class="">{entry.get_readable_qmdl_size()}</span>
    </div>
    <div class="flex flex-col">
        <span class="">Start: {date_formatter.format(entry.start_time)}</span>
        <span class=""
            >Last Message: {(entry.last_message_time &&
                date_formatter.format(entry.last_message_time)) ||
                'N/A'}</span
        >
    </div>
    {#if entry.stop_reason}
        <div
            class="bg-yellow-50 dark:bg-yellow-950 border border-yellow-300 dark:border-yellow-700 rounded-sm p-2 text-yellow-800 dark:text-yellow-200 text-sm"
        >
            {entry.stop_reason}
        </div>
    {/if}
    <div class="flex flex-row justify-between lg:justify-end gap-1 mt-2 overflow-x-auto">
        <DownloadLink url={entry.get_pcap_url()} text="pcap" full_button />
        <DownloadLink url={entry.get_qmdl_url()} text="qmdl" full_button />
        <DownloadLink url={entry.get_zip_url()} text="zip" full_button />
        {#if current}
            <RecordingControls {server_is_recording} />
        {:else}
            <DeleteButton
                prompt={`Are you sure you want to delete entry ${entry.name}?`}
                url={entry.get_delete_url()}
                name={entry.name}
            />
        {/if}
    </div>
    <div class="border-b border-gray-200 dark:border-gray-700 {analysis_visible ? '' : 'hidden'}">
        <AnalysisView {entry} {manager} {current} />
    </div>
</div>
