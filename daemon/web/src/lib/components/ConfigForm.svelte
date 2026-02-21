<script lang="ts">
    import {
        get_config,
        set_config,
        test_notification,
        get_wifi_status,
        scan_wifi_networks,
        type Config,
        type WifiStatus,
        type WifiNetwork,
    } from '../utils.svelte';

    let config = $state<Config | null>(null);

    let loading = $state(false);
    let saving = $state(false);
    let testingNotification = $state(false);
    let message = $state('');
    let messageType = $state<'success' | 'error' | null>(null);
    let testMessage = $state('');
    let testMessageType = $state<'success' | 'error' | null>(null);
    let showConfig = $state(false);
    let wifiStatus = $state<WifiStatus | null>(null);
    let wifiStatusTimer = $state<ReturnType<typeof setInterval> | null>(null);
    let scanning = $state(false);
    let scanResults = $state<WifiNetwork[]>([]);
    let dnsServersInput = $state('');

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

    async function do_scan() {
        scanning = true;
        try {
            scanResults = await scan_wifi_networks();
        } catch {
            scanResults = [];
        } finally {
            scanning = false;
        }
    }

    function select_network(ssid: string) {
        if (config) {
            config.wifi_ssid = ssid;
            config.wifi_password = '';
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
        if (showConfig && !config) {
            load_config();
        }
        if (!showConfig && wifiStatusTimer) {
            clearInterval(wifiStatusTimer);
            wifiStatusTimer = null;
        }
    });
</script>

