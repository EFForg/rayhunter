<script lang="ts">
    import type { Snippet } from 'svelte';

    type Severity = 'error' | 'warning' | 'info';

    let {
        severity,
        title,
        class: className = '',
        titleClass = 'text-xl gap-2',
        iconClass = 'w-6 h-6',
        icon,
        actions,
        children,
    }: {
        severity: Severity;
        title: string;
        class?: string;
        titleClass?: string;
        iconClass?: string;
        icon?: Snippet;
        actions?: Snippet;
        children: Snippet;
    } = $props();

    const severity_classes = {
        error: {
            container: 'bg-red-100 dark:bg-red-950 border-red-100 dark:border-red-900',
            title: 'text-red-600 dark:text-red-400',
            icon: 'text-red-600 dark:text-red-400',
        },
        warning: {
            container: 'bg-yellow-100 dark:bg-yellow-950 border-yellow-400 dark:border-yellow-700',
            title: 'text-yellow-700 dark:text-yellow-300',
            icon: 'text-yellow-600 dark:text-yellow-400',
        },
        info: {
            container: 'bg-sky-100 dark:bg-sky-950 border-sky-300 dark:border-sky-800',
            title: 'text-sky-800 dark:text-sky-200',
            icon: 'text-sky-700 dark:text-sky-300',
        },
    };

    let colors = $derived(severity_classes[severity]);
</script>

{#snippet heading()}
    <span class="font-bold flex flex-row items-center {colors.title} {titleClass}">
        {#if icon}
            {@render icon()}
        {:else}
            <svg
                class="{iconClass} {colors.icon}"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="currentColor"
                viewBox="0 0 24 24"
            >
                <path
                    fill-rule="evenodd"
                    d="M2 12C2 6.477 6.477 2 12 2s10 4.477 10 10-4.477 10-10 10S2 17.523 2 12Zm11-4a1 1 0 1 0-2 0v5a1 1 0 1 0 2 0V8Zm-1 7a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H12Z"
                    clip-rule="evenodd"
                />
            </svg>
        {/if}
        {title}
    </span>
{/snippet}

<div
    class="drop-shadow-sm p-4 flex flex-col gap-2 border rounded-md {colors.container} {className}"
>
    {#if actions}
        <div class="flex flex-row justify-between">
            {@render heading()}
            {@render actions()}
        </div>
    {:else}
        {@render heading()}
    {/if}
    {@render children()}
</div>
