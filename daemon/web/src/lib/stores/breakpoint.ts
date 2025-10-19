// stores/breakpoint.ts
import { readable, type Readable } from 'svelte/store';
import { breakpoints } from '../../theme';

type Breakpoint = keyof typeof breakpoints;

// Store that tracks if a specific breakpoint matches
export function createBreakpointStore(breakpoint: Breakpoint): Readable<boolean> {
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
export const screenIsSmUp: Readable<boolean> = createBreakpointStore('sm');
export const screenIsMdUp: Readable<boolean> = createBreakpointStore('md');
export const screenIsLgUp: Readable<boolean> = createBreakpointStore('lg');
export const screenIsXlUp: Readable<boolean> = createBreakpointStore('xl');
