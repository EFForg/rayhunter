<script lang="ts">
    import { req } from "$lib/utils.svelte";
    let { currently_recording }: {
        currently_recording: boolean;
    } = $props();

    async function start_recording() {
        await req('POST', '/api/start-recording');
    }

    async function stop_recording() {
        await req('POST', '/api/stop-recording');
    }
</script>

<div>
    {#if currently_recording}
        <button onclick={stop_recording}>Stop Recording</button>
    {:else}
        <button onclick={start_recording}>Start Recording</button>
    {/if}
</div>

<style>
</style>
