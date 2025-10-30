<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { listen } from '@tauri-apps/api/event';

    let buttonEnabled = $state(true);
    let installerArgs = $state('');
    let installerOutput = $state('');

    listen<string>('installer-output', (event) => {
        installerOutput += event.payload;
    });

    async function run_installer(event: Event) {
        event.preventDefault();
        buttonEnabled = false;
        installerOutput = '';
        try {
            await invoke('install_rayhunter', { args: installerArgs });
        } catch (error) {
            installerOutput +=
                'Rayhunter GUI installer encountered an internal error. Error was:\n';
            installerOutput += error;
        }
        buttonEnabled = true;
    }
</script>

<div class="p-4 xl:px-8 bg-rayhunter-blue drop-shadow flex flex-row justify-between items-center">
    <!-- https://www.w3.org/WAI/tutorials/images/decorative/ -->
    <img src="/rayhunter_text.png" alt="" class="h-10 xl:h-12" />
    <div class="flex flex-row gap-4">
        <a
            class="flex flex-row gap-1 group"
            href="https://github.com/EFForg/rayhunter/issues"
            target="_blank"
        >
            <span class="hidden text-white group-hover:text-gray-400 lg:flex">Report Issue</span>
            <svg
                class="w-6 h-6 text-white group-hover:text-gray-400"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="currentColor"
                viewBox="0 0 24 24"
            >
                <path
                    fill-rule="evenodd"
                    d="M12.006 2a9.847 9.847 0 0 0-6.484 2.44 10.32 10.32 0 0 0-3.393 6.17 10.48 10.48 0 0 0 1.317 6.955 10.045 10.045 0 0 0 5.4 4.418c.504.095.683-.223.683-.494 0-.245-.01-1.052-.014-1.908-2.78.62-3.366-1.21-3.366-1.21a2.711 2.711 0 0 0-1.11-1.5c-.907-.637.07-.621.07-.621.317.044.62.163.885.346.266.183.487.426.647.71.135.253.318.476.538.655a2.079 2.079 0 0 0 2.37.196c.045-.52.27-1.006.635-1.37-2.219-.259-4.554-1.138-4.554-5.07a4.022 4.022 0 0 1 1.031-2.75 3.77 3.77 0 0 1 .096-2.713s.839-.275 2.749 1.05a9.26 9.26 0 0 1 5.004 0c1.906-1.325 2.74-1.05 2.74-1.05.37.858.406 1.828.101 2.713a4.017 4.017 0 0 1 1.029 2.75c0 3.939-2.339 4.805-4.564 5.058a2.471 2.471 0 0 1 .679 1.897c0 1.372-.012 2.477-.012 2.814 0 .272.18.592.687.492a10.05 10.05 0 0 0 5.388-4.421 10.473 10.473 0 0 0 1.313-6.948 10.32 10.32 0 0 0-3.39-6.165A9.847 9.847 0 0 0 12.007 2Z"
                    clip-rule="evenodd"
                />
            </svg>
        </a>
        <a
            class="flex flex-row gap-1 group"
            href="https://efforg.github.io/rayhunter/"
            target="_blank"
        >
            <span class="hidden text-white group-hover:text-gray-400 lg:flex">Docs</span>
            <svg
                class="w-6 h-6 text-white group-hover:text-gray-400"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M5 19V4a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v13H7a2 2 0 0 0-2 2Zm0 0a2 2 0 0 0 2 2h12M9 3v14m7 0v4"
                />
            </svg>
        </a>
    </div>
</div>
<form class="flex justify-center pt-5" onsubmit={run_installer}>
    <input
        class="mr-1 px-5 py-2 rounded-lg shadow-md"
        placeholder="Enter CLI installer args..."
        bind:value={installerArgs}
    />
    <button
        class="{buttonEnabled ? 'cursor-pointer' : ''} px-5 py-2 rounded-lg shadow-md"
        disabled={!buttonEnabled}
        type="submit">Run</button
    >
</form>
<p class="p-4">Installer output:</p>
<p class="bg-gray-100 px-5 py-2 rounded-lg shadow-md whitespace-pre-line">
    {installerOutput}
</p>
