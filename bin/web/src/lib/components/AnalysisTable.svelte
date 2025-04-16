<script lang="ts">
	import { AnalysisStatus } from "$lib/analysisManager.svelte";
	import { EventType, type AnalyzerMetadata, type ReportMetadata, type AnalysisRow, type AnalysisReport } from "$lib/analysis.svelte";
	import type { ManifestEntry } from "$lib/manifest.svelte";
    let { report }: {
        report: AnalysisReport,
    } = $props();

    const date_formatter = new Intl.DateTimeFormat(undefined, {
        timeStyle: "long",
        dateStyle: "short",
    });
</script>

<p class="text-lg underline">Warnings</p>
<table class="table-auto text-left border">
    <thead class="p-2">
        <tr class="bg-gray-300">
            <th scope="col">Timestamp</th>
            <th scope="col">Warning</th>
            <th scope="col">Severity</th>
        </tr>
    </thead>
    <tbody>
        {#each report.rows as row, row_idx}
            {#each row.analysis as analysis}
                {@const parsed_date = new Date(analysis.timestamp)}
                {@const warnings = analysis.events.filter(e => e.type === EventType.Warning)}
                {#each warnings as warning}
                    {@const severity = ['Low', 'Medium', 'High'][warning.severity]}
                    {@const severity_class = ['bg-red-200', 'bg-red-400', 'bg-red-600'][warning.severity]}
                    <tr class="even:bg-gray-400 border-b">
                        <th class="p-2">{date_formatter.format(parsed_date)}</th>
                        <td class="p-2">{warning.message}</td>
                        <td class="p-2 {severity_class}">{severity}</td>
                    </tr>
                {/each}
            {/each}
        {/each}
    </tbody>
</table>
