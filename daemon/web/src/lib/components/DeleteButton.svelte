<script lang="ts">
    import { user_action_req } from '$lib/utils.svelte';
    import TrashIcon from './TrashIcon.svelte';
    let {
        text,
        url,
        prompt,
        name,
    }: {
        text?: string;
        url: string;
        prompt: string;
        name: string;
    } = $props();

    function confirm_delete() {
        if (window.confirm(prompt)) {
            user_action_req('POST', url, 'Unable to delete recording ' + name);
        }
    }
</script>

<button
    class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-2 sm:px-4 rounded-md flex flex-row"
    onclick={confirm_delete}
    aria-label="delete"
>
    <p>{text}</p>
    <TrashIcon class="w-6 h-6 text-white" />
</button>
