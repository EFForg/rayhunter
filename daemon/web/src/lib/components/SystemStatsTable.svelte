<script lang="ts">
    import { type SystemStats } from "$lib/systemStats";
    let { stats }: {
        stats: SystemStats;
    } = $props();

    const table_cell_classes = "border p-1 lg:p-2";
</script>

<div class="flex-1 drop-shadow p-4 flex flex-col gap-2 border rounded-md bg-gray-100 border-gray-100">
    <p class="text-xl mb-2">System Information</p>
    <table class="table-auto border">
        <tbody>
            <tr class="border">
                <th class={table_cell_classes}>
                    Rayhunter Version
                </th>
                <td class={table_cell_classes}>{stats.runtime_metadata.rayhunter_version}</td>
            </tr>
            <tr class="border">
                <th class={table_cell_classes}>
                    Storage
                </th>
                <td class={table_cell_classes}>
                    {stats.disk_stats.used_percent} used ({stats.disk_stats.used_size} used / {stats.disk_stats.available_size} available)
                </td>
            </tr>
            <tr class="border-b">
                <th class={table_cell_classes}>
                    Memory (RAM)
                </th>
                <td class={table_cell_classes}>
                    Free: {stats.memory_stats.free}, Used: {stats.memory_stats.used}
                </td>
            </tr>
        </tbody>
    </table>
</div>
