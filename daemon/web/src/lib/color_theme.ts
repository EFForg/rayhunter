/**
 * Light/dark colour theme handling.
 *
 * The preference is stored per-browser in localStorage. "system" (the default,
 * and what an absent/unrecognised stored value means) follows the browser's
 * prefers-color-scheme setting, including live changes to it.
 *
 * The active theme is applied as `data-theme` on <html>, which the `dark:`
 * variant in app.css keys off. src/app.html contains a small inline copy of
 * this resolution logic that runs before first paint to avoid a flash of the
 * wrong theme; keep the two in sync.
 */

export type ThemePreference = 'system' | 'light' | 'dark';
export type Theme = 'light' | 'dark';

export const THEME_STORAGE_KEY = 'rayhunter-theme';
export const DARK_MEDIA_QUERY = '(prefers-color-scheme: dark)';

export function parse_preference(stored: string | null): ThemePreference {
    return stored === 'light' || stored === 'dark' ? stored : 'system';
}

export function resolve_theme(preference: ThemePreference, system_prefers_dark: boolean): Theme {
    if (preference === 'system') {
        return system_prefers_dark ? 'dark' : 'light';
    }
    return preference;
}

/**
 * The toggle cycles system -> opposite of system -> same as system -> system,
 * so the first click from the default always visibly changes the theme.
 */
export function next_preference(
    preference: ThemePreference,
    system_prefers_dark: boolean
): ThemePreference {
    const same: Theme = system_prefers_dark ? 'dark' : 'light';
    const opposite: Theme = system_prefers_dark ? 'light' : 'dark';
    if (preference === 'system') {
        return opposite;
    }
    if (preference === opposite) {
        return same;
    }
    return 'system';
}

// localStorage can throw (e.g. storage disabled), in which case we just
// behave as if nothing was ever saved.
export function load_preference(): ThemePreference {
    try {
        return parse_preference(localStorage.getItem(THEME_STORAGE_KEY));
    } catch {
        return 'system';
    }
}

export function save_preference(preference: ThemePreference): void {
    try {
        if (preference === 'system') {
            localStorage.removeItem(THEME_STORAGE_KEY);
        } else {
            localStorage.setItem(THEME_STORAGE_KEY, preference);
        }
    } catch {
        // not persisted; the choice still applies until the page is reloaded
    }
}

export function apply_theme(theme: Theme): void {
    document.documentElement.dataset.theme = theme;
}
