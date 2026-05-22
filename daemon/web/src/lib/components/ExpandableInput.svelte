<script lang="ts">
    import type { Snippet } from 'svelte';

    let {
        value = $bindable(''),
        checkboxId,
        inputId,
        label,
        inputLabel,
        inputPlaceholder = '',
        inputHelp = '',
        children,
    }: {
        value: string | null;
        checkboxId: string;
        inputId: string;
        label: string;
        inputLabel: string;
        inputPlaceholder?: string;
        inputHelp?: string;
        children?: Snippet;
    } = $props();

    function has_value(text: string | null) {
        return text !== null && text.trim() !== '';
    }

    let expanded = $state(has_value(value));
    let inputElement = $state<HTMLInputElement | null>(null);

    function handle_checkbox_change(e: Event) {
        expanded = (e.currentTarget as HTMLInputElement).checked;
        if (expanded) {
            setTimeout(() => inputElement?.focus(), 0);
        } else {
            value = '';
        }
    }

    function handle_input_blur() {
        if (!has_value(value)) {
            expanded = false;
        }
    }
</script>

<div class="flex items-center">
    <input
        id={checkboxId}
        type="checkbox"
        checked={expanded}
        onchange={handle_checkbox_change}
        class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded-sm"
    />
    <label for={checkboxId} class="ml-2 block text-sm text-gray-700">
        {label}
    </label>
</div>

{#if expanded}
    <div>
        <label for={inputId} class="block text-sm font-medium text-gray-700 mb-1">
            {inputLabel}
        </label>
        <input
            id={inputId}
            type="text"
            bind:this={inputElement}
            bind:value
            onblur={handle_input_blur}
            placeholder={inputPlaceholder}
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-hidden focus:ring-2 focus:ring-rayhunter-blue"
        />
        {#if inputHelp}
            <p class="text-xs text-gray-500 mt-1">
                {inputHelp}
            </p>
        {/if}
    </div>

    {@render children?.()}
{/if}
