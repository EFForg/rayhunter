import { parse_ndjson, type NewlineDeliminatedJson } from './ndjson';
import { req } from './utils.svelte';

export type AnalysisReport = {
    metadata: ReportMetadata;
    rows: AnalysisRow[];
    statistics: ReportStatistics;
};

export type ReportStatistics = {
    num_warnings: number;
    num_informational_logs: number;
    num_skipped_packets: number;
};

export type ReportMetadata = {
    analyzers: AnalyzerMetadata[];
    rayhunter: RayhunterMetadata;
};

export type RayhunterMetadata = {
    rayhunter_version: string;
    system_os: string;
    arch: string;
};

export type AnalyzerMetadata = {
    name: string;
    description: string;
};

export type AnalysisRow = {
    timestamp: Date;
    skipped_message_reasons: string[];
    analysis: PacketAnalysis[];
};

export type PacketAnalysis = {
    timestamp: Date;
    events: Event[];
};
export type Event = QualitativeWarning | InformationalEvent;
export enum EventType {
    Informational,
    Warning
}

export type QualitativeWarning = {
    type: EventType.Warning;
    severity: Severity;
    message: string;
};

export enum Severity {
    Low,
    Medium,
    High
}

export type InformationalEvent = {
    type: EventType.Informational;
    message: string;
};

export function parse_finished_report(report_json: NewlineDeliminatedJson): AnalysisReport {
    const metadata: ReportMetadata = report_json[0]; // this can be cast directly
    let num_warnings = 0;
    let num_informational_logs = 0;
    let num_skipped_packets = 0;
    const rows: AnalysisRow[] = report_json.slice(1).map((row_json: any) => {
        const analysis: PacketAnalysis[] = row_json.analysis.map((analysis_json: any) => {
            const events: Event[] = analysis_json.events
                .map((event_json: any): Event | null => {
                    if (event_json === null) {
                        return null;
                    } else if (event_json.event_type.type === 'Informational') {
                        num_informational_logs += 1;
                        return {
                            type: EventType.Informational,
                            message: event_json.message
                        };
                    } else {
                        num_warnings += 1;
                        return {
                            type: EventType.Warning,
                            severity:
                                event_json.event_type.severity === 'High'
                                    ? Severity.High
                                    : event_json.event_type.severity === 'Medium'
                                      ? Severity.Medium
                                      : Severity.Low,
                            message: event_json.message
                        };
                    }
                })
                .filter((maybe_event: Event | null) => maybe_event !== null);
            return {
                timestamp: analysis_json.timestamp,
                events
            };
        });
        num_skipped_packets += row_json.skipped_message_reasons.length;
        return {
            timestamp: new Date(row_json.timestamp),
            skipped_message_reasons: row_json.skipped_message_reasons,
            analysis
        };
    });
    return {
        statistics: {
            num_informational_logs,
            num_warnings,
            num_skipped_packets
        },
        metadata,
        rows
    };
}

export async function get_report(name: string): Promise<AnalysisReport> {
    const report_json = parse_ndjson(await req('GET', `/api/analysis-report/${name}`));
    return parse_finished_report(report_json);
}
