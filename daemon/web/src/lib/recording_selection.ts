/** Selection helpers for bulk actions on the recording list. */

export interface SelectableRecording {
    name: string;
    qmdl_size_bytes: number;
    // undefined until analysis has finished and its report has loaded
    get_num_warnings(): number | undefined;
}

export type QuickSelect = 'all' | 'none' | 'with_warnings' | 'no_warnings' | 'empty';

/**
 * Names matching a quick-select preset. Recordings whose analysis hasn't
 * finished are deliberately excluded from both warning-based presets, so
 * e.g. "no warnings" can never sweep up a recording that hasn't been checked.
 */
export function quick_select(entries: SelectableRecording[], preset: QuickSelect): string[] {
    const matches = {
        all: () => true,
        none: () => false,
        with_warnings: (e: SelectableRecording) => (e.get_num_warnings() ?? 0) > 0,
        no_warnings: (e: SelectableRecording) => e.get_num_warnings() === 0,
        empty: (e: SelectableRecording) => e.qmdl_size_bytes === 0,
    }[preset];
    return entries.filter(matches).map((e) => e.name);
}

/** Selected recordings that the multi-recording ZIP will actually include. */
export function downloadable(
    entries: SelectableRecording[],
    selected: ReadonlySet<string>
): SelectableRecording[] {
    return entries.filter((e) => selected.has(e.name) && e.qmdl_size_bytes > 0);
}

export function zip_recordings_url(names: string[]): string {
    return `/api/zip-recordings?names=${encodeURIComponent(names.join(','))}`;
}

export interface DeleteFailure {
    name: string;
    error: string;
}

/**
 * Deletes recordings one at a time (the daemon handles deletes one by one
 * anyway, and this keeps the device load flat), carrying on past failures.
 */
export async function delete_sequentially(
    names: string[],
    delete_one: (name: string) => Promise<unknown>,
    on_progress?: (done: number, total: number) => void
): Promise<{ deleted: string[]; failed: DeleteFailure[] }> {
    const deleted: string[] = [];
    const failed: DeleteFailure[] = [];
    for (const [i, name] of names.entries()) {
        on_progress?.(i, names.length);
        try {
            await delete_one(name);
            deleted.push(name);
        } catch (e) {
            failed.push({ name, error: e instanceof Error ? e.message : String(e) });
        }
    }
    on_progress?.(names.length, names.length);
    return { deleted, failed };
}
