<script lang="ts">
    import NavItem from './NavItem.svelte';
    import {
        apply_theme,
        DARK_MEDIA_QUERY,
        load_preference,
        next_preference,
        resolve_theme,
        save_preference,
        type ThemePreference,
    } from '$lib/color_theme';

    // Defaults match the prerendered HTML; the real values are read on mount.
    // The page itself is already themed by the inline script in app.html.
    let mounted = $state(false);
    let preference: ThemePreference = $state('system');
    let system_prefers_dark = $state(false);

    $effect(() => {
        const media = window.matchMedia(DARK_MEDIA_QUERY);
        preference = load_preference();
        system_prefers_dark = media.matches;
        mounted = true;

        const handler = (e: MediaQueryListEvent) => {
            system_prefers_dark = e.matches;
        };
        media.addEventListener('change', handler);
        return () => media.removeEventListener('change', handler);
    });

    // Also re-runs when the OS preference changes, so "system" follows it live.
    $effect(() => {
        if (mounted) {
            apply_theme(resolve_theme(preference, system_prefers_dark));
        }
    });

    const preference_names: Record<ThemePreference, string> = {
        system: 'system',
        light: 'light',
        dark: 'dark',
    };
    let next = $derived(next_preference(preference, system_prefers_dark));
    let title = $derived(
        `Theme: ${preference_names[preference]} (switch to ${preference_names[next]})`
    );

    function cycle() {
        preference = next;
        save_preference(preference);
    }
</script>

<NavItem label="Theme" {title} onclick={cycle}>
    {#snippet icon()}
        <svg
            class="w-6 h-6 text-white group-hover:text-gray-400"
            aria-hidden="true"
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            {#if preference === 'light'}
                <!-- sun -->
                <circle cx="12" cy="12" r="4" />
                <path
                    d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41"
                />
            {:else if preference === 'dark'}
                <!-- moon -->
                <path d="M20 14.5A8 8 0 0 1 9.5 4a8 8 0 1 0 10.5 10.5Z" />
            {:else}
                <!-- monitor: follow the system setting -->
                <rect x="3" y="4" width="18" height="12" rx="1.5" />
                <path d="M8 20h8M12 16v4" />
            {/if}
        </svg>
    {/snippet}
</NavItem>
