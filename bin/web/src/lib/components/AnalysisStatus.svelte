<script lang="ts">
	import { AnalysisStatus } from "$lib/analysisManager.svelte";
	import { EventType } from "$lib/analysis.svelte";
	import type { ManifestEntry } from "$lib/manifest.svelte";
    let { entry, onclick }: {
        entry: ManifestEntry,
        onclick: () => void,
    } = $props();

    let summary = $derived.by(() => {
        if (entry.analysis_status === AnalysisStatus.Queued) {
            return 'Queued...';
        } else if (entry.analysis_status === AnalysisStatus.Running) {
            return 'Running...';
        } else if (entry.analysis_status === AnalysisStatus.Finished) {
            if (entry.analysis_report === undefined) {
                return 'Loading...';
            } else if (typeof(entry.analysis_report) === 'string') {
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
    })

    let button_class = $derived(ready ? "text-blue-600 border rounded-full px-2" : '');
</script>

<button class="{button_class} {entry.get_num_warnings() < 1 ? 'text-green-700 border-green-500 bg-green-200' : 'text-red-700 border-red-500 bg-red-200'}" disabled={!ready} {onclick}>
    {summary}
</button>

<style>
</style>
