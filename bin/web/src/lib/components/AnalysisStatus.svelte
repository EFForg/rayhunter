<script lang="ts">
    import { AnalysisStatus } from '$lib/analysisManager.svelte';
    import { EventType } from '$lib/analysis.svelte';
    import type { ManifestEntry } from '$lib/manifest.svelte';
    let {
        entry,
        onclick,
        analysis_visible
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
                let num_warnings = 0;
                for (let row of entry.analysis_report.rows) {
                    for (let analysis of row.analysis) {
                        for (let event of analysis.events) {
                            if (event.type === EventType.Warning) {
                                num_warnings += 1;
                            }
                        }
                    }
                }
                return `${num_warnings} warnings`;
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

    let button_class = $derived(ready ? 'text-blue-600 border rounded-full px-2' : '');
</script>

<button class="flex flex-row gap-1 lg:gap-2" disabled={!ready} {onclick}>
    <span
        class="{button_class} {(entry.get_num_warnings() || 0) < 1
            ? 'text-green-700 border-green-500 bg-green-200'
            : 'text-red-700 border-red-500 bg-red-200'}">{summary}</span
    >
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
