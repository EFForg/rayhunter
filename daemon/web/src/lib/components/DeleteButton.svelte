<script lang="ts">
    import { user_action_req } from '$lib/utils.svelte';
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
    <svg style="width:24px;height:24px" viewBox="0 0 24 24">
        <path
            fill="white"
            d="M19,4H15.5L14.5,3H9.5L8.5,4H5V6H19M6,19A2,2 0 0,0 8,21H16A2,2 0 0,0 18,19V7H6V19Z"
        />
    </svg>
</button>
