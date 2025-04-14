import { get_report, type AnalysisReport } from "./analysis.svelte";
import { AnalysisStatus, type AnalysisManager } from "./analysisManager.svelte";

interface JsonManifest {
    entries: JsonManifestEntry[];
    current_entry: JsonManifestEntry | null;
}

interface JsonManifestEntry {
    name: string;
    start_time: string;
    last_message_time: string;
    qmdl_size_bytes: number;
    analysis_size_bytes: number;
}

export class Manifest {
    public entries: ManifestEntry[] = [];
    public current_entry: ManifestEntry | undefined;

    constructor(json: JsonManifest) {
        for (let entry of json.entries) {
            this.entries.push(new ManifestEntry(entry));
        }
        if (json.current_entry !== null) {
            this.current_entry = new ManifestEntry(json['current_entry']);
        }

        // sort entries in reverse chronological order
        this.entries.reverse();
    }

    async set_analysis_status(manager: AnalysisManager) {
        for (let entry of this.entries) {
            entry.analysis_status = manager.status.get(entry.name);
            entry.analysis_report = manager.reports.get(entry.name);
        }

        if (this.current_entry) {
            try {
                this.current_entry.analysis_report = await get_report(this.current_entry.name);
            } catch(err) {
                this.current_entry.analysis_report = `Err: failed to get analysis report: ${err}`;
            }

            // the current entry should always be considered "finished", as its
            // analysis report is always available
            this.current_entry.analysis_status = AnalysisStatus.Finished;
        }
	}
}

export class ManifestEntry {
    public name = $state("");
    public start_time: Date;
    public last_message_time: Date | undefined = $state(undefined);
    public qmdl_size_bytes = $state(0);
    public analysis_size_bytes = $state(0);
    public analysis_status: AnalysisStatus | undefined = $state(undefined);
    public analysis_report: AnalysisReport | string | undefined = $state(undefined);

    constructor(json: JsonManifestEntry) {
        this.name = json.name;
        this.qmdl_size_bytes = json.qmdl_size_bytes;
        this.analysis_size_bytes = json.analysis_size_bytes;
        this.start_time = new Date(json.start_time);
        if (json.last_message_time !== undefined) {
            this.last_message_time = new Date(json.last_message_time);
        }
    }

    getPcapUrl(): string {
        return `/api/pcap/${this.name}`;
    }

    getQmdlUrl(): string {
        return `/api/qmdl/${this.name}`;
    }

    getAnalysisReportUrl(): string {
        return `/api/analysis-report/${this.name}`;
    }
}
