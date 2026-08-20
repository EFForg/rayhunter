<script lang="ts">
    import { ArgMenuInputData } from '$lib/types.svelte';
    import StylizedButton from '$lib/StylizedButton.svelte';
    import type { InstallerArgument, InstallerSubcommand } from '$lib/types.svelte';

    let {
        inputData = $bindable(new ArgMenuInputData()),
        reselect_device,
        set_args,
        subcommand,
    }: {
        inputData: ArgMenuInputData;
        reselect_device: () => void;
        set_args: (args: string[]) => void;
        subcommand: InstallerSubcommand;
    } = $props();

    let basic_args = $derived(subcommand.arguments.filter((arg) => !arg.advanced));
    let advanced_args = $derived(subcommand.arguments.filter((arg) => arg.advanced));

    function submit_args(): void {
        let args = [subcommand.command];

        for (const [flag, value] of Object.entries(inputData.strings)) {
            if (value) {
                args.push(flag, value);
            }
        }
        for (const [flag, value] of Object.entries(inputData.booleans)) {
            if (value) {
                args.push(flag);
            }
        }

        set_args(args);
    }
</script>

{#snippet submenu(args: InstallerArgument[])}
    {#each args as arg (arg.flag)}
        {#if arg.takes_values}
            <label class="block font-medium mb-1 text-gray-700 text-sm" for={arg.flag}>
                {arg.label}
            </label>
            <input
                bind:value={inputData.strings[arg.flag]}
                class="bg-white border border-gray-300 focus:outline-hidden focus:ring-2 focus:ring-rayhunter-blue px-3 py-2 rounded-md w-full"
                id={arg.flag}
            />
        {:else}
            <div class="flex items-center">
                <input
                    autocapitalize="off"
                    autocorrect="off"
                    bind:checked={inputData.booleans[arg.flag]}
                    class="h-4 w-4"
                    id={arg.flag}
                    spellcheck="false"
                    type="checkbox"
                />
                <label for={arg.flag} class="font-medium ml-2 text-sm text-gray-700">
                    {arg.label}
                </label>
            </div>
        {/if}
    {/each}
{/snippet}

<div class="max-w-1/2 mb-4 mx-auto space-y-4">
    <h1 class="font-semibold text-center text-2xl">{subcommand.label}</h1>
    {#if basic_args.length > 0}
        {@render submenu(basic_args)}
    {:else}
        <p class="text-center text-md">
            {#if advanced_args.length > 0}
                Set advanced settings if desired and then click install.
            {:else}
                Click install to begin the installation process.
            {/if}
        </p>
    {/if}
    {#if advanced_args.length > 0}
        <details class="cursor-pointer open:bg-gray-50">
            <summary class="text-gray-700 text-sm">Advanced Settings</summary>
            <div class="p-4 space-y-4">
                {@render submenu(advanced_args)}
            </div>
        </details>
    {/if}
    <div class="flex justify-evenly">
        <StylizedButton label="Back" onclick={reselect_device} />
        <StylizedButton color="blue" label="Install" onclick={submit_args} />
    </div>
</div>
