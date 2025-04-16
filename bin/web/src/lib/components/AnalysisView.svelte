<script lang="ts">
	import { AnalysisStatus } from "$lib/analysisManager.svelte";
	import { EventType, type AnalyzerMetadata, type ReportMetadata, type AnalysisRow } from "$lib/analysis.svelte";
	import type { ManifestEntry } from "$lib/manifest.svelte";
	import AnalysisTable from "./AnalysisTable.svelte";
    let { entry }: {
        entry: ManifestEntry,
    } = $props();

    const date_formatter = new Intl.DateTimeFormat(undefined, {
        timeStyle: "long",
        dateStyle: "short",
    });
</script>

<div class="container max-h-96 overflow-auto">
    {#if entry.analysis_report === undefined}
        <p>Report unavailable, try refreshing.</p>
    {:else if typeof(entry.analysis_report) === 'string'}
        <p>Error getting analysis report: {entry.analysis_report}</p>
    {:else}
        {@const metadata: ReportMetadata = entry.analysis_report.metadata}
        <div class="flex flex-col p-2 w-3/4">
            {#if entry.analysis_report.rows.length > 0}
                <AnalysisTable report={entry.analysis_report} />
            {:else}
                <p>No warnings to display!</p>
            {/if}
            <div>
                <p class="text-lg underline">Metadata</p>
                <p><b>Rayhunter version:</b> {metadata.rayhunter.rayhunter_version}</p>
                <p><b>Device system OS:</b> {metadata.rayhunter.system_os}</p>
                <p class="text-lg underline">Analyzers</p>
                {#each metadata.analyzers as analyzer}
                    <p><b>{analyzer.name}:</b> {analyzer.description}</p>
                {/each}
            </div>
        </div>
    {/if}
</div>
