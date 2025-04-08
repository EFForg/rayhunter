<script lang="ts">
	import { AnalysisStatus } from "$lib/analysisManager";
	import { EventType } from "$lib/analysis";
	import type { ManifestEntry } from "$lib/manifest";
    let { entry }: {
        entry: ManifestEntry,
    } = $props();

    let summary = $state('Loading...');
    if (entry.analysis_status === AnalysisStatus.Queued) {
        summary = 'Queued...';
    } else if (entry.analysis_status === AnalysisStatus.Running) {
        summary = 'Running...';
    } else if (entry.analysis_status === AnalysisStatus.Finished) {
        if (entry.analysis_report === undefined) {
            summary = 'Loading...';
        } else if (typeof(entry.analysis_report) === 'string') {
            summary = entry.analysis_report;
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
            summary = `${num_warnings} warnings`;
        }
    }
</script>

<p>
    {summary}
</p>

<style>
</style>