<div class="bg-white rounded-lg shadow-md p-6 m-4">
    <button
        class="w-full flex justify-between items-center text-xl font-bold mb-4 text-rayhunter-dark-blue hover:text-rayhunter-blue"
        onclick={() => (showConfig = !showConfig)}
    >
        <span>Configuration</span>
        <svg
            class="w-6 h-6 transition-transform {showConfig ? 'rotate-180' : ''}"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
        >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"
            ></path>
        </svg>
    </button>

    {#if showConfig}
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
                <div>
                    <label for="ui_level" class="block text-sm font-medium text-gray-700 mb-1">
                        Device UI Level
                    </label>
                    <select
                        id="ui_level"
                        bind:value={config.ui_level}
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-rayhunter-blue"
                    >
                        <option value={0}>0 - Invisible mode</option>
                        <option value={1}>1 - Subtle mode (colored line)</option>
                        <option value={2}>2 - Demo mode (orca gif)</option>
                        <option value={3}>3 - EFF logo</option>
                        <option value={4}>4 - High visibility (full screen color)</option>
                    </select>
                    <p class="text-xs text-gray-500 mt-1">
                        Note: Rayhunter draws over the device's native UI, so some flickering is
                        expected
                    </p>
                </div>

                <div>
                    <label
                        for="key_input_mode"
                        class="block text-sm font-medium text-gray-700 mb-1"
                    >
                        Device Input Mode
                    </label>
                    <select
                        id="key_input_mode"
                        bind:value={config.key_input_mode}
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-rayhunter-blue"
                    >
                        <option value={0}>0 - Disable button control</option>
                        <option value={1}
                            >1 - Double-tap power button to start/stop recording</option
                        >
                    </select>
                </div>

                <div class="space-y-3">
                    <div class="flex items-center">
                        <input
                            id="colorblind_mode"
                            type="checkbox"
                            bind:checked={config.colorblind_mode}
                            class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded"
                        />
                        <label for="colorblind_mode" class="ml-2 block text-sm text-gray-700">
                            Colorblind Mode
                        </label>
                    </div>
                </div>

                <div class="border-t pt-4 mt-6 space-y-3">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">Notification Settings</h3>
                    <div>
                        <label for="ntfy_url" class="block text-sm font-medium text-gray-700 mb-1">
                            ntfy URL for Sending Notifications (if unset you will not receive
                            notifications)
                        </label>
                        <input
                            id="ntfy_url"
                            type="url"
                            bind:value={config.ntfy_url}
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-rayhunter-blue"
                        />
                        <p class="text-xs text-gray-500 mt-1">
                            Test button below uses the saved configuration URL, not the input above
                        </p>
                    </div>

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
                                class="mt-2 p-2 rounded text-sm {testMessageType === 'error'
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
                            />
                            <label
                                for="enable_lowbattery_notifications"
                                class="ml-2 block text-sm text-gray-700"
                            >
                                Low Battery
                            </label>
                        </div>
                    </div>
                </div>

                <div class="border-t pt-4 mt-6 space-y-3">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">Storage Management</h3>

                    <div>
                        <label
                            for="min_space_to_start_recording_mb"
                            class="block text-sm font-medium text-gray-700 mb-1"
                        >
                            Minimum Space to Start Recording (MB)
                        </label>
                        <input
                            id="min_space_to_start_recording_mb"
                            type="number"
                            min="1"
                            bind:value={config.min_space_to_start_recording_mb}
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-rayhunter-blue"
                        />
                        <p class="text-xs text-gray-500 mt-1">
                            Recording will not start if less than this amount of disk space is free
                        </p>
                    </div>

                    <div>
                        <label
                            for="min_space_to_continue_recording_mb"
                            class="block text-sm font-medium text-gray-700 mb-1"
                        >
                            Minimum Space to Continue Recording (MB)
                        </label>
                        <input
                            id="min_space_to_continue_recording_mb"
                            type="number"
                            min="1"
                            bind:value={config.min_space_to_continue_recording_mb}
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-rayhunter-blue"
                        />
                        <p class="text-xs text-gray-500 mt-1">
                            Recording will stop automatically if disk space drops below this level
                        </p>
                    </div>
                </div>

                {#if config.device === 'orbic' || config.device === 'moxee'}
                    <div class="border-t pt-4 mt-6 space-y-3">
                        <h3 class="text-lg font-semibold text-gray-800 mb-4">WiFi Client Mode</h3>
                        <p class="text-xs text-gray-500">
                            Connect the device to an existing WiFi network for internet access (e.g.
                            notifications, remote access). The hotspot AP stays running alongside
                            WiFi client mode.
                        </p>

                        <div class="flex items-center">
                            <input
                                id="wifi_enabled"
                                type="checkbox"
                                bind:checked={config.wifi_enabled}
                                class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded"
                            />
                            <label for="wifi_enabled" class="ml-2 block text-sm text-gray-700">
                                Enable WiFi
                            </label>
                        </div>
                        <p class="text-xs text-gray-500">
                            Unchecking stops WiFi without clearing saved credentials.
                        </p>

                        {#if wifiStatus && config.wifi_enabled}
                            {#if wifiStatus.state === 'connected'}
                                <p class="text-xs text-green-600">
                                    Connected to "{wifiStatus.ssid}" ({wifiStatus.ip})
                                </p>
                            {:else if wifiStatus.state === 'connecting'}
                                <p class="text-xs text-amber-600">Connecting...</p>
                            {:else if wifiStatus.state === 'failed'}
                                <p class="text-xs text-red-600">
                                    Failed: {wifiStatus.error}
                                </p>
                            {/if}
                        {/if}

                        <div>
                            <label
                                for="wifi_ssid"
                                class="block text-sm font-medium text-gray-700 mb-1"
                            >
                                WiFi Network Name (SSID)
                            </label>
                            <div class="flex gap-2">
                                <input
                                    id="wifi_ssid"
                                    type="text"
                                    bind:value={config.wifi_ssid}
                                    placeholder="MyWiFiNetwork"
                                    class="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-rayhunter-blue"
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
                        </div>

                        {#if scanResults.length > 0}
                            <div
                                class="border border-gray-200 rounded-md max-h-40 overflow-y-auto divide-y"
                            >
                                {#each scanResults as network}
                                    <button
                                        type="button"
                                        class="w-full px-3 py-2 text-left text-sm hover:bg-gray-50 flex justify-between"
                                        onclick={() => select_network(network.ssid)}
                                    >
                                        <span>{network.ssid}</span>
                                        <span class="text-gray-400"
                                            >{network.signal_dbm} dBm &middot; {network.security}</span
                                        >
                                    </button>
                                {/each}
                            </div>
                        {/if}

                        <div>
                            <label
                                for="wifi_password"
                                class="block text-sm font-medium text-gray-700 mb-1"
                            >
                                WiFi Password
                            </label>
                            <input
                                id="wifi_password"
                                type="password"
                                bind:value={config.wifi_password}
                                placeholder="Enter password"
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-rayhunter-blue"
                            />
                            <p class="text-xs text-gray-500 mt-1">
                                Changing the network requires re-entering the password.
                            </p>
                        </div>

                        {#if config.wifi_ssid}
                            <div>
                                <label
                                    for="dns_servers"
                                    class="block text-sm font-medium text-gray-700 mb-1"
                                >
                                    DNS Servers
                                </label>
                                <input
                                    id="dns_servers"
                                    type="text"
                                    bind:value={dnsServersInput}
                                    placeholder="8.8.8.8, 1.1.1.1"
                                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-rayhunter-blue"
                                />
                                <p class="text-xs text-gray-500 mt-1">
                                    Comma-separated. Used when WiFi is active. Defaults to 8.8.8.8,
                                    1.1.1.1.
                                </p>
                            </div>
                        {/if}
                    </div>
                {/if}

                <div class="border-t pt-4 mt-6 space-y-3">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">Device Security</h3>

                    <div class="flex items-center">
                        <input
                            id="block_ota_daemons"
                            type="checkbox"
                            bind:checked={config.block_ota_daemons}
                            class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded"
                        />
                        <label for="block_ota_daemons" class="ml-2 block text-sm text-gray-700">
                            Block OTA update daemons
                        </label>
                    </div>
                    <p class="text-xs text-gray-500">
                        Prevents Verizon's dmclient and upgrade services from running. They are
                        replaced with stubs at runtime. Disabling requires a reboot to take effect.
                    </p>

                    <div class="flex items-center">
                        <input
                            id="firewall_restrict_outbound"
                            type="checkbox"
                            bind:checked={config.firewall_restrict_outbound}
                            class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded"
                        />
                        <label
                            for="firewall_restrict_outbound"
                            class="ml-2 block text-sm text-gray-700"
                        >
                            Restrict outbound traffic
                        </label>
                    </div>
                    <p class="text-xs text-gray-500">
                        Only allows DNS, DHCP, and HTTPS (port 443) outbound. Blocks all other
                        outbound connections on every interface (WiFi and cellular). Loopback and
                        hotspot traffic are always allowed. Changes take effect immediately.
                    </p>

                    {#if config.firewall_restrict_outbound}
                        <div>
                            <label
                                for="firewall_allowed_ports"
                                class="block text-sm font-medium text-gray-700 mb-1"
                            >
                                Additional Allowed Ports
                            </label>
                            <input
                                id="firewall_allowed_ports"
                                type="text"
                                value={config.firewall_allowed_ports
                                    ? config.firewall_allowed_ports.join(', ')
                                    : ''}
                                oninput={(e) => {
                                    const val = (e.target as HTMLInputElement).value.trim();
                                    config!.firewall_allowed_ports =
                                        val.length > 0
                                            ? val
                                                  .split(',')
                                                  .map((s) => parseInt(s.trim()))
                                                  .filter((n) => !isNaN(n))
                                            : null;
                                }}
                                placeholder="22, 80"
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-rayhunter-blue"
                            />
                            <p class="text-xs text-gray-500 mt-1">
                                Comma-separated TCP ports, e.g. 22, 80
                            </p>
                        </div>
                    {/if}
                </div>

                <div class="border-t pt-4 mt-6">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">
                        Analyzer Heuristic Settings
                    </h3>
                    <div class="space-y-3">
                        <div class="flex items-center">
                            <input
                                id="imsi_requested"
                                type="checkbox"
                                bind:checked={config.analyzers.imsi_requested}
                                class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded"
                            />
                            <label for="imsi_requested" class="ml-2 block text-sm text-gray-700">
                                IMSI Requested Heuristic
                            </label>
                        </div>

                        <div class="flex items-center">
                            <input
                                id="connection_redirect_2g_downgrade"
                                type="checkbox"
                                bind:checked={config.analyzers.connection_redirect_2g_downgrade}
                                class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded"
                            />
                            <label
                                for="connection_redirect_2g_downgrade"
                                class="ml-2 block text-sm text-gray-700"
                            >
                                Connection Redirect 2G Downgrade Heuristic
                            </label>
                        </div>

                        <div class="flex items-center">
                            <input
                                id="lte_sib6_and_7_downgrade"
                                type="checkbox"
                                bind:checked={config.analyzers.lte_sib6_and_7_downgrade}
                                class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded"
                            />
                            <label
                                for="lte_sib6_and_7_downgrade"
                                class="ml-2 block text-sm text-gray-700"
                            >
                                LTE SIB6 and SIB7 Downgrade Heuristic
                            </label>
                        </div>

                        <div class="flex items-center">
                            <input
                                id="null_cipher"
                                type="checkbox"
                                bind:checked={config.analyzers.null_cipher}
                                class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded"
                            />
                            <label for="null_cipher" class="ml-2 block text-sm text-gray-700">
                                Null Cipher Heuristic
                            </label>
                        </div>

                        <div class="flex items-center">
                            <input
                                id="nas_null_cipher"
                                type="checkbox"
                                bind:checked={config.analyzers.nas_null_cipher}
                                class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded"
                            />
                            <label for="nas_null_cipher" class="ml-2 block text-sm text-gray-700">
                                NAS Null Cipher Heuristic
                            </label>
                        </div>

                        <div class="flex items-center">
                            <input
                                id="incomplete_sib"
                                type="checkbox"
                                bind:checked={config.analyzers.incomplete_sib}
                                class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded"
                            />
                            <label for="incomplete_sib" class="ml-2 block text-sm text-gray-700">
                                Incomplete SIB Heuristic
                            </label>
                        </div>

                        <div class="flex items-center">
                            <input
                                id="test_analyzer"
                                type="checkbox"
                                bind:checked={config.analyzers.test_analyzer}
                                class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded"
                            />
                            <label for="test_analyzer" class="ml-2 block text-sm text-gray-700">
                                Test Heuristic (noisy!)
                            </label>
                        </div>
                        <div class="flex items-center">
                            <input
                                id="diagnostic_analyzer"
                                type="checkbox"
                                bind:checked={config.analyzers.diagnostic_analyzer}
                                class="h-4 w-4 text-rayhunter-blue focus:ring-rayhunter-blue border-gray-300 rounded"
                            />
                            <label
                                for="diagnostic_analyzer"
                                class="ml-2 block text-sm text-gray-700"
                            >
                                Diagnostic Analyzer
                            </label>
                        </div>
                    </div>
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
                    class="mt-4 p-3 rounded {messageType === 'error'
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
    {/if}
</div>
