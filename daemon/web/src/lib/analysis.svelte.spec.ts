import { describe, it, expect } from 'vitest';
import { AnalysisRowType, parse_finished_report } from './analysis.svelte';
import { type NewlineDeliminatedJson } from './ndjson';

const SAMPLE_V2_REPORT_NDJSON: NewlineDeliminatedJson = [
    {
        analyzers: [
            {
                name: 'Analyzer 1',
                description: 'A first analyzer',
                version: 2,
            },
            {
                name: 'Analyzer 2',
                description: 'A second analyzer',
                version: 2,
            },
        ],
        report_version: 2,
    },
    {
        skipped_message_reason: 'The reason why the message was skipped',
    },
    {
        packet_timestamp: '2024-08-19T03:33:54.318Z',
        events: [
            null,
            {
                event_type: 'Low',
                message: 'Something nasty happened',
            },
        ],
    },
];

describe('analysis report parsing', () => {
    it('parses v2 example analysis', () => {
        const report = parse_finished_report(SAMPLE_V2_REPORT_NDJSON);
        expect(report.metadata.report_version).toEqual(2);
        expect(report.metadata.analyzers).toEqual([
            {
                name: 'Analyzer 1',
                description: 'A first analyzer',
                version: 2,
            },
            {
                name: 'Analyzer 2',
                description: 'A second analyzer',
                version: 2,
            },
        ]);
        expect(report.rows).toHaveLength(2);
        expect(report.rows[0].type).toBe(AnalysisRowType.Skipped);
        if (report.rows[1].type === AnalysisRowType.Analysis) {
            const row = report.rows[1];
            expect(row.events).toHaveLength(2);
            expect(row.events[0]).toBeNull();
            const event = row.events[1];
            const expected_timestamp = new Date('2024-08-19T03:33:54.318Z');
            expect(row.packet_timestamp.getTime()).toEqual(expected_timestamp.getTime());
            expect(event!.event_type).toEqual('Low');
        } else {
            throw 'wrong row type';
        }
    });
});
