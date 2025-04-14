<script lang="ts">
	import { AnalysisStatus } from "$lib/analysisManager.svelte";
	import { EventType } from "$lib/analysis.svelte";
	import type { ManifestEntry } from "$lib/manifest.svelte";
    let { entry, analysis_status }: {
        entry: ManifestEntry,
        analysis_status: AnalysisStatus | undefined,
    } = $props();

    let summary = $derived.by(() => {
        if (analysis_status === AnalysisStatus.Queued) {
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
    })
</script>

<p>
    {summary}
</p>

<style>
</style>
