import { get_report, type AnalysisReport } from './analysis.svelte';
import { AnalysisStatus, type AnalysisManager } from './analysisManager.svelte';

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
        for (const entry of json.entries) {
            this.entries.push(new ManifestEntry(entry));
        }
        if (json.current_entry !== null) {
            this.current_entry = new ManifestEntry(json['current_entry']);
        }

        // sort entries in reverse chronological order
        this.entries.reverse();
    }

    async set_analysis_status(manager: AnalysisManager) {
        for (const entry of this.entries) {
            entry.analysis_status = manager.status.get(entry.name);
            entry.analysis_report = manager.reports.get(entry.name);
        }

        if (this.current_entry) {
            try {
                this.current_entry.analysis_report = await get_report(this.current_entry.name);
            } catch (err) {
                this.current_entry.analysis_report = `Err: failed to get analysis report: ${err}`;
            }

            // the current entry should always be considered "finished", as its
            // analysis report is always available
            this.current_entry.analysis_status = AnalysisStatus.Finished;
        }
    }
}

export class ManifestEntry {
    public name = $state('');
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
        if (json.last_message_time) {
            this.last_message_time = new Date(json.last_message_time);
        }
    }

    get_readable_qmdl_size(): string {
        if (this.qmdl_size_bytes === 0) return '0 Bytes';
        const k = 1024;
        const dm = 2;
        const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
        const i = Math.floor(Math.log(this.qmdl_size_bytes) / Math.log(k));
        return `${Number.parseFloat((this.qmdl_size_bytes / k ** i).toFixed(dm))} ${sizes[i]}`;
    }

    get_num_warnings(): number | undefined {
        if (this.analysis_report === undefined || typeof this.analysis_report === 'string') {
            return undefined;
        }
        return this.analysis_report.statistics.num_warnings;
    }

    get_pcap_url(): string {
        return `/api/pcap/${this.name}.pcapng`;
    }

    get_qmdl_url(): string {
        return `/api/qmdl/${this.name}.qmdl`;
    }

    get_zip_url(): string {
        return `/api/zip/${this.name}.zip`;
    }

    get_analysis_report_url(): string {
        return `/api/analysis-report/${this.name}`;
    }

    get_delete_url(): string {
        return `/api/delete-recording/${this.name}`;
    }
}
