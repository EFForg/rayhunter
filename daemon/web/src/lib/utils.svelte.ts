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
    test_analyzer: boolean;
    diagnostic_analyzer: boolean;
}

export enum enabled_notifications {
    Warning = 'Warning',
    LowBattery = 'LowBattery',
}

export interface Config {
    ui_level: number;
    colorblind_mode: boolean;
    key_input_mode: number;
    ntfy_url: string;
    enabled_notifications: enabled_notifications[];
    analyzers: AnalyzerConfig;
}

export async function req(method: string, url: string, json_body?: unknown): Promise<string> {
    const options: RequestInit = { method };
    if (json_body !== undefined) {
        options.body = JSON.stringify(json_body);
        options.headers = { 'Content-Type': 'application/json' };
    }
    const response = await fetch(url, options);
    const responseBody = await response.text();
    if (response.status >= 200 && response.status < 300) {
        return responseBody;
    } else {
        throw new Error(responseBody);
    }
}

// A wrapper around req that reports errors to the UI
export async function user_action_req(
    method: string,
    url: string,
    error_msg: string,
    json_body?: unknown
): Promise<string | undefined> {
    try {
        return await req(method, url, json_body);
    } catch (error) {
        if (error instanceof Error) {
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

export async function get_logs(): Promise<string> {
    return await req('GET', '/api/log');
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

export async function test_notification(): Promise<void> {
    const response = await fetch('/api/test-notification', {
        method: 'POST',
    });

    if (!response.ok) {
        const error = await response.text();
        throw new Error(error);
    }
}

export interface TimeResponse {
    system_time: string;
    adjusted_time: string;
    offset_seconds: number;
}

export async function get_daemon_time(): Promise<TimeResponse> {
    return JSON.parse(await req('GET', '/api/time'));
}
