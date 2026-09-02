<script lang="ts">
    import type { ManifestEntry } from '$lib/manifest.svelte';
    import { get_daemon_time } from '$lib/utils.svelte';

    let { entry }: { entry: ManifestEntry | undefined } = $props();
    let daemon_adjusted_time = $state<Date | undefined>(undefined);

    const NO_MESSAGES_THRESHOLD_MS = 5 * 60 * 1000;

    let show_alert = $derived.by(() => {
        if (!entry || !daemon_adjusted_time) return false;
        const latest_activity = entry.last_message_time ?? entry.start_time;
        return (
            daemon_adjusted_time.getTime() - latest_activity.getTime() >= NO_MESSAGES_THRESHOLD_MS
        );
    });

    async function update_daemon_time() {
        try {
            const response = await get_daemon_time();
            daemon_adjusted_time = new Date(response.adjusted_time);
        } catch (err) {
            console.error('Failed to check diagnostic message activity:', err);
        }
    }

    $effect(() => {
        update_daemon_time();
        const interval = setInterval(update_daemon_time, 30_000);
        return () => clearInterval(interval);
    });
</script>

{#if show_alert}
    <div
        class="bg-yellow-100 border-yellow-400 drop-shadow-sm p-4 flex flex-col gap-2 border rounded-md"
    >
        <span class="text-xl font-bold text-yellow-700">No diagnostic messages received</span>
        <p>
            Rayhunter has not received diagnostic messages for at least five minutes and may not be
            recording modem activity.
        </p>
    </div>
{/if}
