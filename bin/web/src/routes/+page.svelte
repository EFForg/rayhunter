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

<div class="p-4 xl:px-8 bg-rayhunter-blue drop-shadow flex flex-row justify-between items-center">
    <span class="text-4xl font-extrabold text-rayhunter-green"><span class="text-white">ray</span>hunter</span>
    <div class="flex flex-row gap-4">
        <a class="flex flex-row gap-2 group" href="https://github.com/EFForg/rayhunter/issues" target="_blank">
            <span class="hidden text-white group-hover:text-gray-400 lg:flex">Report Issue</span>
            <span class="w-6">
                <svg class="fill-white group-hover:fill-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 496 512"><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path d="M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3 .3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5 .3-6.2 2.3zm44.2-1.7c-2.9 .7-4.9 2.6-4.6 4.9 .3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3 .7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3 .3 2.9 2.3 3.9 1.6 1 3.6 .7 4.3-.7 .7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3 .7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3 .7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"/></svg>
            </span>
        </a>
        <a class="flex flex-row gap-2 group" href="https://efforg.github.io/rayhunter/" target="_blank">
            <span class="hidden text-white group-hover:text-gray-400 lg:flex">Docs</span>
            <span class="w-6">
                <svg class="fill-white group-hover:fill-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path d="M96 0C43 0 0 43 0 96L0 416c0 53 43 96 96 96l288 0 32 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l0-64c17.7 0 32-14.3 32-32l0-320c0-17.7-14.3-32-32-32L384 0 96 0zm0 384l256 0 0 64L96 448c-17.7 0-32-14.3-32-32s14.3-32 32-32zm32-240c0-8.8 7.2-16 16-16l192 0c8.8 0 16 7.2 16 16s-7.2 16-16 16l-192 0c-8.8 0-16-7.2-16-16zm16 48l192 0c8.8 0 16 7.2 16 16s-7.2 16-16 16l-192 0c-8.8 0-16-7.2-16-16s7.2-16 16-16z"/></svg>
            </span>
        </a>
    </div>
</div>
<div class="m-4 xl:mx-8 flex flex-col gap-4">
{#if loaded}
    <SystemStatsTable stats={system_stats!} />
    <ControlBar server_is_recording={recording} />
    <ManifestTable entries={entries} current_entry={current_entry} server_is_recording={recording} />
{:else}
    <p>Loading...</p>
{/if}
</div>
