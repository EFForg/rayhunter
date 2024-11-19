import type { Manifest, ManifestEntry } from "./manifest";
import { req } from "./utils";

export enum AnalysisStatus {
    Running,
    Queued,
    Finished,
}

type AnalysisStatusJson = {
    running: string | null;
    queued: string[];
    finished: string[];
};

export type AnalysisResult {
    name: string,
    status: AnalysisStatus,
}

export class AnalysisManager {
    public analysis_status: Map<string, AnalysisStatus> = new Map();

    public async run_analysis(name: string) {
        await req('POST', `/api/analysis/${name}`);
        this.analysis_status.set(name, AnalysisStatus.Queued);
    }

    public async update() {
        this.analysis_status.clear();

        const status: AnalysisStatusJson = JSON.parse(await req('GET', '/api/analysis'));
        if (status.running) {
            this.analysis_status.set(status.running, AnalysisStatus.Running);
        }

        for (const entry of status.queued) {
            this.analysis_status.set(entry, AnalysisStatus.Queued);
        }

        for (const entry of status.finished) {
            this.analysis_status.set(entry, AnalysisStatus.Finished);
        }
    }
}
