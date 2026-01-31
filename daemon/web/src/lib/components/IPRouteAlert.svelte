<script lang="ts">
    import { get_route_status } from '$lib/utils.svelte';
    import WarningIcon from './WarningIcon.svelte';

    let show_alert = $state(false);
    let check_completed = $state(false);

    async function check_route() {
        if (check_completed) return;

        try {
            const status = await get_route_status();
            if (!status.has_default_route) {
                show_alert = true;
            }
        } catch (err) {
            console.error('Failed to check route status:', err);
        }
        check_completed = true;
    }

    function dismiss() {
        show_alert = false;
    }

    $effect(() => {
        check_route();
    });
</script>

{#if show_alert}
    <div
        class="bg-yellow-100 border-yellow-400 drop-shadow p-4 flex flex-col gap-2 border rounded-md"
    >
        <span class="text-xl font-bold flex flex-row items-center gap-2 text-yellow-700">
            <WarningIcon class="w-6 h-6 text-yellow-600" />
            No Default Route Detected
        </span>
        <p>
            This device didn't get an IP address from the network operator. Presumably the SIM card
            is not inserted or very old. Try a different SIM card.
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
