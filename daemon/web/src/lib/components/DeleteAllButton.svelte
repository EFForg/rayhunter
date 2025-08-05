<script lang="ts">
    import DeleteButton from './DeleteButton.svelte';
</script>

<div class="flex flex-row justify-end gap-2">
    <DeleteButton
        text="Delete ALL Recordings"
        prompt={`Are you sure you want to delete ALL recordings?`}
        url={`/api/delete-all-recordings`}
    />
</div>
