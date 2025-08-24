import { add_error } from './action_errors.svelte';
import { Manifest } from './manifest.svelte';
import type { SystemStats } from './systemStats';

export interface AnalyzerConfig {
    imsi_requested: boolean;
    connection_redirect_2g_downgrade: boolean;
    lte_sib6_and_7_downgrade: boolean;
    null_cipher: boolean;
    nas_null_cipher: boolean;
    incomplete_sib: boolean;
}

export interface Config {
    ui_level: number;
    colorblind_mode: boolean;
    key_input_mode: number;
    analyzers: AnalyzerConfig;
}

export async function req(method: string, url: string): Promise<string> {
    const response = await fetch(url, {
        method: method,
    });
    const body = await response.text();
    if (response.status >= 200 && response.status < 300) {
        return body;
    } else {
        throw new Error(body);
    }
}

// A wrapper around req that reports errors to the UI
export async function user_action_req(
    method: string,
    url: string,
    error_msg: string
): Promise<string | undefined> {
    try {
        return await req(method, url);
    } catch (error) {
        if (error instanceof Error) {
            console.log('beeeo');
            add_error(error, error_msg);
        }
        return undefined;
    }
}

export async function get_manifest(): Promise<Manifest> {
    const manifest_json = JSON.parse(await req('GET', '/api/qmdl-manifest'));
    return new Manifest(manifest_json);
}

export async function get_system_stats(): Promise<SystemStats> {
    return JSON.parse(await req('GET', '/api/system-stats'));
}

export async function get_config(): Promise<Config> {
    return JSON.parse(await req('GET', '/api/config'));
}

export async function set_config(config: Config): Promise<void> {
    const response = await fetch('/api/config', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(config),
    });

    if (!response.ok) {
        const error = await response.text();
        throw new Error(error);
    }
}
