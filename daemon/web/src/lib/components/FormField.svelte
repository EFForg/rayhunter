<script lang="ts">
    import type { Snippet } from 'svelte';

    let {
        id,
        label,
        help,
        children,
    }: {
        id: string;
        label: string;
        help?: string | Snippet;
        children: Snippet;
    } = $props();
</script>

<div>
    <label for={id} class="block text-sm font-medium text-gray-700 mb-1">
        {label}
    </label>
    {@render children()}
    {#if help}
        <p class="text-xs text-gray-500 mt-1">
            {#if typeof help === 'string'}
                {help}
            {:else}
                {@render help()}
            {/if}
        </p>
    {/if}
</div>
