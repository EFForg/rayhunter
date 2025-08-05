<script lang="ts">
    import { AnalysisStatus } from '$lib/analysisManager.svelte';
    import type { ManifestEntry } from '$lib/manifest.svelte';
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
            return 'text-gray-700';
        } else if ((entry.get_num_warnings() || 0) < 1) {
            return 'text-green-700 border-green-500 bg-green-200 text-blue-600 border rounded-full px-2';
        } else {
            return 'text-red-700 border-red-500 bg-red-200 text-blue-600 border rounded-full px-2';
        }
    });
</script>

<button class="flex flex-row gap-1 lg:gap-2" disabled={!ready} {onclick}>
    <span class="flex flex-row items-center gap-1">
        {#if entry.analysis_status === AnalysisStatus.Queued || entry.analysis_status === AnalysisStatus.Running || (entry.analysis_status === AnalysisStatus.Finished && entry.analysis_report === undefined)}
            <svg
                class="animate-spin h-4 w-4 text-blue-600"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
            >
                <circle
                    class="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    stroke-width="4"
                ></circle>
                <path
                    class="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                ></path>
            </svg>
        {/if}
        <span class={button_class}>{summary}</span>
    </span>
    <svg
        class="w-6 h-6 text-gray-800 transition-transform {analysis_visible ? 'rotate-180' : ''}"
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
