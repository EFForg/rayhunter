<script lang="ts">
    import ApiRequestButton from './ApiRequestButton.svelte';
    let {
        server_is_recording,
    }: {
        server_is_recording: boolean;
    } = $props();
</script>

<div>
    {#if server_is_recording}
        <ApiRequestButton
            url="/api/stop-recording"
            label="Stop"
            variant="red"
            errorMessage="Error stoppping recording"
        >
            {#snippet icon()}
                <svg
                    class="w-6 h-6 text-white"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    fill="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        d="M7 5a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2H7Z"
                    />
                </svg>
            {/snippet}
        </ApiRequestButton>
    {:else}
        <ApiRequestButton
            url="/api/start-recording"
            label="Start"
            variant="blue"
            errorMessage="Error starting recording"
        >
            {#snippet icon()}
                <svg
                    class="w-6 h-6 text-white"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    fill="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        fill-rule="evenodd"
                        d="M8.6 5.2A1 1 0 0 0 7 6v12a1 1 0 0 0 1.6.8l8-6a1 1 0 0 0 0-1.6l-8-6Z"
                        clip-rule="evenodd"
                    />
                </svg>
            {/snippet}
        </ApiRequestButton>
    {/if}
</div>
