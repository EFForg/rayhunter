<script lang="ts">
    import type { UpdateStatus } from '$lib/utils.svelte';

    let { status = null }: { status: UpdateStatus | null } = $props();

    let is_visible = $derived(
        Boolean(status?.update_available && status.latest_version && status.latest_release_url)
    );
</script>

{#if is_visible && status}
    <div class="bg-sky-100 border-sky-300 drop-shadow-sm p-4 flex flex-col gap-2 border rounded-md">
        <span class="text-xl font-bold flex flex-row items-center gap-2 text-sky-800">
            <svg
                class="w-6 h-6 text-sky-700"
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
            Software Update Available
        </span>
        <p>
            A new version of Rayhunter is available! You are currently running version {status.current_version},
            and the latest release is version {status.latest_version}.
        </p>
        <div class="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
            <span class="text-sm text-sky-900/80">
                View the latest release on GitHub to see what's new and download the update.
            </span>
            <a
                class="inline-flex items-center justify-center rounded-md bg-sky-700 px-4 py-2 text-white font-semibold hover:bg-sky-800"
                href={status.latest_release_url}
                target="_blank"
                rel="noreferrer noopener"
                aria-label="View latest release on GitHub"
            >
                View Release
            </a>
        </div>
    </div>
{/if}
