import { describe, expect, it } from 'vitest';
import { next_preference, parse_preference, resolve_theme } from './color_theme';

describe('parse_preference', () => {
    it('accepts explicit light and dark', () => {
        expect(parse_preference('light')).toBe('light');
        expect(parse_preference('dark')).toBe('dark');
    });

    it('treats missing or unrecognised values as system', () => {
        expect(parse_preference(null)).toBe('system');
        expect(parse_preference('')).toBe('system');
        expect(parse_preference('system')).toBe('system');
        expect(parse_preference('DARK')).toBe('system');
        expect(parse_preference('purple')).toBe('system');
    });
});

describe('resolve_theme', () => {
    it('follows the OS preference in system mode', () => {
        expect(resolve_theme('system', true)).toBe('dark');
        expect(resolve_theme('system', false)).toBe('light');
    });

    it('lets an explicit choice override the OS preference', () => {
        expect(resolve_theme('light', true)).toBe('light');
        expect(resolve_theme('dark', false)).toBe('dark');
    });
});

describe('next_preference', () => {
    it.each([true, false])(
        'cycles back to system after three clicks (system_prefers_dark=%s)',
        (system_prefers_dark) => {
            let pref = next_preference('system', system_prefers_dark);
            pref = next_preference(pref, system_prefers_dark);
            pref = next_preference(pref, system_prefers_dark);
            expect(pref).toBe('system');
        }
    );

    it('always changes the visible theme on the first click from system', () => {
        for (const system_prefers_dark of [true, false]) {
            const before = resolve_theme('system', system_prefers_dark);
            const after = resolve_theme(
                next_preference('system', system_prefers_dark),
                system_prefers_dark
            );
            expect(after).not.toBe(before);
        }
    });

    it('visits both explicit themes before returning to system', () => {
        expect(next_preference('system', false)).toBe('dark');
        expect(next_preference('dark', false)).toBe('light');
        expect(next_preference('light', false)).toBe('system');

        expect(next_preference('system', true)).toBe('light');
        expect(next_preference('light', true)).toBe('dark');
        expect(next_preference('dark', true)).toBe('system');
    });
});
