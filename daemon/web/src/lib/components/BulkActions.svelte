<script lang="ts">
    import type { SvelteSet } from 'svelte/reactivity';
    import { ManifestEntry } from '$lib/manifest.svelte';
    import { req } from '$lib/utils.svelte';
    import { add_error } from '$lib/action_errors.svelte';
    import {
        delete_sequentially,
        downloadable,
        quick_select,
        zip_recordings_url,
        type QuickSelect,
    } from '$lib/recording_selection';
    import DownloadLink from './DownloadLink.svelte';
    import Spinner from './Spinner.svelte';
    import TrashIcon from './TrashIcon.svelte';

    let { entries, selected }: { entries: ManifestEntry[]; selected: SvelteSet<string> } = $props();

    const presets: [QuickSelect, string][] = [
        ['all', 'All'],
        ['none', 'None'],
        ['with_warnings', 'With warnings'],
        ['no_warnings', 'No warnings'],
        ['empty', 'Empty'],
    ];

    let selected_entries = $derived(entries.filter((e) => selected.has(e.name)));
    let to_download = $derived(downloadable(entries, selected));
    let num_empty = $derived(selected_entries.length - to_download.length);
    let progress: { done: number; total: number } | undefined = $state(undefined);

    function plural(n: number, noun: string): string {
        return `${n} ${noun}${n === 1 ? '' : 's'}`;
    }

    function apply_preset(preset: QuickSelect) {
        selected.clear();
        for (const name of quick_select(entries, preset)) {
            selected.add(name);
        }
    }

    async function delete_selected() {
        const targets = selected_entries;
        const with_warnings = targets.filter((e) => (e.get_num_warnings() ?? 0) > 0).length;
        const unanalysed = targets.filter((e) => e.get_num_warnings() === undefined).length;
        let prompt = `Are you sure you want to delete ${plural(targets.length, 'selected recording')}?`;
        const of_them = (n: number) => `${n} of them ${n === 1 ? 'has' : 'have'}`;
        if (with_warnings > 0) {
            prompt += `\n\n${of_them(with_warnings)} warnings.`;
        }
        if (unanalysed > 0) {
            prompt += `\n${of_them(unanalysed)} not finished analysis.`;
        }
        if (!window.confirm(prompt)) {
            return;
        }

        const urls = new Map(targets.map((e) => [e.name, e.get_delete_url()]));
        const result = await delete_sequentially(
            [...urls.keys()],
            (name) => req('POST', urls.get(name)!),
            (done, total) => (progress = { done, total })
        );
        progress = undefined;
        for (const name of result.deleted) {
            selected.delete(name);
        }
        if (result.failed.length > 0) {
            add_error(
                new Error(result.failed.map((f) => `${f.name}: ${f.error}`).join('\n')),
                `Unable to delete ${result.failed.length} of ${plural(targets.length, 'selected recording')}`
            );
        }
    }
</script>

<!-- one row on wide screens; presets, then count and actions, stacked on narrow ones -->
<div class="flex flex-col lg:flex-row lg:items-center gap-2">
    <div class="flex flex-row flex-wrap items-center gap-2">
        <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Select:</span>
        {#each presets as [preset, label] (preset)}
            <button
                class="px-3 py-1 text-sm bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 disabled:opacity-50 border border-gray-300 dark:border-gray-600 rounded-md whitespace-nowrap"
                disabled={progress !== undefined}
                onclick={() => apply_preset(preset)}
            >
                {label}
            </button>
        {/each}
    </div>
    {#if selected_entries.length > 0}
        <div class="flex flex-row flex-wrap items-center gap-2 lg:flex-1">
            <span class="text-sm text-gray-500 dark:text-gray-400 flex-1">
                {selected_entries.length} selected{num_empty > 0
                    ? ` (${num_empty} empty, skipped when downloading)`
                    : ''}
            </span>
            <div class="flex flex-row flex-wrap gap-2">
                {#if to_download.length > 0 && progress === undefined}
                    <div class="shrink-0">
                        <DownloadLink
                            url={zip_recordings_url(to_download.map((e) => e.name))}
                            text="Download selected"
                            full_button
                        />
                    </div>
                {/if}
                <button
                    class="shrink-0 whitespace-nowrap bg-red-500 hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed text-white font-bold py-2 px-2 sm:px-4 rounded-md flex flex-row items-center gap-1"
                    disabled={progress !== undefined}
                    onclick={delete_selected}
                >
                    {#if progress}
                        <span>
                            Deleting {Math.min(progress.done + 1, progress.total)}/{progress.total}
                        </span>
                        <Spinner class="w-4 h-4 text-white" />
                    {:else}
                        <span>Delete selected</span>
                        <TrashIcon class="w-6 h-6 text-white" />
                    {/if}
                </button>
            </div>
        </div>
    {/if}
</div>
