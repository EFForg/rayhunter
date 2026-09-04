<script lang="ts">
    import type { UpdateStatus } from '$lib/utils.svelte';
    import Alert from './Alert.svelte';

    let { status = null }: { status: UpdateStatus | null } = $props();

    let is_visible = $derived(
        Boolean(status?.update_available && status.latest_version && status.latest_release_url)
    );
</script>

{#if is_visible && status}
    <Alert severity="info" title="Software Update Available">
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
    </Alert>
{/if}
