<script lang="ts">
    import type { Snippet } from 'svelte';

    let {
        label,
        icon,
        href,
        target,
        onclick,
    }: {
        label: string;
        icon: Snippet;
        href?: string;
        target?: string;
        onclick?: () => void;
    } = $props();
</script>

{#snippet content()}
    <span class="hidden text-white group-hover:text-gray-400 lg:flex">{label}</span>
    {@render icon()}
{/snippet}

{#if href}
    <a
        class="flex flex-row gap-1 group"
        {href}
        {target}
        rel={target === '_blank' ? 'noreferrer noopener' : undefined}
    >
        {@render content()}
    </a>
{:else}
    <button {onclick} class="flex flex-row gap-1 group">
        {@render content()}
    </button>
{/if}
