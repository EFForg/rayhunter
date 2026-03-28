<script lang="ts">
    import { ManifestEntry } from '$lib/manifest.svelte';
    import { get_manifest, get_system_stats, get_gps, get_config, type GpsData } from '$lib/utils.svelte';
    import ManifestTable from '$lib/components/ManifestTable.svelte';
    import Card from '$lib/components/ManifestCard.svelte';
    import type { SystemStats } from '$lib/systemStats';
    import { AnalysisManager } from '$lib/analysisManager.svelte';
    import SystemStatsTable from '$lib/components/SystemStatsTable.svelte';
    import DeleteAllButton from '$lib/components/DeleteAllButton.svelte';
    import RecordingControls from '$lib/components/RecordingControls.svelte';
    import ConfigForm from '$lib/components/ConfigForm.svelte';
    import ActionErrors from '$lib/components/ActionErrors.svelte';
    import ClockDriftAlert from '$lib/components/ClockDriftAlert.svelte';
    import LogView from '$lib/components/LogView.svelte';

    let manager: AnalysisManager = new AnalysisManager();
    let loaded = $state(false);
    let filter_threshold: boolean = $state(false);
    let entries: ManifestEntry[] = $state([]);
    let current_entry: ManifestEntry | undefined = $state(undefined);
    let system_stats: SystemStats | undefined = $state(undefined);
    let update_error: string | undefined = $state(undefined);
    let logview_shown: boolean = $state(false);
    let config_shown: boolean = $state(false);
    let gps_data: GpsData | null = $state(null);
    let gps_mode: number = $state(0);
    $effect(() => {
        get_config().then((c) => {
            gps_mode = c.gps_mode;
        });

        const interval = setInterval(async () => {
            try {
                // Don't update UI if browser tab isn't visible
                if (document.hidden) {
                    return;
                }

                await manager.update();
                let new_manifest = await get_manifest();
                await new_manifest.set_analysis_status(manager);
                entries = filter_threshold
                    ? new_manifest.entries.filter((e) => e.get_num_warnings())
                    : new_manifest.entries;

                current_entry = new_manifest.current_entry;

                system_stats = await get_system_stats();
                gps_data = await get_gps();
                update_error = undefined;
                loaded = true;
            } catch (error) {
                if (error instanceof Error) {
                    update_error = error.message;
                } else {
                    update_error = '';
                }
            }
        }, 1000);

        return () => clearInterval(interval);
    });
</script>

<LogView bind:shown={logview_shown} />
<ConfigForm bind:shown={config_shown} />
<div
    class="p-4 xl:px-8 bg-rayhunter-blue drop-shadow-sm flex flex-row justify-between items-center"
