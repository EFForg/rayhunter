import { describe, it, expect } from 'vitest';
import { EventType, parse_finished_report, Severity } from './analysis.svelte';
import { type NewlineDeliminatedJson } from './ndjson';

const SAMPLE_REPORT_NDJSON: NewlineDeliminatedJson = [
    {
        analyzers: [
            {
                name: 'LTE SIB 6/7 Downgrade',
                description:
                    'Tests for LTE cells broadcasting a SIB type 6 and 7 which include 2G/3G frequencies with higher priorities.'
            },
            {
                name: 'IMSI Provided',
                description: "Tests whether the UE's IMSI was ever provided to the cell"
            },
            {
                name: 'Null Cipher',
                description: 'Tests whether the cell suggests using a null cipher (EEA0)'
            },
            {
                name: 'Example Analyzer',
                description:
                    'Always returns true, if you are seeing this you are either a developer or you are about to have problems.'
            }
        ]
    },
    {
        timestamp: '2024-10-08T13:25:43.011689003-07:00',
        skipped_message_reasons: [
            'DecodingError(UperDecodeError(Error { cause: BufferTooShort, msg: "PerCodec:DecodeError:Requested Bits to decode 3, Remaining bits 1", context: [] }))'
        ],
        analysis: []
    },
    {
        timestamp: '2024-10-08T13:25:43.480872496-07:00',
        skipped_message_reasons: [],
        analysis: [
            {
                timestamp: '2024-08-19T03:33:54.318Z',
                events: [
                    null,
                    null,
                    null,
                    {
                        event_type: { type: 'QualitativeWarning', severity: 'Low' },
                        message: 'TMSI was provided to cell'
                    }
                ]
            }
        ]
    }
];

describe('analysis report parsing', () => {
    it('parses the example analysis', () => {
        const report = parse_finished_report(SAMPLE_REPORT_NDJSON);
        expect(report.metadata.analyzers).toEqual([
            {
                name: 'LTE SIB 6/7 Downgrade',
                description:
                    'Tests for LTE cells broadcasting a SIB type 6 and 7 which include 2G/3G frequencies with higher priorities.'
            },
            {
                name: 'IMSI Provided',
                description: "Tests whether the UE's IMSI was ever provided to the cell"
            },
            {
                name: 'Null Cipher',
                description: 'Tests whether the cell suggests using a null cipher (EEA0)'
            },
            {
                name: 'Example Analyzer',
                description:
                    'Always returns true, if you are seeing this you are either a developer or you are about to have problems.'
            }
        ]);
        expect(report.rows).toHaveLength(2);
        expect(report.rows[0].skipped_message_reasons).toHaveLength(1);
        expect(report.rows[0].analysis).toHaveLength(0);
        expect(report.rows[1].skipped_message_reasons).toHaveLength(0);
        expect(report.rows[1].analysis).toHaveLength(1);
        expect(report.rows[1].analysis[0].events).toHaveLength(1);
        const event = report.rows[1].analysis[0].events[0];
        if (event.type === EventType.Warning) {
            expect(event.severity).toEqual(Severity.Low);
        } else {
            throw 'wrong event type';
        }
    });
});
