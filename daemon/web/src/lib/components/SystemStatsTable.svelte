<script lang="ts">
    import { type SystemStats } from '$lib/systemStats';
    import { gpsModeLabel, GpsMode, type GpsData } from '$lib/utils.svelte';
    let {
        stats,
        gps_data = null,
        gps_mode = GpsMode.Disabled,
    }: {
        stats: SystemStats;
        gps_data?: GpsData | null;
        gps_mode?: GpsMode;
    } = $props();

    let battery_level = $derived(stats.battery_status ? stats.battery_status.level : 0);
    let bar_color = $derived.by(() => {
        if (stats.battery_status === undefined) {
            return '';
        }
        if (battery_level <= 10) {
            return 'fill-red-500';
        }
        if (battery_level <= 25) {
            return 'fill-yellow-300';
        }
        return 'fill-green-500';
    });
    let title_text = $derived.by(() => {
        if (stats.battery_status === undefined) {
            return 'Rayhunter does not yet support displaying the battery level for this device.';
        }

        let text = `Battery is ${stats.battery_status.level}% full`;

        if (stats.battery_status.is_plugged_in) {
            text += ' and plugged in';
        }
        return text;
    });

    const gps_date_formatter = new Intl.DateTimeFormat(undefined, {
        timeStyle: 'long',
        dateStyle: 'short',
    });
</script>

<div
    class="flex-1 drop-shadow-sm p-4 flex flex-col gap-2 border rounded-md bg-gray-100 border-gray-100"
>
    <p class="text-xl mb-2">System Information</p>
    <table class="text-sm w-full">
        <tbody>
            <tr class="border-b border-gray-200">
                <td class="py-1 pr-4 text-gray-500 font-medium">Rayhunter Version</td>
                <td class="py-1">{stats.runtime_metadata.rayhunter_version}</td>
            </tr>
            <tr class="border-b border-gray-200">
                <td class="py-1 pr-4 text-gray-500 font-medium">Storage</td>
                <td class="py-1">
                    {stats.disk_stats.used_percent} used ({stats.disk_stats.used_size} used / {stats
                        .disk_stats.available_size} available)
                </td>
            </tr>
            <tr class="border-b border-gray-200">
                <td class="py-1 pr-4 text-gray-500 font-medium">Memory (RAM)</td>
                <td class="py-1">
                    Free: {stats.memory_stats.free}, Used: {stats.memory_stats.used}
                </td>
            </tr>
            <tr class={gps_mode !== GpsMode.Disabled ? 'border-b border-gray-200' : ''}>
                <td class="py-1 pr-4 text-gray-500 font-medium">Battery</td>
                <td class="py-1">
                    <svg
                        width="80"
                        height="30"
                        viewBox="0 0 80 30"
                        role="img"
                        xmlns="http://www.w3.org/2000/svg"
                        class="battery-icon"
                    >
                        <title>{title_text}</title>
                        <rect
                            class="fill-none stroke-neutral-800 stroke-2"
                            width="70"
                            height="30"
                            rx="3"
                            ry="3"
                        />
                        <rect
                            class="fill-neutral-800"
                            x="70"
                            y="7"
                            width="8"
                            height="16"
                            rx="2"
                            ry="2"
                        />
                        <rect
                            class={bar_color}
                            x="2"
                            y="2"
                            height="26"
                            rx="2"
                            ry="2"
                            style="width: {battery_level * 0.66}px;"
                        />
                        {#if stats.battery_status && stats.battery_status.is_plugged_in}
                            <path
                                class="fill-yellow-300 stroke-neutral-800 stroke-1"
                                d="M38 3 L28 17 L34 17 L30 27 L40 13 L34 13 Z"
                            />
                        {/if}
                        {#if !stats.battery_status}
                            <text
                                class="fill-neutral-500 text-[20px] font-bold [text-anchor:middle] [dominant-baseline:central]"
                                x="35"
                                y="15">?</text
                            >
                        {/if}
                    </svg>
                </td>
            </tr>
            {#if gps_mode !== GpsMode.Disabled}
                <tr class="border-b border-gray-200">
                    <td class="py-1 pr-4 text-gray-500 font-medium">GPS Mode</td>
                    <td class="py-1">{gpsModeLabel(gps_mode)}</td>
                </tr>
                {#if gps_data}
                    <tr class="border-b border-gray-200">
                        <td class="py-1 pr-4 text-gray-500 font-medium">Latitude</td>
                        <td class="py-1 font-mono">{gps_data.latitude.toFixed(6)}</td>
                    </tr>
                    <tr class="border-b border-gray-200">
                        <td class="py-1 pr-4 text-gray-500 font-medium">Longitude</td>
                        <td class="py-1 font-mono">{gps_data.longitude.toFixed(6)}</td>
                    </tr>
                    <tr>
                        <td class="py-1 pr-4 text-gray-500 font-medium">GPS Timestamp</td>
                        <td class="py-1 font-mono">
                            {gps_data.timestamp > 0
                                ? gps_date_formatter.format(new Date(gps_data.timestamp * 1000))
                                : 'Fixed'}
                        </td>
                    </tr>
                {:else}
                    <tr>
                        <td class="py-1 pr-4 text-gray-500 font-medium">GPS Data</td>
                        <td class="py-1 text-gray-400">Awaiting GPS data...</td>
                    </tr>
                {/if}
            {/if}
        </tbody>
    </table>
</div>
