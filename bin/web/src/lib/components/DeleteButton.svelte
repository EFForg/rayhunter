<script lang="ts">
    import { ManifestEntry } from '$lib/manifest.svelte';
    import { req } from '$lib/utils.svelte';
    let {
        text,
        url,
        prompt
    }: {
        text?: string;
        url: string;
        prompt: string;
    } = $props();

    function confirmDelete() {
        if (window.confirm(prompt)) {
            req('POST', url);
        }
    }
</script>

<button
    class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded-md flex flex-row"
    onclick={confirmDelete}
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
