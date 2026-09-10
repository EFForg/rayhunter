<script lang="ts">
    let {
        id,
        label,
        checked = $bindable(false),
        disabled = false,
        onchange,
        help,
    }: {
        id: string;
        label: string;
        checked?: boolean;
        disabled?: boolean;
        onchange?: (event: Event) => void;
        help?: string;
    } = $props();
</script>

<div class="flex items-center">
    <input
        {id}
        type="checkbox"
        {disabled}
        {onchange}
        bind:checked
        class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded-sm"
    />
    <label for={id} class="ml-2 block text-sm text-gray-700">
        {label}
    </label>
</div>
{#if help}
    <p class="text-xs text-gray-500">
        {help}
    </p>
{/if}
