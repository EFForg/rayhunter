<script lang="ts">
    import { type SystemStats } from '$lib/systemStats';
    let {
        stats,
    }: {
        stats: SystemStats;
    } = $props();

    const table_cell_classes = 'border p-1 lg:p-2';

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

    // Cell signal quality assessment based on RSRP
    // Excellent: >= -80 dBm, Good: -80 to -90, Fair: -90 to -100, Poor: < -100
    let signal_quality = $derived.by(() => {
        const rsrp = stats.cell_info?.rsrp_dbm;
        if (rsrp === undefined) return { label: 'Unknown', color: 'text-gray-500' };
        if (rsrp >= -80) return { label: 'Excellent', color: 'text-green-600' };
        if (rsrp >= -90) return { label: 'Good', color: 'text-green-600' };
        if (rsrp >= -100) return { label: 'Fair', color: 'text-yellow-600' };
        return { label: 'Poor', color: 'text-red-600' };
    });

    // Format signal value with unit
    function format_signal(value: number | undefined, unit: string): string {
        if (value === undefined) return '—';
        return `${value.toFixed(1)} ${unit}`;
    }
</script>

<div
    class="flex-1 drop-shadow p-4 flex flex-col gap-2 border rounded-md bg-gray-100 border-gray-100"
>
    <p class="text-xl mb-2">System Information</p>
    <table class="table-auto border">
        <tbody>
            <tr class="border">
                <th class={table_cell_classes}> Rayhunter Version </th>
                <td class={table_cell_classes}>{stats.runtime_metadata.rayhunter_version}</td>
            </tr>
            <tr class="border">
                <th class={table_cell_classes}> Storage </th>
                <td class={table_cell_classes}>
                    {stats.disk_stats.used_percent} used ({stats.disk_stats.used_size} used / {stats
                        .disk_stats.available_size} available)
                </td>
            </tr>
            <tr class="border-b">
                <th class={table_cell_classes}> Memory (RAM) </th>
                <td class={table_cell_classes}>
                    Free: {stats.memory_stats.free}, Used: {stats.memory_stats.used}
                </td>
            </tr>
            <tr class="border-b">
                <th class={table_cell_classes}> Battery </th>
                <td class={table_cell_classes}>
                    <svg
                        width="80"
                        height="30"
                        viewBox="0 0 80 30"
                        role="img"
                        xmlns="http://www.w3.org/2000/svg"
                        class="battery-icon"
                    >
                        <title>{title_text}</title>
                        <!-- Battery body -->
                        <rect
                            class="fill-none stroke-neutral-800 stroke-2"
                            width="70"
                            height="30"
                            rx="3"
                            ry="3"
                        />
                        <!-- Battery terminal -->
                        <rect
                            class="fill-neutral-800"
                            x="70"
                            y="7"
                            width="8"
                            height="16"
                            rx="2"
                            ry="2"
                        />
                        <!-- Battery charge bar -->
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
                            <!-- Lightning bolt icon -->
                            <path
                                class="fill-yellow-300 stroke-neutral-800 stroke-1"
                                d="M38 3 L28 17 L34 17 L30 27 L40 13 L34 13 Z"
                            />
                        {/if}
                        {#if !stats.battery_status}
                            <!-- Question mark icon -->
                            <text
                                class="fill-neutral-500 text-[20px] font-bold [text-anchor:middle] [dominant-baseline:central]"
                                x="35"
                                y="15">?</text
                            >
                        {/if}
                    </svg>
                </td>
            </tr>
            {#if stats.cell_info}
                <tr class="border-b">
                    <th class={table_cell_classes}> Cell Signal </th>
                    <td class={table_cell_classes}>
                        <div class="flex flex-col gap-1 text-sm">
                            <div class="flex items-center gap-2">
                                <span class="font-medium {signal_quality.color}">
                                    {signal_quality.label}
                                </span>
                                {#if stats.cell_info.rsrp_dbm !== undefined}
                                    <span class="text-gray-600">
                                        (RSRP: {format_signal(stats.cell_info.rsrp_dbm, 'dBm')})
                                    </span>
                                {/if}
                            </div>
                            <div class="text-gray-600 text-xs">
                                {#if stats.cell_info.pci !== undefined}
                                    <span>PCI: {stats.cell_info.pci}</span>
                                {/if}
                                {#if stats.cell_info.earfcn !== undefined}
                                    <span class="ml-2">EARFCN: {stats.cell_info.earfcn}</span>
                                {/if}
                                {#if stats.cell_info.rsrq_db !== undefined}
                                    <span class="ml-2"
                                        >RSRQ: {format_signal(stats.cell_info.rsrq_db, 'dB')}</span
                                    >
                                {/if}
                                {#if stats.cell_info.rssi_dbm !== undefined}
                                    <span class="ml-2"
                                        >RSSI: {format_signal(
                                            stats.cell_info.rssi_dbm,
                                            'dBm'
                                        )}</span
                                    >
                                {/if}
                            </div>
                        </div>
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
