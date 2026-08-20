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
}

export interface InstallerArgument {
    advanced: boolean;
    flag: string;
    label: string;
    takes_values: boolean;
}
