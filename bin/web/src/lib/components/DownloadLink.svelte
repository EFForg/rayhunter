<script lang="ts">
    let { url, text }: {
        url: string;
        text: string;
    } = $props();
</script>

<a href={url}>📥 {text}</a>

<style>
    a {
        @apply underline text-blue-400;
    }
</style>
