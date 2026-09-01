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

{#snippet argument_label(arg: InstallerArgument)}
    <label for={arg.flag} class="font-medium text-sm text-gray-700">
        {arg.label}
    </label>
    {#if arg.help}
        <span class="group relative inline-flex">
            <button
                aria-label={`${arg.label}: ${arg.help}`}
                class="border border-gray-400 flex h-4 items-center justify-center rounded-full text-[10px] text-gray-600 w-4"
                type="button"
            >
                ?
            </button>
            <span
                class="absolute bottom-full left-1/2 mb-2 hidden max-w-72 -translate-x-1/2 rounded bg-gray-900 px-2 py-1 text-xs text-white whitespace-normal w-max z-10 group-focus-within:block group-hover:block"
                role="tooltip"
            >
                {arg.help}
            </span>
        </span>
    {/if}
{/snippet}

{#snippet submenu(args: InstallerArgument[])}
    {#each args as arg (arg.flag)}
        {#if arg.takes_values}
            <div class="flex items-center gap-1 mb-1">
                {@render argument_label(arg)}
            </div>
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
                <div class="flex items-center gap-1 ml-2">
                    {@render argument_label(arg)}
                </div>
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
