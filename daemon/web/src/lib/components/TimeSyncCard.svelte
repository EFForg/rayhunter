<script lang="ts">
    import { get_time_correction, sync_time_from_browser, type TimeCorrection } from '$lib/utils.svelte';

    let time_correction: TimeCorrection | undefined = $state(undefined);
    let syncing = $state(false);
    let sync_message = $state<string | undefined>(undefined);
    let error_message = $state<string | undefined>(undefined);

    async function load_time_correction() {
        try {
            time_correction = await get_time_correction();
            error_message = undefined;
        } catch (error) {
            console.error('Failed to load time correction:', error);
            error_message = error instanceof Error ? error.message : 'Failed to load time correction';
        }
    }

    async function sync_time() {
        syncing = true;
        sync_message = undefined;
        error_message = undefined;
        try {
            const response = await sync_time_from_browser();
            sync_message = response.message;
            time_correction = await get_time_correction();
        } catch (error) {
            console.error('Failed to sync time:', error);
            error_message = error instanceof Error ? error.message : 'Failed to sync time';
        } finally {
            syncing = false;
        }
    }

    function format_offset(offset_seconds: number): string {
        const abs_offset = Math.abs(offset_seconds);
        const hours = Math.floor(abs_offset / 3600);
        const minutes = Math.floor((abs_offset % 3600) / 60);
        const seconds = abs_offset % 60;

        let result = '';
        if (hours > 0) result += `${hours}h `;
        if (minutes > 0) result += `${minutes}m `;
        result += `${seconds}s`;

        return offset_seconds >= 0 ? `+${result}` : `-${result}`;
    }

    function format_datetime(iso_string?: string): string {
        if (!iso_string) return 'Never';
        try {
            const date = new Date(iso_string);
            return date.toLocaleString();
        } catch {
            return 'Invalid date';
        }
    }

    // Load time correction on mount and periodically
    $effect(() => {
        load_time_correction();
        const interval = setInterval(load_time_correction, 5000);
        return () => clearInterval(interval);
    });
</script>

<div
    class="drop-shadow p-4 flex flex-col gap-2 border rounded-md bg-gray-100 border-gray-100"
>
    <p class="text-xl mb-2">Time Synchronization</p>

    {#if error_message}
        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
            <strong class="font-bold">Error: </strong>
            <span class="block sm:inline">{error_message}</span>
        </div>
    {/if}

    {#if sync_message}
        <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded relative" role="alert">
            <span class="block sm:inline">{sync_message}</span>
        </div>
    {/if}

    {#if time_correction}
        <div class="flex flex-col gap-2">
            <div class="flex flex-row justify-between items-center">
                <span class="text-sm text-gray-600">Current Offset:</span>
                <span class="font-mono text-lg font-semibold">
                    {format_offset(time_correction.offset_seconds)}
                </span>
            </div>
            <div class="flex flex-row justify-between items-center">
                <span class="text-sm text-gray-600">Last Synced:</span>
                <span class="text-sm">
                    {format_datetime(time_correction.last_updated)}
                </span>
            </div>
        </div>
    {/if}

    <div class="flex flex-col gap-2 mt-2">
        <button
            onclick={sync_time}
            disabled={syncing}
            class="px-4 py-2 bg-rayhunter-blue text-white rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
        >
            {syncing ? 'Syncing...' : 'Sync Time from Browser'}
        </button>
        <p class="text-xs text-gray-500">
            Click to synchronize the device time offset with your browser's time. This will not modify the system clock.
        </p>
    </div>
</div>
