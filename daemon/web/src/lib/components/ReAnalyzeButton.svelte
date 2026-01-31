<script lang="ts">
    import ApiRequestButton from './ApiRequestButton.svelte';
    import { AnalysisStatus, AnalysisManager } from '$lib/analysisManager.svelte';
    import type { ManifestEntry } from '$lib/manifest.svelte';

    let {
        entry,
        manager,
    }: {
        entry: ManifestEntry;
        manager: AnalysisManager;
    } = $props();

    let url = $derived(entry.get_reanalyze_url());
    let entry_name = $derived(entry.name);
    let analysis_status = $derived(entry.analysis_status);

    let is_processing = $derived(
        analysis_status === AnalysisStatus.Queued || analysis_status === AnalysisStatus.Running
    );

    async function handle_re_analyze() {
        // Update the entry directly for immediate UI feedback
        entry.analysis_status = AnalysisStatus.Queued;
        entry.analysis_report = undefined;
        manager.set_queued_status(entry_name);
    }
</script>

<ApiRequestButton
    {url}
    label="Re-analyze"
    loadingLabel="Analyzing..."
    disabled={is_processing}
    variant="blue"
    onclick={handle_re_analyze}
    ariaLabel="re-analyze"
    errorMessage="Error re-analyzing recoding"
>
    {#snippet icon()}
        <svg style="width:20px;height:20px" viewBox="0 0 24 24">
            <path
                fill="white"
                d="M12,18A6,6 0 0,1 6,12C6,11 6.25,10.03 6.7,9.2L5.24,7.74C4.46,8.97 4,10.43 4,12A8,8 0 0,0 12,20V23L16,19L12,15M12,4V1L8,5L12,9V6A6,6 0 0,1 18,12C18,13 17.75,13.97 17.3,14.8L18.76,16.26C19.54,15.03 20,13.57 20,12A8,8 0 0,0 12,4Z"
            />
        </svg>
    {/snippet}
</ApiRequestButton>
