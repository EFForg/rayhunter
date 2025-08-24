<script lang="ts">
    import { get_logs } from '$lib/utils.svelte';
    import { onMount } from 'svelte';

    let { shown = $bindable() }: { shown: boolean } = $props();
    let content: string | undefined = $state(undefined);

    onMount(() => {
        // Used by LogView modal
        window.addEventListener('scroll', () => {
            document.documentElement.style.setProperty('--scroll-y', `${window.scrollY}px`);
        });
    });

    $effect(() => {
        if (shown) {
            const scrollY = document.documentElement.style.getPropertyValue('--scroll-y');
            const body = document.body;
            body.style.position = 'fixed';
            body.style.top = `-${scrollY}`;
        } else {
            const body = document.body;
            const scrollY = body.style.top;
            body.style.position = '';
            body.style.top = '';
            window.scrollTo(0, parseInt(scrollY || '0') * -1);
        }

        const interval = setInterval(async () => {
            try {
                // Don't update UI if browser tab isn't visible
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

{#if shown}
    <div
        class="fixed left-5 right-5 top-5 bottom-5 z-50 bg-white border border-white rounded-md
		flex flex-col p-2 drop-shadow"
    >
        <div class="flex h-20 justify-between items-center p-1">
            <span class="text-2xl mb-2">Log</span>
            <button onclick={() => (shown = false)} aria-label="close">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    aria-hidden="true"
                    width="24"
                    height="24"
                    fill="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        d="M5.29289 5.29289C5.68342 4.90237 6.31658 4.90237 6.70711 5.29289L12 10.5858L17.2929 5.29289C17.6834 4.90237 18.3166 4.90237 18.7071 5.29289C19.0976 5.68342 19.0976 6.31658 18.7071 6.70711L13.4142 12L18.7071 17.2929C19.0976 17.6834 19.0976 18.3166 18.7071 18.7071C18.3166 19.0976 17.6834 19.0976 17.2929 18.7071L12 13.4142L6.70711 18.7071C6.31658 19.0976 5.68342 19.0976 5.29289 18.7071C4.90237 18.3166 4.90237 17.6834 5.29289 17.2929L10.5858 12L5.29289 6.70711C4.90237 6.31658 4.90237 5.68342 5.29289 5.29289Z"
                        fill="#0F1729"
                    />
                </svg>
            </button>
        </div>
        <div class="bg-gray-100 border border-gray-100 rounded-md overflow-scroll">
            <pre class="m-2">{content}</pre>
        </div>
    </div>
{/if}
