import { get_report, type AnalysisReport } from './analysis.svelte';
import { req } from './utils.svelte';

export enum AnalysisStatus {
    // rayhunter is currently analyzing this entry (note that this is distinct
    // from the currently-recording entry)
    Running,
    // this entry is queued to be analyzed
    Queued,
    // analysis is finished, and the new report can be accessed
    Finished
}

type AnalysisStatusJson = {
    running: string | null;
    queued: string[];
    finished: string[];
};

export type AnalysisResult = {
    name: string;
    status: AnalysisStatus;
};

export class AnalysisManager {
    public status: Map<string, AnalysisStatus> = new Map();
    public reports: Map<string, AnalysisReport | string> = new Map();

    public async run_analysis(name: string) {
        await req('POST', `/api/analysis/${name}`);
        this.status.set(name, AnalysisStatus.Queued);
        this.reports.delete(name);
    }

    public async update() {
        const status: AnalysisStatusJson = JSON.parse(await req('GET', '/api/analysis'));
        if (status.running) {
            this.status.set(status.running, AnalysisStatus.Running);
        }

        for (const entry of status.queued) {
            this.status.set(entry, AnalysisStatus.Queued);
        }

        for (const entry of status.finished) {
            // if entry was already finished, nothing to do
            if (this.status.get(entry) === AnalysisStatus.Finished) {
                continue;
            }

            this.status.set(entry, AnalysisStatus.Finished);

            // fetch the analysis report
            this.reports.delete(entry);
            get_report(entry)
                .then((report) => {
                    this.reports.set(entry, report);
                })
                .catch((err) => {
                    this.reports.set(entry, `Failed to get analysis: ${err}`);
                });
        }
    }
}
