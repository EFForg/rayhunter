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
}

export class ManifestEntry {
    public name: string;
    public start_time: Date;
    public last_message_time: Date | undefined;
    public qmdl_size_bytes: number;
    public analysis_size_bytes: number;

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
