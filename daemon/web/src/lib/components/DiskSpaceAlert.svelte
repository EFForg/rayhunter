<script lang="ts">
    import type { SystemStats } from '$lib/systemStats';
    import type { Config } from '$lib/utils.svelte';
    import { get_config } from '$lib/utils.svelte';

    interface Props {
        stats: SystemStats;
        is_recording: boolean;
    }

    let { stats, is_recording }: Props = $props();

    let config: Config | undefined = $state(undefined);

    $effect(() => {
        if (!config) {
            get_config().then((c) => (config = c));
        }
    });

    let warning_message = $derived.by(() => {
        if (!config || !stats.disk_stats.available_bytes) return undefined;

        const available_mb = Math.floor(stats.disk_stats.available_bytes / 1024 / 1024);
        const start_threshold = config.min_space_to_start_recording_mb;
        const continue_threshold = config.min_space_to_continue_recording_mb;

        if (!is_recording && available_mb < start_threshold) {
            return `Disk space too low to record: ${available_mb}MB free, ${start_threshold}MB required. Delete old recordings to free space.`;
        }

        if (is_recording && available_mb < start_threshold) {
            return `Disk space running low: ${available_mb}MB free. Recording will stop automatically below ${continue_threshold}MB.`;
        }

        return undefined;
    });
</script>

{#if warning_message}
    <div
        class="bg-yellow-100 border-yellow-400 drop-shadow p-4 flex flex-col gap-2 border rounded-md"
    >
        <span class="text-xl font-bold flex flex-row items-center gap-2 text-yellow-700">
            <svg
                class="w-6 h-6 text-yellow-600"
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
            Low Disk Space
        </span>
        <p>{warning_message}</p>
    </div>
{/if}