>
    <!-- https://www.w3.org/WAI/tutorials/images/decorative/ -->
    <img src="/rayhunter_text.png" alt="" class="h-10 xl:h-12" />
    <div class="flex flex-row gap-4">
        <button onclick={() => (logview_shown = true)} class="flex flex-row gap-1 group">
            <span class="hidden text-white group-hover:text-gray-400 lg:flex">Logs</span>
            <svg
                class="w-6 h-6 text-white group-hover:text-gray-400"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="currentColor"
                viewBox="0 0 24 24"
            >
                <path
                    d="M10 14H3"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                />
                <path
                    d="M10 18H3"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                />
                <path
                    d="M14 15L17.5 18L21 15"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
                <path
                    d="M3 6L13.5 6M20 6L17.75 6"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                />
                <path
                    d="M20 10L9.5 10M3 10H5.25"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                />
            </svg>
        </button>
        <button onclick={() => (config_shown = true)} class="flex flex-row gap-1 group">
            <span class="hidden text-white group-hover:text-gray-400 lg:flex">Config</span>
            <svg
                class="w-6 h-6 text-white group-hover:text-gray-400"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M21 13v-2a1 1 0 0 0-1-1h-.757l-.707-1.707.535-.536a1 1 0 0 0 0-1.414l-1.414-1.414a1 1 0 0 0-1.414 0l-.536.535L14 5.757V5a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v.757L8.293 6.464l-.536-.535a1 1 0 0 0-1.414 0L4.929 7.343a1 1 0 0 0 0 1.414l.535.536L4.757 11H4a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h.757l.707 1.707-.535.536a1 1 0 0 0 0 1.414l1.414 1.414a1 1 0 0 0 1.414 0l.536-.535L10 18.243V19a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-.757l1.707-.707.536.535a1 1 0 0 0 1.414 0l1.414-1.414a1 1 0 0 0 0-1.414l-.535-.536.707-1.707H20a1 1 0 0 0 1-1Z"
                />
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z"
                />
            </svg>
        </button>
        <div class="w-px bg-white/30 self-stretch"></div>
        <a
            class="flex flex-row gap-1 group"
            href="https://github.com/EFForg/rayhunter/issues"
            target="_blank"
        >
            <span class="hidden text-white group-hover:text-gray-400 lg:flex">Report Issue</span>
            <svg
                class="w-6 h-6 text-white group-hover:text-gray-400"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="currentColor"
                viewBox="0 0 24 24"
            >
                <path
                    fill-rule="evenodd"
                    d="M12.006 2a9.847 9.847 0 0 0-6.484 2.44 10.32 10.32 0 0 0-3.393 6.17 10.48 10.48 0 0 0 1.317 6.955 10.045 10.045 0 0 0 5.4 4.418c.504.095.683-.223.683-.494 0-.245-.01-1.052-.014-1.908-2.78.62-3.366-1.21-3.366-1.21a2.711 2.711 0 0 0-1.11-1.5c-.907-.637.07-.621.07-.621.317.044.62.163.885.346.266.183.487.426.647.71.135.253.318.476.538.655a2.079 2.079 0 0 0 2.37.196c.045-.52.27-1.006.635-1.37-2.219-.259-4.554-1.138-4.554-5.07a4.022 4.022 0 0 1 1.031-2.75 3.77 3.77 0 0 1 .096-2.713s.839-.275 2.749 1.05a9.26 9.26 0 0 1 5.004 0c1.906-1.325 2.74-1.05 2.74-1.05.37.858.406 1.828.101 2.713a4.017 4.017 0 0 1 1.029 2.75c0 3.939-2.339 4.805-4.564 5.058a2.471 2.471 0 0 1 .679 1.897c0 1.372-.012 2.477-.012 2.814 0 .272.18.592.687.492a10.05 10.05 0 0 0 5.388-4.421 10.473 10.473 0 0 0 1.313-6.948 10.32 10.32 0 0 0-3.39-6.165A9.847 9.847 0 0 0 12.007 2Z"
                    clip-rule="evenodd"
                />
            </svg>
        </a>
        <a
            class="flex flex-row gap-1 group"
            href="https://efforg.github.io/rayhunter/"
            target="_blank"
        >
            <span class="hidden text-white group-hover:text-gray-400 lg:flex">Docs</span>
            <svg
                class="w-6 h-6 text-white group-hover:text-gray-400"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M5 19V4a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v13H7a2 2 0 0 0-2 2Zm0 0a2 2 0 0 0 2 2h12M9 3v14m7 0v4"
                />
            </svg>
        </a>
        <a
            class="flex flex-row gap-1 group"
            href="https://supporters.eff.org/donate"
            target="_blank"
        >
            <span class="hidden text-white group-hover:text-gray-400 lg:flex">Donate</span>
            <svg
                class="w-6 h-6 text-white group-hover:text-gray-400"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="currentColor"
                viewBox="0 0 24 24"
            >
                <path
                    d="m12.75 20.66 6.184-7.098c2.677-2.884 2.559-6.506.754-8.705-.898-1.095-2.206-1.816-3.72-1.855-1.293-.034-2.652.43-3.963 1.537-1.31-1.108-2.67-1.571-3.962-1.537-1.515.04-2.823.76-3.72 1.855-1.806 2.2-1.924 5.821.753 8.705l6.184 7.098.245.281a.75.75 0 0 0 1.09 0l.246-.281Z"
                />
            </svg>
        </a>
    </div>
