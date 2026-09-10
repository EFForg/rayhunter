<script lang="ts">
    import { user_action_req } from '$lib/utils.svelte';
    import Spinner from './Spinner.svelte';

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
        <Spinner class="w-4 h-4 text-white" />
    {:else if icon}
        {@render icon()}
    {/if}
</button>
