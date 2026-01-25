// stores/breakpoint.ts
import { readable, type Readable } from 'svelte/store';
import { breakpoints } from '../../theme';

type Breakpoint = keyof typeof breakpoints;

// Store that tracks if a specific breakpoint matches
export function create_breakpoint_store(breakpoint: Breakpoint): Readable<boolean> {
    return readable<boolean>(false, (set) => {
        const width = breakpoints[breakpoint];
        const mediaQuery = window.matchMedia(`(min-width: ${width})`);

        // Set initial value
        set(mediaQuery.matches);

        // Update on change
        const handler = (e: MediaQueryListEvent) => set(e.matches);
        mediaQuery.addEventListener('change', handler);

        // Cleanup
        return () => mediaQuery.removeEventListener('change', handler);
    });
}

// Create stores for each breakpoint
export const screenIsSmUp: Readable<boolean> = create_breakpoint_store('sm');
export const screenIsMdUp: Readable<boolean> = create_breakpoint_store('md');
export const screenIsLgUp: Readable<boolean> = create_breakpoint_store('lg');
export const screenIsXlUp: Readable<boolean> = create_breakpoint_store('xl');
