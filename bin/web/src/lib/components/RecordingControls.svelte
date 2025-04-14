<script lang="ts">
    import { req } from "$lib/utils.svelte";
    let { server_is_recording: currently_recording }: {
        server_is_recording: boolean;
    } = $props();

    let client_set_recording = $state(currently_recording);
    let waiting_for_server = $derived(client_set_recording !== currently_recording);

    async function start_recording() {
        await req('POST', '/api/start-recording');
        client_set_recording = true;
    }

    async function stop_recording() {
        await req('POST', '/api/stop-recording');
        client_set_recording = false;
    }

    const stop_recording_classes = "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded-full";
    const start_recording_classes = "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full";
</script>

<div>
    {#if waiting_for_server}
        <button class={currently_recording ? stop_recording_classes : start_recording_classes}>
            {currently_recording ? "Stopping..." : "Starting..."}
        </button>
    {:else if currently_recording}
        <button class={stop_recording_classes} onclick={stop_recording}>Stop Recording</button>
    {:else}
        <button class={start_recording_classes} onclick={start_recording}>Start Recording</button>
    {/if}
</div>

<style>
</style>
