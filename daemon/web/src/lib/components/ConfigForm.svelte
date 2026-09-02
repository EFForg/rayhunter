<script lang="ts">
    import {
        get_config,
        set_config,
        test_notification,
        get_wifi_status,
        scan_wifi_networks,
        GpsMode,
        ClockSyncMode,
        enabled_notifications,
        type Config,
        type WifiStatus,
        type WifiNetwork,
    } from '../utils.svelte';
    import Modal from './Modal.svelte';
    import ExpandableInput from './ExpandableInput.svelte';
    import CheckboxField from './CheckboxField.svelte';
    import FormField from './FormField.svelte';

    let { shown = $bindable() }: { shown: boolean } = $props();
    let config = $state<Config | null>(null);

    let loading = $state(false);
    let saving = $state(false);
    let testingNotification = $state(false);
    let message = $state('');
    let messageType = $state<'success' | 'error' | null>(null);
    let testMessage = $state('');
    let testMessageType = $state<'success' | 'error' | null>(null);
    let wifiStatus = $state<WifiStatus | null>(null);
    let wifiStatusTimer = $state<ReturnType<typeof setInterval> | null>(null);
    let scanning = $state(false);
    let scanResults = $state<WifiNetwork[]>([]);
    let dnsServersInput = $state('');
    let gpsMode = $derived(config?.gps_mode);

    async function load_config() {
        try {
            loading = true;
            config = await get_config();
            dnsServersInput = config.dns_servers ? config.dns_servers.join(', ') : '';
            message = '';
            messageType = null;
            poll_wifi_status();
        } catch (error) {
            message = `Failed to load config: ${error}`;
            messageType = 'error';
        } finally {
            loading = false;
        }
    }

    async function save_config() {
        if (!config) return;

        const trimmed = dnsServersInput.trim();
        config.dns_servers =
            trimmed.length > 0
                ? trimmed
                      .split(',')
                      .map((s) => s.trim())
                      .filter((s) => s.length > 0)
                : null;

        try {
            saving = true;
            await set_config(config);
            message =
                'Config saved successfully! Rayhunter is restarting now. Reload the page in a few seconds.';
            messageType = 'success';
        } catch (error) {
            message = `Failed to save config: ${error}`;
            messageType = 'error';
        } finally {
            saving = false;
        }
    }

    async function poll_wifi_status() {
        if (wifiStatusTimer) clearInterval(wifiStatusTimer);
        try {
            wifiStatus = await get_wifi_status();
        } catch {
            wifiStatus = null;
        }
        wifiStatusTimer = setInterval(async () => {
            try {
                wifiStatus = await get_wifi_status();
            } catch {
                wifiStatus = null;
            }
        }, 5000);
    }

    let scanError = $state('');

    async function do_scan() {
        scanning = true;
        scanError = '';
        try {
            scanResults = await scan_wifi_networks();
        } catch (error) {
            scanResults = [];
            scanError = `Scan failed: ${error}`;
        } finally {
            scanning = false;
        }
    }

    function select_network(network: WifiNetwork) {
        if (config) {
            config.wifi_ssid = network.ssid;
            config.wifi_password = '';
            config.wifi_security =
                network.security === 'WPA3' || network.security === 'WPA3 (transition)'
                    ? 'sae'
                    : 'wpa_psk';
            scanResults = [];
        }
    }

    async function send_test_notification() {
        try {
            testingNotification = true;
            testMessage = '';
            testMessageType = null;
            await test_notification();
            testMessage = 'Test notification sent successfully!';
            testMessageType = 'success';
        } catch (error) {
            testMessage = `${error}`;
            testMessageType = 'error';
        } finally {
            testingNotification = false;
        }
    }

    $effect(() => {
        if (shown && !config) {
            load_config();
        }
        if (!shown && wifiStatusTimer) {
            clearInterval(wifiStatusTimer);
            wifiStatusTimer = null;
        }
        return () => {
            if (wifiStatusTimer) {
                clearInterval(wifiStatusTimer);
                wifiStatusTimer = null;
            }
        };
    });
</script>

