import { invoke } from '@tauri-apps/api/core';

import type { InstallerCommand } from '$lib/types.svelte';

export function load(): Promise<InstallerCommand> {
    return invoke('rayhunter_options');
}
