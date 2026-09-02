<script lang="ts">
    import type { Snippet } from 'svelte';
    import CheckboxField from './CheckboxField.svelte';
    import FormField from './FormField.svelte';

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

<CheckboxField id={checkboxId} {label} checked={expanded} onchange={handle_checkbox_change} />

{#if expanded}
    <FormField id={inputId} label={inputLabel} help={inputHelp}>
        <input
            id={inputId}
            type="text"
            bind:this={inputElement}
            bind:value
            onblur={handle_input_blur}
            placeholder={inputPlaceholder}
            class="form-control w-full"
        />
    </FormField>

    {@render children?.()}
{/if}
