<script lang="ts">
    import { ManifestEntry } from "$lib/manifest.svelte";
    import { get_manifest, get_system_stats } from "$lib/utils.svelte";
    import ManifestTable from "$lib/components/ManifestTable.svelte";
    import type { SystemStats } from "$lib/systemStats";
    import { AnalysisManager } from "$lib/analysisManager.svelte";
	import SystemStatsTable from "$lib/components/SystemStatsTable.svelte";
	import ControlBar from "$lib/components/ControlBar.svelte";

    let manager: AnalysisManager = new AnalysisManager();
    let loaded = $state(false);
    let recording = $state(false);
    let entries: ManifestEntry[] = $state([]);
    let current_entry: ManifestEntry | undefined = $state(undefined);
    let system_stats: SystemStats | undefined = $state(undefined);
    $effect(() => {
        const interval = setInterval(async () => {
            await manager.update();
            let new_manifest = await get_manifest();
            await new_manifest.set_analysis_status(manager);
            entries = new_manifest.entries;
            current_entry = new_manifest.current_entry;
            recording = current_entry !== undefined;

            system_stats = await get_system_stats();
            loaded = true;
        }, 1000);

        return () => clearInterval(interval);
    })
</script>

<h1 class="m-4 xl:m-8 text-4xl font-extrabold">Rayhunter Dashboard</h1>
<div class="m-4 xl:m-8 flex flex-col gap-4">
{#if loaded}
    <SystemStatsTable stats={system_stats!} />
    <ControlBar server_is_recording={recording} />
    <ManifestTable entries={entries} current_entry={current_entry} />
{:else}
    <p>Loading...</p>
{/if}
</div>
