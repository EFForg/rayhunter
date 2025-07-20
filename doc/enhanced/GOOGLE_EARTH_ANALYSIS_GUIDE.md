# Google Earth and Analysis Guide

## Files Generated for Your Analysis

Your GPS correlation analysis has generated several files ready for different types of analysis:

### 📍 Google Earth Visualization
- **KML File**: `/Users/beisenmann/rayhunter-enhanced/tmp/rayhunter_correlation.kml` (49.8 KB)
- **GPX File**: `/Users/beisenmann/rayhunter-enhanced/tmp/rayhunter_correlation.gpx` (110.9 KB)

#### How to Load in Google Earth:
1. **Option 1 - Direct Open:**
   - Double-click the KML file
   - It should automatically open in Google Earth Pro

2. **Option 2 - Manual Import:**
   - Open Google Earth Pro
   - Go to File → Open
   - Select `rayhunter_correlation.kml`

3. **Option 3 - Google Earth Web:**
   - Go to earth.google.com
   - Click the menu (☰) → Projects → Open → Import KML file from computer
   - Upload the KML file

#### What You'll See in Google Earth:
- **Red Track Line**: Your complete 43.78 km journey path
- **Green Start Marker**: Beginning of your journey
- **Red End Marker**: End of your journey  
- **Blue Waypoints**: 21 evenly spaced points along the route
- **Popup Info**: Click any point to see timestamp, coordinates, and speed data

### 📊 Excel/Python Data Analysis
- **CSV File**: `/Users/beisenmann/rayhunter-enhanced/tmp/rayhunter_correlation.csv` (49.7 KB)
- **JSON File**: `/Users/beisenmann/rayhunter-enhanced/tmp/rayhunter_correlation.json` (151.5 KB)

#### CSV Analysis in Excel:
1. Open Excel
2. File → Open → Select `rayhunter_correlation.csv`
3. Data is organized with columns:
   - `timestamp`: Unix timestamp
   - `datetime`: Human-readable date/time
   - `latitude`, `longitude`: GPS coordinates
   - `speed_kmh`: Speed in km/h
   - `altitude`: Elevation data
   - `distance_km`: Cumulative distance

#### Python Analysis Example:
```python
import pandas as pd
import matplotlib.pyplot as plt

# Load the data
df = pd.read_csv('/Users/beisenmann/rayhunter-enhanced/tmp/rayhunter_correlation.csv')

# Convert datetime column
df['datetime'] = pd.to_datetime(df['datetime'])

# Plot speed over time
plt.figure(figsize=(12, 6))
plt.plot(df['datetime'], df['speed_kmh'])
plt.title('Speed During 43.78 km Journey')
plt.xlabel('Time')
plt.ylabel('Speed (km/h)')
plt.xticks(rotation=45)
plt.tight_layout()
plt.show()

# Journey statistics
print(f"Total Distance: {df['distance_km'].max():.2f} km")
print(f"Duration: {(df['timestamp'].max() - df['timestamp'].min()) / 3600:.2f} hours")
print(f"Average Speed: {df['speed_kmh'].mean():.2f} km/h")
print(f"Max Speed: {df['speed_kmh'].max():.2f} km/h")
```

### 🗼 Cellular Network Analysis

Your data includes 997 GPS points collected over 8.2 hours. To analyze cellular network behavior:

#### Cell Tower Correlation:
```bash
# Run the journey analyzer
cd /Users/beisenmann/rayhunter-enhanced
python3 tools/journey_analyzer.py \
    --json tmp/rayhunter_correlation.json \
    --csv tmp/rayhunter_correlation.csv \
    --output tmp/journey_analysis.json
```

#### Cell Handoff Analysis:
The journey analyzer will detect:
- Cell tower handoffs during movement
- Signal strength variations
- Network technology changes (3G/4G/5G)
- Operator network switches
- Dwell time in each cell coverage area

### 📈 Advanced Analysis Capabilities

#### 1. Speed Profile Analysis:
- Identify stops, acceleration, and cruising speeds
- Detect traffic patterns and congestion
- Calculate fuel efficiency estimates

#### 2. Route Optimization:
- Compare actual path vs. optimal routing
- Identify detours and alternative routes
- Traffic pattern analysis

#### 3. Cellular Network Performance:
- Signal strength mapping along the route
- Handoff frequency analysis
- Dead zone identification
- Carrier performance comparison

#### 4. Geospatial Analysis:
- Elevation profile analysis
- Urban vs. rural coverage comparison
- Geographic correlation with signal quality

## Quick Start Commands

```bash
# Navigate to your workspace
cd /Users/beisenmann/rayhunter-enhanced

# Install required Python packages for analysis
pip install pandas matplotlib seaborn

# Run comprehensive journey analysis
python3 tools/journey_analyzer.py \
    --json tmp/rayhunter_correlation.json \
    --csv tmp/rayhunter_correlation.csv \
    --output tmp/journey_analysis.json

# View the generated files
ls -la tmp/rayhunter_correlation.*
```

## File Locations Summary

All your analysis files are in: `/Users/beisenmann/rayhunter-enhanced/tmp/`

- `rayhunter_correlation.kml` - Google Earth visualization
- `rayhunter_correlation.gpx` - GPS track (also works in Google Earth)
- `rayhunter_correlation.csv` - Spreadsheet data
- `rayhunter_correlation.json` - Structured data for programming
- `journey_analysis.json` - Comprehensive analysis report (after running analyzer)

## Next Steps

1. **Visualize**: Open the KML file in Google Earth to see your 43.78 km journey
2. **Analyze**: Import CSV into Excel or Python for detailed analysis
3. **Correlate**: Run the journey analyzer to detect cellular network patterns
4. **Optimize**: Use insights for route planning and network analysis

Your data represents a substantial 8.2-hour journey with 997 GPS tracking points - perfect for comprehensive analysis!
