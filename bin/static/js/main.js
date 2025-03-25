const STATUS_RUNNING = 'running';
const STATUS_QUEUED = 'queued';
const STATUS_NEEDS_UPDATE = 'needs-update';
const STATUS_COMPLETE = 'complete';

async function populateDivs() {
    const systemStats = await getSystemStats();
    const systemStatsDiv = document.getElementById('system-stats');
    systemStatsDiv.innerHTML = JSON.stringify(systemStats, null, 2);

    const analysisReportDiv = document.getElementById('analysis-report');
    try {
        const analysisReport = await getAnalysisReport('live');
        analysisReportDiv.innerHTML = JSON.stringify(analysisReport, null, 2);
    } catch (e) {
        analysisReportDiv.innerHTML = e.toString();
    }

    const qmdlManifest = await getQmdlManifest();
    await updateAnalysisStatus(qmdlManifest);
    await updateAnalysisResults(qmdlManifest);
    updateQmdlManifestTable(qmdlManifest);
}

function setStatus(qmdlManifest, name, status) {
    // ignore qmdlManifest.current_entry, it's always running
    for (const entry of qmdlManifest.entries) {
        if (entry.name === name) {
            entry['status'] = status;
            return;
        }
    }
}

async function updateAnalysisStatus(qmdlManifest) {
    const status = JSON.parse(await req('GET', '/api/analysis'));
    if (status.running) {
        setStatus(qmdlManifest, status.running, STATUS_RUNNING);
    }
    for (const queued in status.queued) {
        setStatus(qmdlManifest, queued, STATUS_QUEUED);
    }
}

function parseNewlineDelimitedJSON(inputStr) {
    const lines = inputStr.split('\n');
    const result = [];
    let currentLine = '';
    while (lines.length > 0) {
        currentLine += lines.shift();
        try {
            const entry = JSON.parse(currentLine);
            result.push(entry);
            currentLine = '';
        // if this chunk wasn't valid JSON, there was an escaped newline in the
        // JSON line, so simply continue to the next one
        } catch (e) {}
    }
    return result;
}

async function updateEntryAnalysisResult(entry) {
    entry.analysis = {
        warnings: [],
    };
    const report = parseNewlineDelimitedJSON(await req('GET', `/api/analysis-report/${entry.name}`));
    for (const row of report) {
      if (row["analysis"]) {
        const timestamp = new Date(row["timestamp"]);
        const analysis = row["analysis"];
        for (const warning of analysis) {
          entry.analysis.warnings.push({
            timestamp,
            warning,
          })
        }
      }
    }
    if (entry.analysis.warnings.length === 0) {
        entry.analysis_result = `0 warnings!`;
    } else {
        entry.analysis_result = `!!! ${entry.analysis.warnings.length} warnings !!!`;
        for (const warning of entry.analysis.warnings) {
            for (const event of warning.warning.events) {
                if (event === null) continue;
                msg = `${warning.timestamp}: ${event.message}`
                entry.analysis_result += `<br>${msg}`
            }
        }
    }
}

async function updateAnalysisResults(qmdlManifest) {
    if (qmdlManifest.current_entry) {
        await updateEntryAnalysisResult(qmdlManifest.current_entry);
    }
    for (const entry of qmdlManifest.entries) {
        if (entry.status === STATUS_NEEDS_UPDATE) {
            await updateEntryAnalysisResult(entry);
            entry.status = STATUS_COMPLETE;
        }
    }
}

function updateQmdlManifestTable(manifest) {
    const table = document.getElementById('qmdl-manifest-table');
    const numRows = table.rows.length;
    for (let i=1; i<numRows; i++) {
        table.deleteRow(1);
    }
    if (manifest.current_entry) {
        const row = createEntryRow(manifest.current_entry, true);
        row.classList.add('current');
        table.appendChild(row)
    }
    for (let entry of manifest.entries) {
        table.appendChild(createEntryRow(entry), false);
    }
}

function createLink(uri, text) {
    const link = document.createElement('a');
    link.href = uri;
    link.innerText = text;
    return link;
}

function createEntryRow(entry, isCurrent) {
    const row = document.createElement('tr');
    const name = document.createElement('th');
    name.scope = 'row';
    name.innerText = entry.name;
    row.appendChild(name);

    for (const key of ['start_time', 'last_message_time', 'qmdl_size_bytes']) {
        const td = document.createElement('td');
        td.innerText = entry[key];
        row.appendChild(td);
    }

    const pcapTd = document.createElement('td');
    pcapTd.appendChild(createLink(`/api/pcap/${entry.name}`, 'pcap'));
    row.appendChild(pcapTd);

    const qmdlTd = document.createElement('td');
    qmdlTd.appendChild(createLink(`/api/qmdl/${entry.name}.qmdl`, 'qmdl'));
    row.appendChild(qmdlTd);

    const analysisResult = document.createElement('td');
    analysisResult.innerHTML = entry.analysis_result;
    if (entry.analysis.warnings.length > 0) {
        row.classList.add("warning");
    }
    row.appendChild(analysisResult);

    return row;
}

async function getAnalysisReport(name) {
    const rows = await req('GET', `/api/analysis-report/${name}`);
    return rows.split('\n')
        .filter(row => row.length > 0)
        .map(row => JSON.parse(row));
}

async function getSystemStats() {
    return JSON.parse(await req('GET', '/api/system-stats'));
}

async function getQmdlManifest() {
    const manifest = JSON.parse(await req('GET', '/api/qmdl-manifest'));
    if (manifest.current_entry) {
        manifest.current_entry.status = STATUS_NEEDS_UPDATE;
        manifest.current_entry.analysis_result = 'Waiting...';
        manifest.current_entry.start_time = new Date(manifest.current_entry.start_time);
        if (manifest.current_entry.last_message_time === undefined) {
            manifest.current_entry.last_message_time = "N/A";
        } else {
            manifest.current_entry.last_message_time = new Date(manifest.current_entry.last_message_time);
        }
    }
    for (entry of manifest.entries) {
        entry.status = STATUS_NEEDS_UPDATE;
        entry.analysis_result = 'Waiting...';
        entry.start_time = new Date(entry.start_time);
        entry.last_message_time = new Date(entry.last_message_time);
    }
    // sort them in reverse chronological order
    manifest.entries.reverse();
    return manifest;
}

async function startRecording() {
    await req('POST', '/api/start-recording');
    populateDivs();
}

async function stopRecording() {
    await req('POST', '/api/stop-recording');
    populateDivs();
}

async function req(method, url) {
    const response = await fetch(url, {
        method: method,
    });
    const body = await response.text();
    if (response.status >= 200 && response.status < 300) {
        return body;
    } else {
        throw new Error(body);
    }
}
