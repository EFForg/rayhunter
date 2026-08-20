<script lang="ts">
    import StylizedButton from '$lib/StylizedButton.svelte';
    import type { InstallerSubcommand } from '$lib/types.svelte';

    let {
        initialSelection: selection,
        set_device,
        subcommands,
    }: {
        initialSelection: InstallerSubcommand | null;
        set_device: (s: InstallerSubcommand) => void;
        subcommands: InstallerSubcommand[];
    } = $props();

    let buttonDisabled = $derived(selection === null);

    function onclick() {
        if (selection !== null) {
            set_device(selection);
        }
    }
</script>

<div class="flex flex-col gap-4 items-center text-xl">
    <img alt="rayhunter orca logo" class="h-35 w-35" src="/orca.svg" />
    <h1 class="font-bold text-3xl">Install Rayhunter</h1>
    <label class="text-gray-600" for="device-select">
        Select your device and installation method
    </label>
    <div class="flex gap-4">
        <select
            bind:value={selection}
            class="border border-gray-300 text-base"
            name="devices"
            id="device-select"
        >
            <option value={null}></option>
            {#each subcommands as subcommand (subcommand.command)}
                <option value={subcommand}>{subcommand.label}</option>
            {/each}
        </select>
        <StylizedButton color="blue" disabled={buttonDisabled} label="Next" {onclick} />
    </div>
</div>
