<script lang="ts">
    import { action_errors } from '../action_errors.svelte';
    import WarningIcon from './WarningIcon.svelte';

    let pos = $state(0);
    let current_error = $derived(action_errors[pos]);

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
    <div
        class="bg-red-100 border-red-100 drop-shadow p-4 flex flex-col gap-2
        border rounded-md flex-1 justify-between fixed z-10 right-3 bottom-3 ml-3"
    >
        <div class="flex flex-row justify-between">
            <span class="text-xl font-bold mb-2 mr-5 flex flex-row items-center gap-1 text-red-600">
                <WarningIcon class="w-6 h-6 text-red-600" />
                Error Completing Action {current_error.times > 1 ? `x${current_error.times}` : ''}
            </span>
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
                    <svg style="width:24px;height:24px" viewBox="0 0 24 24">
                        <path
                            d="M19,4H15.5L14.5,3H9.5L8.5,4H5V6H19M6,19A2,2 0 0,0 8,21H16A2,2 0 0,0 18,19V7H6V19Z"
                        />
                    </svg>
                </button>
            </div>
        </div>
        <span>{current_error.message}</span>
        {#if current_error.cause}
            <details>
                <summary>Details</summary>
                <code>{current_error.cause}</code>
            </details>
        {/if}
    </div>
{/if}
