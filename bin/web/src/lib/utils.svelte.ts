import { Manifest } from "./manifest.svelte";
import type { SystemStats } from "./systemStats";

export interface AnalyzerConfig {
    imsi_requested: boolean;
    connection_redirect_2g_downgrade: boolean;
    lte_sib6_and_7_downgrade: boolean;
    null_cipher: boolean;
}

export interface Config {
    ui_level: number;
    colorblind_mode: boolean;
    key_input_mode: number;
    ntfy_topic: string;
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
        body: JSON.stringify(config)
    });
    
    if (!response.ok) {
        const error = await response.text();
        throw new Error(error);
    }
}
