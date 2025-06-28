<script lang="ts">
    import { req } from '$lib/utils.svelte';
    import DeleteButton from './DeleteButton.svelte';

    function confirmDelete() {
        if (window.confirm(`Permanently delete ALL recordings?`)) {
            req('POST', '/api/delete-all-recordings');
        }
    }
</script>

<div class="flex flex-row justify-end gap-2">
    <DeleteButton
        text="Delete ALL Recordings"
        prompt={`Are you sure you want to delete ALL recordings?`}
        url={`/api/delete-all-recordings`}
    />
</div>
