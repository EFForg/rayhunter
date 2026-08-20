<script lang="ts">
    import { onDestroy, tick } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { emit, listen } from '@tauri-apps/api/event';
    import { exit } from '@tauri-apps/plugin-process';

    import StylizedButton from '$lib/StylizedButton.svelte';

    type InstallerState = 'Running' | 'Succeeded' | 'Failed';

    let {
        deviceName,
        installerArgs,
        reselect_args,
        reselect_device,
    }: {
        deviceName: string;
        installerArgs: string[];
        reselect_args: () => void;
        reselect_device: () => void;
    } = $props();

    let currentState: InstallerState = $state('Running');
    let installerOutput = $state('');
    let outputDiv: HTMLDivElement | undefined;

    const listenPromise = listen<string>('installer-output', (event) => {
        // The number in the comparison specifies the number of pixels the
        // scrollbar needs to be from the bottom to autoscroll as new text is
        // added. The current value was chosen somewhat arbitrarily.
        const autoscroll =
            outputDiv && outputDiv.scrollHeight - outputDiv.scrollTop - outputDiv.clientHeight < 25;

        installerOutput += event.payload;

        if (autoscroll) {
            tick().then(() => {
                if (outputDiv) {
                    outputDiv.scrollTop = outputDiv.scrollHeight;
                }
            });
        }
    });

    onDestroy(async () => {
        const unlisten = await listenPromise;
        unlisten();
    });

    async function run_installer() {
        // We await to ensure the listener is set up before continuing.
        await listenPromise;

        try {
            await invoke('install_rayhunter', { args: installerArgs });
            currentState = 'Succeeded';
        } catch (error) {
            // Just to be safe and ensure correct output order, we use emit
            // rather than modifying installerOutput directly.
            emit('installer-output', error);
            currentState = 'Failed';
        }
    }

    run_installer();
</script>

<div class="mx-8">
    <h1 class="font-semibold mb-4 text-center text-2xl">
        {#if currentState === 'Running'}
            Installing on {deviceName}
        {:else if currentState === 'Succeeded'}
            Installation succeeded!
        {:else}
            Installation failed!
        {/if}
    </h1>
    <div
        bind:this={outputDiv}
        class="bg-gray-800 max-h-[60vh] 2xl:max-h-[70vh] overflow-y-auto p-4 rounded-xl text-gray-200 whitespace-pre-line"
    >
        {installerOutput}
    </div>
    {#if currentState !== 'Running'}
        <div class="flex justify-evenly mt-6">
            {#if currentState === 'Succeeded'}
                <StylizedButton label="Install another device" onclick={reselect_device} />
                <StylizedButton color="blue" label="Exit" onclick={() => exit(0)} />
            {:else}
                <StylizedButton color="blue" label="Retry" onclick={reselect_args} />
                <a
                    class="rayhunter-button"
                    href="https://github.com/EFForg/rayhunter/issues"
                    target="_blank"
                >
                    Report Issue
                </a>
            {/if}
        </div>
    {/if}
</div>
