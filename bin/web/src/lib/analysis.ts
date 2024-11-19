import { parse_ndjson, type NewlineDeliminatedJson } from "./ndjson";
import { req } from "./utils";

export type AnalysisReport =
    | LoadingReport
    | FinishedReport;

export type LoadingReport = {};

export type FinishedReport = {
    metadata: ReportMetadata;
    rows: AnalysisRow[];
};

export type ReportMetadata = {
    analyzers: AnalyzerMetadata[];
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
    Warning,
}

export type QualitativeWarning = {
    type: EventType.Warning;
    severity: Severity;
    message: string;
};

export enum Severity {
    Low,
    Medium,
    High,
}

export type InformationalEvent = {
    type: EventType.Informational;
    message: string;
};

export function parse_finished_report(report_json: NewlineDeliminatedJson): FinishedReport {
    const metadata: ReportMetadata = report_json[0]; // this can be cast directly
    const rows: AnalysisRow[] = report_json.slice(1).map((row_json: any) => {
        const analysis: PacketAnalysis[] = row_json.analysis.map((analysis_json: any) => {
            const events: Event[] = analysis_json.events.map((event_json: any): Event | null => {
                    if (event_json === null) {
                        return null;
                    } else if (event_json.event_type === "Informational") {
                        return {
                            type: EventType.Informational,
                            message: event_json.message,
                        };
                    } else {
                        return {
                            type: EventType.Warning,
                            severity: event_json.severity === "High" ? Severity.High :
                                event_json.severity === "Medium" ? Severity.Medium : Severity.Low,
                            message: event_json.message,
                        };
                    }
                })
                .filter((maybe_event: Event | null) => maybe_event !== null);
            return {
                timestamp: analysis_json.timestamp,
                events,
            };
        });
        return {
            timestamp: new Date(row_json.timestamp),
            skipped_message_reasons: row_json.skipped_message_reasons,
            analysis,
        };
    });
    return {
        metadata,
        rows,
    };
}

export async function get_report(name: string): Promise<FinishedReport> {
    const report_json = parse_ndjson(await req('GET', `/api/analysis-report/${name}`));
    return parse_finished_report(report_json);
}
