<script lang="ts">
    import StylizedButton from '$lib/StylizedButton.svelte';
    import type { InstallerArgument } from '$lib/types.svelte';

    let {
        on_back,
        on_install,
        allArgs,
        setArgs,
    }: {
        on_back: () => void;
        on_install: () => void;
        allArgs: InstallerArgument[];
        setArgs: string[];
    } = $props();

    let adminIP = $derived.by(() => {
        const flagIndex = setArgs.findIndex((arg) => arg == '--admin-ip');
        return flagIndex !== -1
            ? setArgs[flagIndex + 1]
            : allArgs.find((arg) => arg.flag == '--admin-ip')?.default_values[0];
    });
</script>

<div class="max-w-3/4 mx-auto space-y-2 text-lg">
    <h1 class="font-bold mb-4 text-center text-2xl">Connect your device</h1>
    <p>
        Turn on your device and then connect to it using either WiFi or USB tethering.
        {#if adminIP !== undefined}
            You know you are in the right network when you can access
            <a class="text-blue-600 underline" href="http://{adminIP}" target="_blank"
                >http://{adminIP}</a
            >
            and see the hardware's own admin menu.
        {/if}
    </p>
    <p>Once you have connected, click install.</p>
    <div class="flex justify-evenly mt-6">
        <StylizedButton label="Back" onclick={on_back} />
        <StylizedButton color="blue" label="Install" onclick={on_install} />
    </div>
</div>
