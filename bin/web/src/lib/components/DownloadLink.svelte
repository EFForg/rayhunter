<script lang="ts">
    let {
        url,
        text,
        full_button = false
    }: {
        url: string;
        text: string;
        full_button?: boolean;
    } = $props();

    function download() {
        window.location.href = url;
    }
</script>

<button
    class="flex flex-row {full_button
        ? 'bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-md'
        : 'text-blue-600 underline'}"
    onclick={download}
>
    {text}
    <svg class="fill-current w-4 h-4 m-1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
        <path d="M13 8V2H7v6H2l8 8 8-8h-5zM0 18h20v2H0v-2z" />
    </svg>
</button>
