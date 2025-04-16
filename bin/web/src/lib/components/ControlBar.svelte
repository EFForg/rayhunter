<script lang="ts">
	import { req } from "$lib/utils.svelte";
	import DeleteButton from "./DeleteButton.svelte";
	import RecordingControls from "./RecordingControls.svelte";
	let { server_is_recording }: {
        server_is_recording: boolean;
    } = $props();

    function confirmDelete() {
        if (window.confirm(`Permanently delete ALL entries?`)) {
            req('POST', '/api/delete-all-recordings')
        }
    }
</script>

<div class="flex flex-row gap-2">
    <RecordingControls {server_is_recording} />
    <DeleteButton
        text="Delete ALL Entries"
        prompt={`Are you sure you want to delete ALL entries?`}
        url={`/api/delete-all-recordings`}
    />
</div>
