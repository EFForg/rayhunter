export class ArgMenuInputData {
    booleans: Record<string, boolean> = $state({});
    strings: Record<string, string> = $state({});
}

export interface InstallerCommand {
    subcommands: InstallerSubcommand[];
}

export interface InstallerSubcommand {
    arguments: InstallerArgument[];
    command: string;
    label: string;
    show_network_setup: boolean;
}

export interface InstallerArgument {
    advanced: boolean;
    // default_values will almost always contain a single element. clap allows for
    // multiple default values for options that take multiple arguments.
    default_values: string[];
    flag: string;
    help: string;
    label: string;
    takes_values: boolean;
}
