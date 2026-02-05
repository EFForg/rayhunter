<script lang="ts">
    import { get_config, set_config, test_notification, type Config } from '../utils.svelte';

    let config = $state<Config | null>(null);

    let loading = $state(false);
    let saving = $state(false);
    let testingNotification = $state(false);
    let message = $state('');
    let messageType = $state<'success' | 'error' | null>(null);
    let testMessage = $state('');
    let testMessageType = $state<'success' | 'error' | null>(null);
    let showConfig = $state(false);

    async function load_config() {
        try {
            loading = true;
            config = await get_config();
            message = '';
            messageType = null;
        } catch (error) {
            message = `Failed to load config: ${error}`;
            messageType = 'error';
        } finally {
            loading = false;
        }
    }

    async function save_config() {
        if (!config) return;

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
