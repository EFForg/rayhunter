<script lang="ts">
    import { get_logs } from '$lib/utils.svelte';
    import Modal from './Modal.svelte';

    let { shown = $bindable() }: { shown: boolean } = $props();
    let content: string | undefined = $state(undefined);

    $effect(() => {
        const interval = setInterval(async () => {
            try {
                if (content !== undefined && (document.hidden || !shown)) {
                    return;
                }
                content = await get_logs();
            } catch (error) {
                console.log(error);
            }
        }, 1000);

        return () => clearInterval(interval);
    });
</script>

<Modal bind:shown title="Logs">
    <div class="bg-gray-100 border border-gray-100 rounded-md overflow-scroll">
        <pre class="m-2">{content}</pre>
    </div>
</Modal>
