<script lang="ts">
    import { get_tls_status } from '$lib/utils.svelte';
    import WarningIcon from './WarningIcon.svelte';

    let show_alert = $state(false);
    let fallback_reason = $state('');
    let tls_path = $state('');
    let check_completed = $state(false);

    async function check_tls_status() {
        if (check_completed) return;

        try {
            const status = await get_tls_status();
            if (status.https_enabled && status.fallback_mode) {
                show_alert = true;
                fallback_reason = status.fallback_reason || 'TLS certificate issues detected.';
                tls_path = status.tls_path;
            }
        } catch (err) {
            console.error('Failed to check TLS status:', err);
        }
        check_completed = true;
    }

    function dismiss() {
        show_alert = false;
    }

    $effect(() => {
        check_tls_status();
    });
</script>

{#if show_alert}
    <div
        class="bg-yellow-100 border-yellow-400 drop-shadow p-4 flex flex-col gap-2 border rounded-md"
    >
        <span class="text-xl font-bold flex flex-row items-center gap-2 text-yellow-700">
            <WarningIcon class="w-6 h-6 text-yellow-600" />
            HTTPS Fallback Mode
        </span>
        <p>
            {fallback_reason}
        </p>
        <p class="text-sm text-gray-600">
            Rayhunter is running in HTTP-only mode. To fix this, connect via ADB and delete the
            <code class="bg-gray-200 px-1 rounded">{tls_path}</code> directory, then reboot the device.
        </p>
        <div class="flex flex-row gap-2 justify-end">
            <button
                class="font-medium py-2 px-4 rounded-md border border-gray-400 hover:bg-yellow-200"
                onclick={dismiss}
            >
                Dismiss
            </button>
        </div>
    </div>
{/if}
