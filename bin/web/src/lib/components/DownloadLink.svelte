<script lang="ts">
    let { url, text }: {
        url: string;
        text: string;
    } = $props();

    function download() {
        window.location.href = url;
    }
</script>

<button class="text-blue-600 flex flex-row underline" onclick={download}>
    {text} <svg class="fill-current w-4 h-4 m-1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
        <path d="M13 8V2H7v6H2l8 8 8-8h-5zM0 18h20v2H0v-2z"/>
    </svg>
</button>
