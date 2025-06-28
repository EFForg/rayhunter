<script lang="ts">
    import { req } from "$lib/utils.svelte";
    let { server_is_recording }: {
        server_is_recording: boolean;
    } = $props();

    let client_set_recording = $state(server_is_recording);
    let waiting_for_server = $derived(client_set_recording !== server_is_recording);

    async function start_recording() {
        await req('POST', '/api/start-recording');
        client_set_recording = true;
    }

    async function stop_recording() {
        await req('POST', '/api/stop-recording');
        client_set_recording = false;
    }

    const recording_button_classes = "text-white font-bold py-2 px-4 rounded-md flex flex-row gap-1";
    const stop_recording_classes = `${recording_button_classes} bg-red-500 opacity-50 cursor-not-allowed`;
    const start_recording_classes = `${recording_button_classes} bg-blue-500 opacity-50 cursor-not-allowed`;
</script>

<div>
    {#if waiting_for_server}
        <button class={server_is_recording ? stop_recording_classes : start_recording_classes} disabled>
            <span>{server_is_recording ? "Stopping..." : "Starting..."}</span>
            <svg class="w-4 h-4 text-white animate-spin" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="m4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
        </button>
    {:else if server_is_recording}
        <button class="{recording_button_classes} bg-red-500 hover:bg-red-700" onclick={stop_recording}>
            <span>Stop</span>
            <svg class="w-6 h-6 text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                <path d="M7 5a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2H7Z"/>
            </svg>
        </button>
    {:else}
        <button class="{recording_button_classes} bg-blue-500 hover:bg-blue-700" onclick={start_recording}>
            <span>Start</span>
            <svg class="w-6 h-6 text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                <path fill-rule="evenodd" d="M8.6 5.2A1 1 0 0 0 7 6v12a1 1 0 0 0 1.6.8l8-6a1 1 0 0 0 0-1.6l-8-6Z" clip-rule="evenodd"/>
            </svg>
        </button>
    {/if}
</div>

<style>
</style>
