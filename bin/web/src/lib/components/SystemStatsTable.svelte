<script lang="ts">
    import { type SystemStats } from "$lib/systemStats";
    let { stats }: {
        stats: SystemStats;
    } = $props();

</script>

<div>
    <p class="text-xl mb-2">System Information</p>
    <table class="table-auto border">
        <tbody>
            <tr class="border">
                <th class="border">
                    Rayhunter Version
                </th>
                <td class="border">{stats.runtime_metadata.rayhunter_version}</td>
            </tr>
            <tr class="border">
                <th class="border">
                    Storage
                </th>
                <td class="border">
                    {stats.disk_stats.used_percent} used ({stats.disk_stats.used_size} / {stats.disk_stats.available_size})
                </td>
            </tr>
            <tr class="border-b">
                <th class="border">
                    Memory (RAM)
                </th>
                <td class="border">
                    Free: {stats.memory_stats.free}, Used: {stats.memory_stats.used}
                </td>
            </tr>
        </tbody>
    </table>
</div>
