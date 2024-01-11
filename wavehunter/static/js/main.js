async function populateDivs() {
    const systemStats = await getSystemStats();
    const diagStats = await getDiagStats();

    const systemStatsDiv = document.getElementById('system-stats');
    const diagStatsDiv = document.getElementById('diag-stats');

    systemStatsDiv.innerHTML = JSON.stringify(systemStats, null, 2);
    diagStatsDiv.innerHTML = JSON.stringify(diagStats, null, 2);
}

async function getSystemStats() {
    return await getJson('/api/system-stats');
}

async function getDiagStats() {
    return await getJson('/api/diag-stats');
}

async function getJson(url) {
    const response = await fetch(url);
    const data = await response.json();
    return data;
}
