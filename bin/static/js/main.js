async function populateDivs() {
    const systemStats = await getSystemStats();
    const systemStatsDiv = document.getElementById('system-stats');
    systemStatsDiv.innerHTML = JSON.stringify(systemStats, null, 2);

    const qmdlManifest = await getQmdlManifest();
    updateQmdlManifestTable(qmdlManifest);
}

function updateQmdlManifestTable(manifest) {
    const table = document.getElementById('qmdl-manifest-table');
    const numRows = table.rows.length;
    for (let i=1; i<numRows; i++) {
        table.deleteRow(1);
    }
    if (manifest.current_entry) {
        const row = createEntryRow(manifest.current_entry);
        row.classList.add('current');
        table.appendChild(row)
    }
    for (let entry of manifest.entries) {
        table.appendChild(createEntryRow(entry));
    }
}

function createEntryRow(entry) {
    const row = document.createElement('tr');
    const name = document.createElement('th');
    name.scope = 'row';
    name.innerText = entry.name;
    row.appendChild(name);
    for (const key of ['start_time', 'last_message_time', 'size_bytes']) {
        const td = document.createElement('td');
        td.innerText = entry[key];
        row.appendChild(td);
    }
    const pcap_td = document.createElement('td');
    const pcap_link = document.createElement('a');
    pcap_link.href = `/api/pcap/${entry.name}`;
    pcap_link.innerText = 'pcap';
    pcap_td.appendChild(pcap_link);
    row.appendChild(pcap_td);
    const qmdl_td = document.createElement('td');
    const qmdl_link = document.createElement('a');
    qmdl_link.href = `/api/qmdl/${entry.name}`;
    qmdl_link.innerText = 'qmdl';
    qmdl_td.appendChild(qmdl_link);
    row.appendChild(qmdl_td);
    return row;
}

async function getSystemStats() {
    return JSON.parse(await req('GET', '/api/system-stats'));
}

async function getQmdlManifest() {
    const manifest = JSON.parse(await req('GET', '/api/qmdl-manifest'));
    if (manifest.current_entry) {
        manifest.current_entry.start_time = new Date(manifest.current_entry.start_time);
        if (manifest.current_entry.last_message_time === undefined) {
            manifest.current_entry.last_message_time = "N/A";
        } else {
            manifest.current_entry.last_message_time = new Date(manifest.current_entry.last_message_time);
        }
    }
    for (entry of manifest.entries) {
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
