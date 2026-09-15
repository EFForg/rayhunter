<script lang="ts">
    import { get_daemon_time, get_config, set_time_offset, ClockSyncMode } from '$lib/utils.svelte';
    import ApiRequestButton from './ApiRequestButton.svelte';
    import Alert from './Alert.svelte';

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
            const config = await get_config();
            if (config.clock_sync_mode === ClockSyncMode.Off) {
                check_completed = true;
                return;
            }

            const daemon_time_response = await get_daemon_time();
            const browser_now = new Date();
            const daemon_system_ms = new Date(daemon_time_response.system_time).getTime();
            const device_adjusted_ms = new Date(daemon_time_response.adjusted_time).getTime();
            const drift_seconds = Math.round((browser_now.getTime() - device_adjusted_ms) / 1000);

            if (Math.abs(drift_seconds) <= DRIFT_THRESHOLD_SECONDS) {
                check_completed = true;
                return;
            }

            if (config.clock_sync_mode === ClockSyncMode.Autosync) {
                // Offset needed: browser_time - daemon_system_time
                await set_time_offset(
                    Math.round((browser_now.getTime() - daemon_system_ms) / 1000)
                );
                check_completed = true;
                return;
            }

            if (!dismissed) {
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
    <Alert severity="warning" title="Clock Mismatch Detected">
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
        <div class="flex flex-row flex-wrap gap-2 items-center justify-end">
            <p class="text-sm text-yellow-700 mr-auto">
                Rayhunter can sync this automatically with the "Clock Sync" setting in Config.
            </p>
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
    </Alert>
{/if}
