<script lang="ts">
    import { get_daemon_time } from '$lib/utils.svelte';
    import ApiRequestButton from './ApiRequestButton.svelte';

    let show_alert = $state(false);
    let device_system_time = $state('');
    let device_adjusted_time = $state('');
    let browser_time = $state('');
    let has_offset = $state(false);
    let computed_offset = $state(0);
    let dismissed = $state(false);
    let check_completed = $state(false);

    const DRIFT_THRESHOLD_SECONDS = 30;

    function format_time(date: Date): string {
        return date.toLocaleString();
    }

    async function check_clock_drift() {
        if (check_completed) return;

        try {
            const daemon_time_response = await get_daemon_time();
            const browser_now = new Date();
            const daemon_system_ms = new Date(daemon_time_response.system_time).getTime();
            const device_adjusted_ms = new Date(daemon_time_response.adjusted_time).getTime();
            const drift_seconds = Math.round((browser_now.getTime() - device_adjusted_ms) / 1000);

            if (Math.abs(drift_seconds) > DRIFT_THRESHOLD_SECONDS && !dismissed) {
                device_system_time = format_time(new Date(daemon_time_response.system_time));
                device_adjusted_time = format_time(new Date(daemon_time_response.adjusted_time));
                browser_time = format_time(browser_now);
                has_offset = daemon_time_response.offset_seconds !== 0;
                // Calculate offset needed: browser_time - daemon_system_time
                computed_offset = Math.round((browser_now.getTime() - daemon_system_ms) / 1000);
                show_alert = true;
            }
        } catch (err) {
            console.error('Failed to check clock drift:', err);
        }
        check_completed = true;
    }

    function dismiss() {
        show_alert = false;
        dismissed = true;
    }

    // Check clock drift on component mount
    $effect(() => {
        check_clock_drift();
    });
</script>

{#if show_alert}
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
                    d="M2 12C2 6.477 6.477 2 12 2s10 4.477 10 10-4.477 10-10 10S2 17.523 2 12Zm11-4a1 1 0 1 0-2 0v4a1 1 0 0 0 .293.707l3 3a1 1 0 0 0 1.414-1.414L13 11.586V8Z"
                    clip-rule="evenodd"
                />
            </svg>
            Clock Mismatch Detected
        </span>
        <p>
            Rayhunter's clock doesn't match your browser's, and may be incorrect. This can happen if
            Rayhunter is unable to get the correct time from the internet. Consider synchronizing
            your browser's clock with the button below, or using another SIM card for better
            results.
        </p>
        <table class="w-fit">
            <tbody>
                <tr>
                    <td class="pr-2">Rayhunter clock (system):</td>
                    <td class="font-mono">{device_system_time}</td>
                </tr>
                {#if has_offset}
                    <tr>
                        <td class="pr-2">Rayhunter clock (adjusted):</td>
                        <td class="font-mono">{device_adjusted_time}</td>
                    </tr>
                {/if}
                <tr>
                    <td class="pr-2">Browser clock:</td>
                    <td class="font-mono">{browser_time}</td>
                </tr>
            </tbody>
        </table>
        <p>Copy browser clock to device?</p>
        <div class="flex flex-row gap-2 justify-end">
            <button
                class="font-medium py-2 px-4 rounded-md border border-gray-400 hover:bg-yellow-200"
                onclick={dismiss}
            >
                Dismiss
            </button>
            <ApiRequestButton
                url="/api/time-offset"
                label="Sync Clock"
                loadingLabel="Syncing..."
                variant="green"
                jsonBody={{ offset_seconds: computed_offset }}
                onclick={dismiss}
                errorMessage="Error syncing clock"
            />
        </div>
    </div>
{/if}
