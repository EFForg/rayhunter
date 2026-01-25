<script lang="ts">
    import { user_action_req } from '$lib/utils.svelte';

    let {
        url,
        method = 'POST',
        label,
        loadingLabel,
        disabled = false,
        variant = 'blue',
        icon,
        onclick,
        ariaLabel,
        errorMessage,
        jsonBody,
    }: {
        url: string;
        method?: string;
        label: string;
        loadingLabel?: string;
        disabled?: boolean;
        variant?: 'blue' | 'red' | 'green';
        icon?: any; // Svelte snippet
        onclick?: () => void | Promise<void>;
        ariaLabel?: string;
        errorMessage?: string;
        jsonBody?: unknown;
    } = $props();

    let is_requesting = $state(false);
    let is_disabled = $derived(disabled || is_requesting);

    const variantClasses = {
        blue: {
            enabled: 'bg-blue-500 hover:bg-blue-700',
            disabled: 'bg-blue-500 opacity-50 cursor-not-allowed',
        },
        red: {
            enabled: 'bg-red-500 hover:bg-red-700',
            disabled: 'bg-red-500 opacity-50 cursor-not-allowed',
        },
        green: {
            enabled: 'bg-green-500 hover:bg-green-700',
            disabled: 'bg-green-500 opacity-50 cursor-not-allowed',
        },
    };

    async function handle_click() {
        if (is_disabled) return;

        is_requesting = true;
        try {
            await user_action_req(
                method,
                url,
                errorMessage ? errorMessage : 'Error performing action',
                jsonBody
            );
            if (onclick) {
                await onclick();
            }
        } catch (err) {
            console.error(`Failed to ${method} ${url}:`, err);
            alert(`Request failed. Please try again.`);
        } finally {
            is_requesting = false;
        }
    }

    let buttonClasses = $derived(
        is_disabled ? variantClasses[variant].disabled : variantClasses[variant].enabled
    );
</script>

<button
    class="text-white font-bold py-2 px-2 sm:px-4 rounded-md flex flex-row items-center gap-1 {buttonClasses}"
    onclick={handle_click}
    disabled={is_disabled}
    aria-label={ariaLabel || label}
>
    <span>{is_requesting && loadingLabel ? loadingLabel : label}</span>
    {#if is_requesting}
        <svg
            class="w-4 h-4 text-white animate-spin"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
        >
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
            ></circle>
            <path
                class="opacity-75"
                fill="currentColor"
                d="m4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            ></path>
        </svg>
    {:else if icon}
        {@render icon()}
    {/if}
</button>
