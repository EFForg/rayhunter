<script lang="ts">
    import { Manifest, ManifestEntry } from "$lib/manifest.svelte";
    import { get_manifest, get_system_stats } from "$lib/utils.svelte";
    import ManifestTable from "$lib/components/ManifestTable.svelte";
    import { onMount } from "svelte";
    import type { SystemStats } from "$lib/systemStats";
    import { AnalysisManager } from "$lib/analysisManager.svelte";
    import { writable, readable, type Readable, type Writable } from "svelte/store";
	import RecordingControls from "$lib/components/RecordingControls.svelte";

    let manager: AnalysisManager = new AnalysisManager();
    let loaded = $state(false);
    let recording = $state(false);
    let entries: ManifestEntry[] = $state([]);
    let current_entry: ManifestEntry | undefined = $state(undefined);
    let system_stats: SystemStats | undefined = $state(undefined);
    $effect(() => {
        const interval = setInterval(async () => {
            loaded = true;
            await manager.update();
            let new_manifest = await get_manifest();
            await new_manifest.set_analysis_status(manager);
            entries = new_manifest.entries;
            current_entry = new_manifest.current_entry;
            recording = current_entry !== undefined;

            system_stats = await get_system_stats();
        }, 3000);

        return () => clearInterval(interval);
    })
</script>

<div class="p-8">
{#if loaded}
    <RecordingControls currently_recording={recording} />
    <ManifestTable entries={entries} current_entry={current_entry} />
{:else}
    <p>Loading...</p>
{/if}
</div>