</div>
<div class="m-4 xl:mx-8 flex flex-col gap-4">
    {#if update_error !== undefined}
        <div
            class="bg-red-100 border-red-100 drop-shadow-sm p-4 flex flex-col gap-2 border rounded-md flex-1 justify-between"
        >
            <span class="text-2xl font-bold mb-2 flex flex-row items-center gap-2 text-red-600">
                <svg
                    class="w-8 h-8 text-red-600"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    fill="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        fill-rule="evenodd"
                        d="M2 12C2 6.477 6.477 2 12 2s10 4.477 10 10-4.477 10-10 10S2 17.523 2 12Zm11-4a1 1 0 1 0-2 0v5a1 1 0 1 0 2 0V8Zm-1 7a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H12Z"
                        clip-rule="evenodd"
                    />
                </svg>
                Connection Error
            </span>
            <span
                >This webpage is not currently receiving updates from your Rayhunter device. This
                could be due to loss of connection or some issue with your device.</span
            >
            {#if update_error}
                <details>
                    <summary>Error</summary>
                    <code>{update_error}</code>
                </details>
            {/if}
        </div>
    {/if}
    <ActionErrors />
    <ClockDriftAlert />
    {#if loaded}
        <div class="flex flex-col lg:flex-row gap-4">
            {#if current_entry}
                <Card
                    entry={current_entry}
                    current={true}
                    server_is_recording={!!current_entry}
                    {manager}
                />
            {:else}
                <div
                    class="bg-red-100 border-red-100 drop-shadow-sm p-4 flex flex-col gap-2 border rounded-md flex-1 justify-between"
                >
                    <span
                        class="text-2xl font-bold mb-2 flex flex-row items-center gap-2 text-red-600"
                    >
                        <svg
                            class="w-8 h-8 text-red-600"
                            aria-hidden="true"
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            fill="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M2 12C2 6.477 6.477 2 12 2s10 4.477 10 10-4.477 10-10 10S2 17.523 2 12Zm11-4a1 1 0 1 0-2 0v5a1 1 0 1 0 2 0V8Zm-1 7a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H12Z"
                                clip-rule="evenodd"
                            />
                        </svg>
                        WARNING: Not Running
                    </span>
                    <span>
                        Rayhunter is not currently running and will not detect abnormal behavior!
                    </span>
                    <div class="flex flex-row justify-end mt-2">
                        <RecordingControls server_is_recording={!!current_entry} />
                    </div>
                </div>
            {/if}
            <SystemStatsTable stats={system_stats!} />
        </div>
        {#if gps_mode !== 0}
        <div class="bg-white border border-gray-200 drop-shadow rounded-md p-4 flex flex-col gap-2">
            <span class="text-lg font-semibold flex flex-row items-center gap-2">
                <svg
                    class="w-5 h-5 text-rayhunter-blue"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    fill="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        fill-rule="evenodd"
                        d="M11.906 1.994a8.002 8.002 0 0 1 8.09 8.421 7.996 7.996 0 0 1-1.297 3.957.996.996 0 0 1-.133.204l-.108.129c-.178.243-.37.477-.573.699l-5.112 6.224a1 1 0 0 1-1.545 0L5.982 15.26l-.002-.002a18.146 18.146 0 0 1-.309-.38l-.133-.163a.999.999 0 0 1-.13-.202 7.995 7.995 0 0 1 6.498-12.518ZM15 9.997a3 3 0 1 1-5.999 0 3 3 0 0 1 5.999 0Z"
                        clip-rule="evenodd"
                    />
                </svg>
                GPS Status
            </span>
            {#if gps_data}
                <table class="w-full text-sm">
                    <tbody>
                        <tr class="border-b border-gray-100">
                            <td class="py-1 pr-4 text-gray-500 font-medium">Latitude</td>
                            <td class="py-1 font-mono">{gps_data.latitude.toFixed(6)}</td>
                        </tr>
                        <tr class="border-b border-gray-100">
                            <td class="py-1 pr-4 text-gray-500 font-medium">Longitude</td>
                            <td class="py-1 font-mono">{gps_data.longitude.toFixed(6)}</td>
                        </tr>
                        <tr>
                            <td class="py-1 pr-4 text-gray-500 font-medium">GPS Timestamp</td>
                            <td class="py-1 font-mono">{gps_data.timestamp}</td>
                        </tr>
                    </tbody>
                </table>
            {:else}
                <span class="text-gray-400 text-sm">Awaiting GPS data...</span>
            {/if}
        </div>
        {/if}
        <div class="flex flex-col gap-2">
            <div class="flex flex-row gap-2">
                <div class="text-xl flex-1">History</div>
                <div class="flex flex-row items-center gap-2 px-3">
                    <label
                        for="filter_threshold"
                        class="block text-md font-medium text-gray-700 mb-1"
                    >
                        Filter for Warnings
                    </label>
                    <input
                        type="checkbox"
                        id="filter_threshold"
                        bind:checked={filter_threshold}
                        class="px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-rayhunter-blue"
                    />
                </div>
            </div>
            <ManifestTable {entries} server_is_recording={!!current_entry} {manager} />
        </div>
        <DeleteAllButton />
    {:else}
        <div class="flex flex-col justify-center items-center">
            <!-- https://www.w3.org/WAI/tutorials/images/decorative/ -->
            <img src="/rayhunter_orca_only.png" alt="" class="h-48 animate-spin" />
            <p class="text-xl">Loading...</p>
        </div>
    {/if}
</div>
