<script lang="ts">
    import { AnalysisStatus } from '$lib/analysisManager.svelte';
    import {
        EventType,
        type AnalyzerMetadata,
        type ReportMetadata,
        type AnalysisRow,
        type AnalysisReport
    } from '$lib/analysis.svelte';
    import type { ManifestEntry } from '$lib/manifest.svelte';
    let {
        report
    }: {
        report: AnalysisReport;
    } = $props();

    const date_formatter = new Intl.DateTimeFormat(undefined, {
        timeStyle: 'long',
        dateStyle: 'short'
    });

    const skipped_messages: Map<string, number> = $derived.by(() => {
        let map = new Map();
        for (const row of report.rows) {
            for (const message of row.skipped_message_reasons) {
                let count = map.get(message);
                if (count === undefined) {
                    count = 0;
                }
                map.set(message, count + 1);
            }
        }
        return map;
    });
</script>

<div>
    <p class="text-lg underline">Warnings and Informational Logs</p>
    {#if report.statistics.num_warnings === 0 && report.statistics.num_informational_logs === 0}
        <p>Nothing to show!</p>
    {:else}
        <table class="table-auto text-left">
            <thead class="p-2">
                <tr class="bg-gray-300">
                    <th class="p-2">Timestamp</th>
                    <th class="p-2">Warning</th>
                    <th class="p-2">Severity</th>
                </tr>
            </thead>
            <tbody>
                {#each report.rows as row, row_idx}
                    {#each row.analysis as analysis}
                        {@const parsed_date = new Date(analysis.timestamp)}
                        {#each analysis.events.filter((e) => e !== null) as event}
                            <tr class="even:bg-gray-200 odd:bg-white">
                                {#if event.type === EventType.Warning}
                                    {@const severity = ['Low', 'Medium', 'High'][event.severity]}
                                    {@const severity_class = [
                                        'bg-red-200',
                                        'bg-red-400',
                                        'bg-red-600'
                                    ][event.severity]}
                                    <td class="p-2">{date_formatter.format(parsed_date)}</td>
                                    <td class="p-2">{event.message}</td>
                                    <td class="p-2 {severity_class} text-center">{severity}</td>
                                {:else if event.type === EventType.Informational}
                                    <td class="p-2">{date_formatter.format(parsed_date)}</td>
                                    <td class="p-2">{event.message}</td>
                                    <td class="p-2">Info</td>
                                {/if}
                            </tr>
                        {/each}
                    {/each}
                {/each}
            </tbody>
        </table>
    {/if}
</div>
{#if report.statistics.num_skipped_packets > 0}
    <div>
        <p class="text-lg underline">Unparsed Messages</p>
        <p>
            These are due to a limitation or bug in Rayhunter's parser, and aren't ususally a
            problem.
        </p>
        <table class="table-auto text-left">
            <thead class="p-2">
                <tr class="bg-gray-300">
                    <th scope="col" class="p-2">Total Msgs Affected</th>
                    <th scope="col">Reason/Error</th>
                </tr>
            </thead>
            <tbody>
                {#each skipped_messages.entries() as [message, count]}
                    <tr class="even:bg-gray-200 odd:bg-white">
                        <td class="text-center">{count}</td>
                        <td>{message}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}
