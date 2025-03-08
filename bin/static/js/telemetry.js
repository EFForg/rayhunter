// Description: This file contains the client-side JavaScript code for the telemetry settings UI.
async function getTelemetryStatus() {
    try {
        const response = await fetch('/api/telemetry');
        if (!response.ok) {
            throw new Error(`Failed to get telemetry status: ${response.status}`);
        }
        return await response.json();
    } catch (error) {
        console.error('Error fetching telemetry status:', error);
        return null;
    }
}

async function updateTelemetrySettings(settings) {
    try {
        const response = await fetch('/api/telemetry', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(settings)
        });
        
        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`Failed to update telemetry settings: ${errorText}`);
        }
        
        return await response.json();
    } catch (error) {
        console.error('Error updating telemetry settings:', error);
        throw error;
    }
}

function createTelemetryUI(container) {
    // Create telemetry settings section
    const telemetrySection = document.createElement('div');
    telemetrySection.classList.add('telemetry-section');
    telemetrySection.innerHTML = `
        <h3>Telemetry Settings</h3>
        <div class="telemetry-status">Loading telemetry status...</div>
        <div class="telemetry-controls" style="display: none;">
            <div class="setting">
                <label for="telemetry-enabled">Enable Telemetry:</label>
                <input type="checkbox" id="telemetry-enabled">
            </div>
            <div class="setting">
                <label for="telemetry-endpoint">Endpoint URL:</label>
                <input type="text" id="telemetry-endpoint" placeholder="https://your-telemetry-server.org/api">
            </div>
            <div class="setting">
                <label for="telemetry-api-key">API Key:</label>
                <input type="password" id="telemetry-api-key" placeholder="Enter API key">
            </div>
            <div class="setting">
                <label for="telemetry-interval">Send Interval (seconds):</label>
                <input type="number" id="telemetry-interval" min="60" step="60">
            </div>
            <div class="setting">
                <label for="telemetry-include-warnings">Include Warnings:</label>
                <input type="checkbox" id="telemetry-include-warnings">
            </div>
            <div class="setting">
                <label for="telemetry-include-stats">Include System Stats:</label>
                <input type="checkbox" id="telemetry-include-stats">
            </div>
            <div class="device-id-info">
                <span>Device ID: </span><span id="telemetry-device-id"></span>
            </div>
            <button id="save-telemetry-settings">Save Settings</button>
        </div>
    `;
    
    container.appendChild(telemetrySection);
    
    // Fetch and display current telemetry status
    refreshTelemetryStatus(telemetrySection);
    
    // Set up event listeners
    const saveButton = telemetrySection.querySelector('#save-telemetry-settings');
    saveButton.addEventListener('click', async () => {
        const settings = {
            enabled: telemetrySection.querySelector('#telemetry-enabled').checked,
            endpoint: telemetrySection.querySelector('#telemetry-endpoint').value,
            api_key: telemetrySection.querySelector('#telemetry-api-key').value,
            send_interval_secs: parseInt(telemetrySection.querySelector('#telemetry-interval').value, 10),
            include_warnings: telemetrySection.querySelector('#telemetry-include-warnings').checked,
            include_stats: telemetrySection.querySelector('#telemetry-include-stats').checked
        };
        
        try {
            await updateTelemetrySettings(settings);
            alert('Telemetry settings updated successfully!');
            refreshTelemetryStatus(telemetrySection);
        } catch (error) {
            alert(`Failed to update telemetry settings: ${error.message}`);
        }
    });
}

async function refreshTelemetryStatus(container) {
    const statusElement = container.querySelector('.telemetry-status');
    const controlsElement = container.querySelector('.telemetry-controls');
    
    try {
        const status = await getTelemetryStatus();
        if (!status) {
            statusElement.textContent = 'Error fetching telemetry status. Telemetry may not be configured.';
            controlsElement.style.display = 'none';
            return;
        }
        
        // Update status text
        statusElement.textContent = status.enabled ? 
            `Telemetry is enabled (sending to ${status.endpoint} every ${status.send_interval_secs} seconds)` : 
            'Telemetry is currently disabled';
        
        // Update form fields with current values
        container.querySelector('#telemetry-enabled').checked = status.enabled;
        container.querySelector('#telemetry-endpoint').value = status.endpoint;
        container.querySelector('#telemetry-interval').value = status.send_interval_secs;
        container.querySelector('#telemetry-include-warnings').checked = status.include_warnings;
        container.querySelector('#telemetry-include-stats').checked = status.include_stats;
        container.querySelector('#telemetry-device-id').textContent = status.device_id;
        
        // Show the controls
        controlsElement.style.display = 'block';
    } catch (error) {
        statusElement.textContent = `Error: ${error.message}`;
        controlsElement.style.display = 'none';
    }
}

// Export functions for use in main.js
window.telemetry = {
    createTelemetryUI,
    refreshTelemetryStatus,
    getTelemetryStatus,
    updateTelemetrySettings
};