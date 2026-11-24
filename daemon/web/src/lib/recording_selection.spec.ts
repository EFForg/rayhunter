import { describe, expect, it } from 'vitest';
import {
    delete_sequentially,
    downloadable,
    quick_select,
    zip_recordings_url,
    type SelectableRecording,
} from './recording_selection';

function rec(name: string, warnings: number | undefined, size = 100): SelectableRecording {
    return { name, qmdl_size_bytes: size, get_num_warnings: () => warnings };
}

const entries = [
    rec('warn', 2),
    rec('clean', 0),
    rec('unanalysed', undefined),
    rec('empty', 0, 0),
    rec('empty-unanalysed', undefined, 0),
];

describe('quick_select', () => {
    it('selects everything or nothing', () => {
        expect(quick_select(entries, 'all')).toEqual(entries.map((e) => e.name));
        expect(quick_select(entries, 'none')).toEqual([]);
    });

    it('selects recordings with warnings', () => {
        expect(quick_select(entries, 'with_warnings')).toEqual(['warn']);
    });

    it('never treats unanalysed recordings as having no warnings', () => {
        expect(quick_select(entries, 'no_warnings')).toEqual(['clean', 'empty']);
    });

    it('selects empty recordings regardless of analysis state', () => {
        expect(quick_select(entries, 'empty')).toEqual(['empty', 'empty-unanalysed']);
    });
});

describe('downloadable', () => {
    it('only includes selected, non-empty recordings', () => {
        const selected = new Set(['warn', 'empty', 'unanalysed']);
        expect(downloadable(entries, selected).map((e) => e.name)).toEqual(['warn', 'unanalysed']);
    });
});

describe('zip_recordings_url', () => {
    it('builds a comma-separated names query', () => {
        expect(zip_recordings_url(['1', '2'])).toBe('/api/zip-recordings?names=1%2C2');
    });
});

describe('delete_sequentially', () => {
    it('deletes one at a time and carries on past failures', async () => {
        let in_flight = 0;
        let max_in_flight = 0;
        const progress: number[] = [];
        const result = await delete_sequentially(
            ['a', 'b', 'c'],
            async (name) => {
                in_flight++;
                max_in_flight = Math.max(max_in_flight, in_flight);
                await new Promise((resolve) => setTimeout(resolve, 1));
                in_flight--;
                if (name === 'b') throw new Error('nope');
            },
            (done) => progress.push(done)
        );
        expect(max_in_flight).toBe(1);
        expect(result.deleted).toEqual(['a', 'c']);
        expect(result.failed).toEqual([{ name: 'b', error: 'nope' }]);
        expect(progress).toEqual([0, 1, 2, 3]);
    });
});