<Modal bind:shown title="Configuration">
    <div class="p-2">
        {#if loading}
            <div class="text-center py-4">Loading config...</div>
        {:else if config}
            <form
                class="space-y-4"
                onsubmit={(e) => {
                    e.preventDefault();
                    save_config();
                }}
            >
                <FormField
                    id="ui_level"
                    label="Device UI Level"
                    help="Note: Rayhunter draws over the device's native UI, so some flickering is expected"
                >
                    <select id="ui_level" bind:value={config.ui_level} class="form-control w-full">
                        <option value={0}>Invisible mode</option>
                        <option value={1}>Subtle mode (colored line)</option>
                        <option value={2}>Demo mode (orca gif)</option>
                        <option value={3}>EFF logo</option>
                        <option value={4}>High visibility (full screen color)</option>
                    </select>
                </FormField>

                <FormField id="key_input_mode" label="Device Input Mode">
                    <select
                        id="key_input_mode"
                        bind:value={config.key_input_mode}
                        class="form-control w-full"
                    >
                        <option value={0}>Disable button control</option>
                        <option value={1}>Double-tap power button to start new recording</option>
                    </select>
                </FormField>

                <div class="space-y-3">
                    <CheckboxField
                        id="colorblind_mode"
                        label="Colorblind Mode"
                        bind:checked={config.colorblind_mode}
                    />
                </div>

                <FormField
                    id="clock_sync_mode"
                    label="Clock Sync"
                    help="What to do when Rayhunter's clock drifts from your browser's. The offset isn't saved across daemon restarts, so autosync re-applies it whenever you open the web UI."
                >
                    <select
                        id="clock_sync_mode"
                        bind:value={config.clock_sync_mode}
                        class="form-control w-full"
                    >
                        <option value={ClockSyncMode.Off}>Off (never warn or sync)</option>
                        <option value={ClockSyncMode.Autosync}>
                            Autosync (copy browser clock automatically)
                        </option>
                        <option value={ClockSyncMode.Prompt}>Prompt (ask before syncing)</option>
                    </select>
                </FormField>

                <div class="border-t border-gray-200 pt-4 mt-6 space-y-3">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">Notification Settings</h3>

                    <CheckboxField
                        id="auto_check_updates"
                        label="Automatically check for software updates"
                        bind:checked={config.auto_check_updates}
                        help="When enabled, Rayhunter periodically checks GitHub for new releases and shows an update notice in the web UI."
                    />

                    <ExpandableInput
                        bind:value={config.ntfy_url}
                        checkboxId="ntfy_enabled"
                        inputId="ntfy_url"
                        label="Enable ntfy notifications"
                        inputLabel="ntfy URL"
                        inputPlaceholder="https://ntfy.sh/my-rayhunter"
                        inputHelp="Test button below uses the saved configuration URL, not the input above"
                    >
                        <div>
                            <button
                                type="button"
                                onclick={send_test_notification}
                                disabled={testingNotification}
                                class="bg-rayhunter-blue hover:bg-rayhunter-dark-blue disabled:opacity-50 disabled:cursor-not-allowed text-white font-bold py-2 px-4 rounded-md flex flex-row gap-1 items-center"
                            >
                                {#if testingNotification}
                                    <div
                                        class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"
                                    ></div>
                                    Sending...
                                {:else}
                                    <svg
                                        class="w-4 h-4"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"
                                        ></path>
                                    </svg>
                                    Send Test Notification
                                {/if}
                            </button>
                            {#if testMessage}
                                <div
                                    class="mt-2 p-2 rounded-sm text-sm {testMessageType === 'error'
                                        ? 'bg-red-100 text-red-700'
                                        : 'bg-green-100 text-green-700'}"
                                >
                                    {testMessage}
                                </div>
                            {/if}
                        </div>

                        <div class="space-y-2">
                            <div class="block text-sm font-medium text-gray-700 mb-1">
                                Enabled Notification Types
                            </div>
                            <div class="flex items-center">
                                <input
                                    type="checkbox"
                                    id="enable_warning_notifications"
                                    value="Warning"
                                    bind:group={config.enabled_notifications}
                                    class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded-sm"
                                />
                                <label
                                    for="enable_warning_notifications"
                                    class="ml-2 block text-sm text-gray-700"
                                >
                                    Warnings
                                </label>
                            </div>
                            <div class="flex items-center">
                                <input
                                    type="checkbox"
                                    id="enable_lowbattery_notifications"
                                    value="LowBattery"
                                    bind:group={config.enabled_notifications}
                                    class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded-sm"
                                />
                                <label
                                    for="enable_lowbattery_notifications"
                                    class="ml-2 block text-sm text-gray-700"
                                >
                                    Low Battery
                                </label>
                            </div>
                            <div class="flex items-center">
                                <input
                                    type="checkbox"
                                    id="enable_update_notifications"
                                    value={enabled_notifications.Update}
                                    bind:group={config.enabled_notifications}
                                    class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded-sm"
                                />
                                <label
                                    for="enable_update_notifications"
                                    class="ml-2 block text-sm text-gray-700"
                                >
                                    Software Updates
                                </label>
                            </div>
                        </div>
                    </ExpandableInput>
                </div>

                <div class="border-t border-gray-200 pt-4 mt-6 space-y-3">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">Storage Management</h3>

                    <FormField
                        id="min_space_to_start_recording_mb"
                        label="Minimum Space to Start Recording (MB)"
                        help="Recording will not start if less than this amount of disk space is free"
                    >
                        <input
                            id="min_space_to_start_recording_mb"
                            type="number"
                            min="1"
                            bind:value={config.min_space_to_start_recording_mb}
                            class="form-control w-full"
                        />
                    </FormField>

                    <FormField
                        id="min_space_to_continue_recording_mb"
                        label="Minimum Space to Continue Recording (MB)"
                        help="Recording will stop automatically if disk space drops below this level"
                    >
                        <input
                            id="min_space_to_continue_recording_mb"
                            type="number"
                            min="1"
                            bind:value={config.min_space_to_continue_recording_mb}
                            class="form-control w-full"
                        />
                    </FormField>
                </div>

                <div class="border-t border-gray-200 pt-4 mt-6 space-y-3">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">WebDAV Upload</h3>
                    <p class="text-xs text-gray-500">
                        Once a recording has been closed for at least the configured age, both the
                        .qmdl and .ndjson files are uploaded in the background to the WebDAV server.
                    </p>

                    <ExpandableInput
                        bind:value={config.webdav.url}
                        checkboxId="webdav_enabled"
                        inputId="webdav_url"
                        label="Enable WebDAV upload"
                        inputLabel="Server URL"
                        inputPlaceholder="https://dav.example.com/rayhunter/"
                        inputHelp="Files are uploaded via HTTP PUT under this base URL. No folders are created, and folders in this base URL are assumed to exist already."
                    >
                        <FormField
                            id="webdav_username"
                            label="Username"
                            help="Optional. Leave blank for unauthenticated uploads."
                        >
                            <input
                                id="webdav_username"
                                type="text"
                                bind:value={config.webdav.username}
                                class="form-control w-full"
                            />
                        </FormField>

                        <FormField
                            id="webdav_password"
                            label="Password"
                            help="A password without a username will be rejected and the request will be sent unauthenticated."
                        >
                            <input
                                id="webdav_password"
                                type="password"
                                bind:value={config.webdav.password}
                                class="form-control w-full"
                            />
                        </FormField>

                        <FormField id="webdav_upload_timeout_secs" label="Upload Timeout (seconds)">
                            <input
                                id="webdav_upload_timeout_secs"
                                type="number"
                                min="1"
                                bind:value={config.webdav.upload_timeout_secs}
                                class="form-control w-full"
                            />
                        </FormField>

                        <FormField
                            id="webdav_poll_interval_secs"
                            label="Poll Interval (seconds)"
                            help="How often the worker checks for new entries to upload."
                        >
                            <input
                                id="webdav_poll_interval_secs"
                                type="number"
                                min="1"
                                bind:value={config.webdav.poll_interval_secs}
                                class="form-control w-full"
                            />
                        </FormField>

                        <FormField
                            id="webdav_min_age_secs"
                            label="Minimum Age Before Upload (seconds)"
                            help="How long a recording must be closed before it becomes eligible for upload."
                        >
                            <input
                                id="webdav_min_age_secs"
                                type="number"
                                min="0"
                                bind:value={config.webdav.min_age_secs}
                                class="form-control w-full"
                            />
                        </FormField>

                        <CheckboxField
                            id="webdav_delete_on_upload"
                            label="Delete on successful upload"
                            bind:checked={config.webdav.delete_on_upload}
                            help="When enabled, the local files are removed after a successful upload. Otherwise the manifest is just marked as uploaded."
                        />
                    </ExpandableInput>
                </div>

                {#if config.device === 'orbic' || config.device === 'moxee' || config.device === 'tmobile' || config.device === 'wingtech'}
                    <div class="border-t border-gray-200 pt-4 mt-6 space-y-3">
                        <h3 class="text-lg font-semibold text-gray-800 mb-4">WiFi Client Mode</h3>
                        <p class="text-xs text-gray-500">
                            Connect the device to an existing WiFi network for internet access (e.g.
                            notifications, remote access). The hotspot AP stays running alongside
                            WiFi client mode.
                        </p>

                        <CheckboxField
                            id="wifi_enabled"
                            label="Enable WiFi"
                            bind:checked={config.wifi_enabled}
                            help="Unchecking stops WiFi without clearing saved credentials."
                        />

                        {#if wifiStatus && config.wifi_enabled}
                            {#if wifiStatus.state === 'connected'}
                                <p class="text-xs text-green-600">
                                    Connected to "{wifiStatus.ssid}" ({wifiStatus.ip})
                                </p>
                            {:else if wifiStatus.state === 'connecting'}
                                <p class="text-xs text-amber-600">Connecting...</p>
                            {:else if wifiStatus.state === 'recovering'}
                                <p class="text-xs text-amber-600">Recovering connection...</p>
                            {:else if wifiStatus.state === 'dataPathDead'}
                                <p class="text-xs text-amber-600">
                                    Data path stalled, attempting recovery...
                                </p>
                            {:else if wifiStatus.state === 'failed'}
                                <p class="text-xs text-red-600">
                                    Failed: {wifiStatus.error}
                                </p>
                            {/if}
                        {/if}

                        <FormField id="wifi_ssid" label="WiFi Network Name (SSID)">
                            <div class="flex gap-2">
                                <input
                                    id="wifi_ssid"
                                    type="text"
                                    bind:value={config.wifi_ssid}
                                    placeholder="MyWiFiNetwork"
                                    class="form-control flex-1"
                                />
                                <button
                                    type="button"
                                    onclick={do_scan}
                                    disabled={scanning}
                                    class="px-3 py-2 text-sm bg-gray-100 hover:bg-gray-200 disabled:opacity-50 border border-gray-300 rounded-md"
                                >
                                    {scanning ? 'Scanning...' : 'Scan'}
                                </button>
                            </div>
                        </FormField>

                        {#if scanError}
                            <p class="text-xs text-red-600">{scanError}</p>
                        {/if}

                        {#if scanResults.length > 0}
                            <div
                                class="border border-gray-200 rounded-md max-h-40 overflow-y-auto divide-y divide-gray-200"
                            >
                                {#each scanResults as network}
                                    <button
                                        type="button"
                                        class="w-full px-3 py-2 text-left text-sm hover:bg-gray-50 flex justify-between"
                                        onclick={() => select_network(network)}
                                    >
                                        <span>{network.ssid}</span>
                                        <span class="text-gray-400"
                                            >{network.signal_dbm} dBm &middot; {network.security}</span
                                        >
                                    </button>
                                {/each}
                            </div>
                        {/if}

                        {#if config.wifi_ssid}
                            <FormField id="wifi_security" label="Security Type">
                                <select
                                    id="wifi_security"
                                    bind:value={config.wifi_security}
                                    class="form-control w-full"
                                >
                                    <option value="wpa_psk">WPA2 (WPA-PSK)</option>
                                    <option value="sae">WPA3 (SAE)</option>
                                </select>
                            </FormField>
                        {/if}

                        <FormField
                            id="wifi_password"
                            label="WiFi Password"
                            help="Changing the network requires re-entering the password."
                        >
                            <input
                                id="wifi_password"
                                type="password"
                                bind:value={config.wifi_password}
                                placeholder="Enter password"
                                class="form-control w-full"
                            />
                        </FormField>

                        {#if config.wifi_ssid}
                            <FormField
                                id="dns_servers"
                                label="DNS Servers"
                                help="Comma-separated. Used when WiFi is active. Defaults to 9.9.9.9, 149.112.112.112 (Quad9)."
                            >
                                <input
                                    id="dns_servers"
                                    type="text"
                                    bind:value={dnsServersInput}
                                    placeholder="9.9.9.9, 149.112.112.112"
                                    class="form-control w-full"
                                />
                            </FormField>
                        {/if}
                    </div>
                {/if}

                <div class="border-t border-gray-200 pt-4 mt-6">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">
                        Analyzer Heuristic Settings
                    </h3>
                    <div class="space-y-3">
                        <CheckboxField
                            id="imsi_requested"
                            label="IMSI Requested Heuristic"
                            bind:checked={config.analyzers.imsi_requested}
                        />

                        <CheckboxField
                            id="connection_redirect_2g_downgrade"
                            label="Connection Redirect 2G Downgrade Heuristic"
                            bind:checked={config.analyzers.connection_redirect_2g_downgrade}
                        />

                        <CheckboxField
                            id="lte_sib6_and_7_downgrade"
                            label="LTE SIB6 and SIB7 Downgrade Heuristic"
                            bind:checked={config.analyzers.lte_sib6_and_7_downgrade}
                        />

                        <CheckboxField
                            id="null_cipher"
                            label="Null Cipher Heuristic"
                            bind:checked={config.analyzers.null_cipher}
                        />

                        <CheckboxField
                            id="nas_null_cipher"
                            label="NAS Null Cipher Heuristic"
                            bind:checked={config.analyzers.nas_null_cipher}
                        />

                        <CheckboxField
                            id="incomplete_sib"
                            label="Incomplete SIB Heuristic"
                            bind:checked={config.analyzers.incomplete_sib}
                        />

                        <CheckboxField
                            id="test_analyzer"
                            label="Test Heuristic (noisy!)"
                            bind:checked={config.analyzers.test_analyzer}
                        />
                        <CheckboxField
                            id="diagnostic_analyzer"
                            label="Diagnostic Analyzer"
                            bind:checked={config.analyzers.diagnostic_analyzer}
                        />
                    </div>
                </div>

                <div class="border-t border-gray-200 pt-4 mt-6 space-y-3">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">GPS Settings</h3>
                    <FormField id="gps_mode" label="GPS Mode">
                        {#snippet help()}
                            {#if gpsMode === GpsMode.Api}
                                POST latitude and longitude to <code>/api/gps</code> from any device on
                                the network. Timestamp is derived from packet capture timing.
                            {:else if gpsMode === GpsMode.Fixed}
                                GPS coordinates are fixed to the values below.
                            {:else}
                                GPS is disabled; no coordinates will be tracked.
                            {/if}
                        {/snippet}
                        <select
                            id="gps_mode"
                            bind:value={config.gps_mode}
                            class="form-control w-full"
                        >
                            <option value={GpsMode.Disabled}>Disabled</option>
                            <option value={GpsMode.Fixed}>Fixed coordinates</option>
                            <option value={GpsMode.Api}>API endpoint</option>
                        </select>
                    </FormField>
                    {#if config.gps_mode === GpsMode.Fixed}
                        <FormField
                            id="gps_fixed_latitude"
                            label="Fixed Latitude"
                            help="Decimal degrees, -90 to 90"
                        >
                            <input
                                id="gps_fixed_latitude"
                                type="number"
                                min="-90"
                                max="90"
                                step="any"
                                required
                                bind:value={config.gps_fixed_latitude}
                                placeholder="e.g. 37.7749"
                                class="form-control w-full"
                            />
                        </FormField>
                        <FormField
                            id="gps_fixed_longitude"
                            label="Fixed Longitude"
                            help="Decimal degrees, -180 to 180"
                        >
                            <input
                                id="gps_fixed_longitude"
                                type="number"
                                min="-180"
                                max="180"
                                step="any"
                                required
                                bind:value={config.gps_fixed_longitude}
                                placeholder="e.g. -122.4194"
                                class="form-control w-full"
                            />
                        </FormField>
                    {/if}
                </div>

                <div class="flex gap-2 pt-4">
                    <button
                        type="submit"
                        disabled={saving}
                        class="bg-blue-500 hover:bg-blue-700 disabled:opacity-50 text-white font-bold py-2 px-4 rounded-md flex flex-row gap-1 items-center"
                    >
                        {#if saving}
                            <div
                                class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"
                            ></div>
                            Saving...
                        {:else}
                            <svg
                                class="w-4 h-4"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M5 13l4 4L19 7"
                                ></path>
                            </svg>
                            Apply and restart
                        {/if}
                    </button>
                </div>
            </form>
            {#if message}
                <div
                    class="mt-4 p-3 rounded-sm {messageType === 'error'
                        ? 'bg-red-100 text-red-700'
                        : 'bg-green-100 text-green-700'}"
                >
                    {message}
                </div>
            {/if}
        {:else}
            <div class="text-center py-4 text-red-600">
                Failed to load configuration. Please try reloading the page.
            </div>
        {/if}
    </div>
</Modal>
