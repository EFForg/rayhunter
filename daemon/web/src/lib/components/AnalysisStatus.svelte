<script lang="ts">
    import { AnalysisStatus } from '$lib/analysisManager.svelte';
    import type { ManifestEntry } from '$lib/manifest.svelte';
    import Spinner from './Spinner.svelte';
    let {
        entry,
        onclick,
        analysis_visible,
    }: {
        entry: ManifestEntry;
        onclick: () => void;
        analysis_visible: boolean;
    } = $props();

    let summary = $derived.by(() => {
        if (entry.analysis_status === AnalysisStatus.Queued) {
            return 'Queued...';
        } else if (entry.analysis_status === AnalysisStatus.Running) {
            return 'Running...';
        } else if (entry.analysis_status === AnalysisStatus.Finished) {
            if (entry.analysis_report === undefined) {
                return 'Loading...';
            } else if (typeof entry.analysis_report === 'string') {
                return entry.analysis_report;
            } else {
                return `${entry.analysis_report.statistics.num_warnings} warnings`;
            }
        } else {
            return 'Loading...';
        }
    });

    let ready = $derived.by(() => {
        let finished = entry.analysis_status === AnalysisStatus.Finished;
        let report_available = entry.analysis_report !== undefined;
        return finished && report_available;
    });

    let button_class = $derived.by(() => {
        if (!ready) {
            return 'text-gray-700 dark:text-gray-300';
        } else if ((entry.get_num_warnings() || 0) < 1) {
            return 'text-green-700 dark:text-green-300 border-green-500 dark:border-green-700 bg-green-200 dark:bg-green-950 text-blue-600 border rounded-full px-2';
        } else {
            return 'text-red-700 dark:text-red-300 border-red-500 dark:border-red-700 bg-red-200 dark:bg-red-950 text-blue-600 border rounded-full px-2';
        }
    });
</script>

<button class="flex flex-row gap-1 lg:gap-2" disabled={!ready} {onclick}>
    <span class="flex flex-row items-center gap-1">
        {#if entry.analysis_status === AnalysisStatus.Queued || entry.analysis_status === AnalysisStatus.Running || (entry.analysis_status === AnalysisStatus.Finished && entry.analysis_report === undefined)}
            <Spinner class="h-4 w-4 text-blue-600 dark:text-blue-400" />
        {/if}
        <span class={button_class}>{summary}</span>
    </span>
    <svg
        class="w-6 h-6 text-gray-800 dark:text-gray-200 transition-transform {analysis_visible
            ? 'rotate-180'
            : ''}"
        aria-hidden="true"
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        fill="none"
        viewBox="0 0 24 24"
    >
        <path
            stroke="currentColor"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="m19 9-7 7-7-7"
        />
    </svg>
</button>
