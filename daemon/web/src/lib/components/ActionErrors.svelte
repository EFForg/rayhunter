<script lang="ts">
    import { action_errors } from '../action_errors.svelte';
    import Alert from './Alert.svelte';
    import TrashIcon from './TrashIcon.svelte';

    let pos = $state(0);
    let current_error = $derived(action_errors[pos]);
    let error_times = $derived(current_error?.times ?? 1);
    let error_title = $derived(
        `Error Completing Action ${error_times > 1 ? `x${error_times}` : ''}`
    );

    function prev_error() {
        if (pos > 0) pos -= 1;
        else pos = action_errors.length - 1;
    }
    function next_error() {
        if (pos + 1 < action_errors.length) pos += 1;
        else pos = 0;
    }
    function clear_errors() {
        pos = 0;
        action_errors.length = 0;
    }
</script>

{#if action_errors.length > 0}
    <Alert
        severity="error"
        title={error_title}
        class="flex-1 justify-between fixed z-10 right-3 bottom-3 ml-3"
        titleClass="text-xl mb-2 mr-5 gap-1"
    >
        {#snippet actions()}
            <div class="flex items-center mb-2">
                {#if action_errors.length > 1}
                    <span>{pos + 1}/{action_errors.length}</span>
                    <button title="previous error" aria-label="previous error" onclick={prev_error}>
                        <svg
                            aria-hidden="true"
                            width="24"
                            height="24"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke="currentColor"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="m 15.499979,19.499979 -6.9999997,-7 6.9999997,-6.9999997"
                            />
                        </svg>
                    </button>
                    <button title="next error" aria-label="next error" onclick={next_error}>
                        <svg
                            aria-hidden="true"
                            width="24"
                            height="24"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke="currentColor"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="m 8.5000207,5.4999793 7.0000003,6.9999997 -7.0000003,7"
                            />
                        </svg>
                    </button>
                {/if}
                <button title="clear errors" aria-label="clear errors" onclick={clear_errors}>
                    <TrashIcon class="w-6 h-6" />
                </button>
            </div>
        {/snippet}
        <span>{current_error.message}</span>
        {#if current_error.cause}
            <details>
                <summary>Details</summary>
                <code>{current_error.cause}</code>
            </details>
        {/if}
    </Alert>
{/if}
