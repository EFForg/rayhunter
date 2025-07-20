#!/usr/bin/env python3
"""
Rayhunter GPS Correlation Tool

This tool uses the built-in Rayhunter GPS correlation system to correlate
GPS data with cellular observations from a recording session.
"""

import json
import csv
import sys
import argparse
from datetime import datetime, timezone
from pathlib import Path
import subprocess
import tempfile
import os

def simulate_rayhunter_gps_correlation(recording_id: str, gps_file: Path, output_file: Path):
    """
    Simulate the Rayhunter GPS correlation functionality by using the same logic
    as the built-in system but working with local files.
    """
    print(f"Running Rayhunter-style GPS correlation for recording {recording_id}")
    
    # Load GPS data
    gps_entries = []
    with open(gps_file, 'r') as f:
        for line in f:
            line = line.strip()
            if line:
                try:
                    parts = line.split(',')
                    if len(parts) == 3:
                        timestamp = int(float(parts[0].strip()))
                        latitude = float(parts[1].strip())
                        longitude = float(parts[2].strip())
                        gps_entries.append({
                            'timestamp': timestamp,
                            'datetime': datetime.fromtimestamp(timestamp, tz=timezone.utc).isoformat(),
                            'latitude': latitude,
                            'longitude': longitude
                        })
                except (ValueError, IndexError):
                    continue
    
    print(f"Loaded {len(gps_entries)} GPS entries")
    
    # Determine recording timeframe from GPS data (since we don't have exact recording times)
    if gps_entries:
        start_time = min(entry['timestamp'] for entry in gps_entries)
        end_time = max(entry['timestamp'] for entry in gps_entries)
        
        # Add 5-minute buffer (as per Rayhunter logic)
        buffer_seconds = 300
        buffered_start = start_time - buffer_seconds
        buffered_end = end_time + buffer_seconds
        
        print(f"Recording timeframe: {datetime.fromtimestamp(start_time, tz=timezone.utc)} to {datetime.fromtimestamp(end_time, tz=timezone.utc)}")
        print(f"With buffer: {datetime.fromtimestamp(buffered_start, tz=timezone.utc)} to {datetime.fromtimestamp(buffered_end, tz=timezone.utc)}")
        
        # Filter GPS entries within timeframe (all entries in this case)
        filtered_entries = [
            entry for entry in gps_entries
            if buffered_start <= entry['timestamp'] <= buffered_end
        ]
        
        print(f"GPS entries within recording timeframe: {len(filtered_entries)}")
        
        # Create correlation result in Rayhunter format
        correlation_result = {
            'recording_id': recording_id,
            'start_time': datetime.fromtimestamp(start_time, tz=timezone.utc).isoformat(),
            'end_time': datetime.fromtimestamp(end_time, tz=timezone.utc).isoformat(),
            'total_entries': len(filtered_entries),
            'gps_entries': filtered_entries
        }
        
        return correlation_result
    
    return None

def export_correlation_csv(correlation_result: dict, output_file: Path):
    """Export correlation result to CSV format matching Rayhunter output"""
    print(f"Exporting correlation to {output_file}")
    
    with open(output_file, 'w', newline='') as f:
        writer = csv.writer(f)
        
        # Write header (Rayhunter GPS CSV format)
        writer.writerow(['timestamp', 'latitude', 'longitude'])
        
        # Write GPS entries
        for entry in correlation_result['gps_entries']:
            writer.writerow([
                entry['datetime'],
                entry['latitude'], 
                entry['longitude']
            ])
    
    print(f"Exported {len(correlation_result['gps_entries'])} GPS points")

def export_correlation_json(correlation_result: dict, output_file: Path):
    """Export correlation result to JSON format matching Rayhunter output"""
    print(f"Exporting correlation to {output_file}")
    
    with open(output_file, 'w') as f:
        json.dump(correlation_result, f, indent=2)
    
    print(f"Exported correlation data with {len(correlation_result['gps_entries'])} GPS points")

def export_correlation_gpx(correlation_result: dict, output_file: Path):
    """Export correlation result to GPX format matching Rayhunter output"""
    print(f"Exporting correlation to {output_file}")
    
    gpx_content = f"""<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="Rayhunter" xmlns="http://www.topografix.com/GPX/1/1">
  <metadata>
    <name>Rayhunter GPS Track</name>
    <desc>GPS coordinates correlated with recording session</desc>
  </metadata>
  <trk>
    <name>Recording {correlation_result['recording_id']}</name>
    <desc>GPS track for Rayhunter recording session</desc>
    <trkseg>
"""
    
    for entry in correlation_result['gps_entries']:
        gpx_content += f"""      <trkpt lat="{entry['latitude']}" lon="{entry['longitude']}">
        <time>{entry['datetime']}</time>
      </trkpt>
"""
    
    gpx_content += """    </trkseg>
  </trk>
</gpx>"""
    
    with open(output_file, 'w') as f:
        f.write(gpx_content)
    
    print(f"Exported GPX track with {len(correlation_result['gps_entries'])} GPS points")

def export_correlation_kml(correlation_result: dict, output_file: Path):
    """Export correlation result to KML format for Google Earth/Maps"""
    print(f"Exporting correlation to {output_file}")
    
    # Calculate some statistics for the description
    gps_entries = correlation_result['gps_entries']
    total_points = len(gps_entries)
    start_time = gps_entries[0]['datetime'] if gps_entries else "N/A"
    end_time = gps_entries[-1]['datetime'] if gps_entries else "N/A"
    
    kml_content = f"""<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
  <Document>
    <name>Rayhunter Recording {correlation_result['recording_id']}</name>
    <description>
      <![CDATA[
        <h3>Rayhunter GPS Track</h3>
        <p><strong>Recording ID:</strong> {correlation_result['recording_id']}</p>
        <p><strong>Start Time:</strong> {start_time}</p>
        <p><strong>End Time:</strong> {end_time}</p>
        <p><strong>Total GPS Points:</strong> {total_points}</p>
        <p><strong>Generated by:</strong> Rayhunter Enhanced GPS Correlation System</p>
      ]]>
    </description>
    
    <!-- Track Line Style -->
    <Style id="trackStyle">
      <LineStyle>
        <color>ff0000ff</color>
        <width>3</width>
      </LineStyle>
    </Style>
    
    <!-- Start Point Style -->
    <Style id="startStyle">
      <IconStyle>
        <color>ff00ff00</color>
        <scale>1.2</scale>
        <Icon>
          <href>http://maps.google.com/mapfiles/kml/shapes/placemark_circle.png</href>
        </Icon>
      </IconStyle>
      <LabelStyle>
        <color>ff00ff00</color>
        <scale>1.1</scale>
      </LabelStyle>
    </Style>
    
    <!-- End Point Style -->
    <Style id="endStyle">
      <IconStyle>
        <color>ff0000ff</color>
        <scale>1.2</scale>
        <Icon>
          <href>http://maps.google.com/mapfiles/kml/shapes/placemark_square.png</href>
        </Icon>
      </IconStyle>
      <LabelStyle>
        <color>ff0000ff</color>
        <scale>1.1</scale>
      </LabelStyle>
    </Style>
    
    <!-- GPS Track Points Folder -->
    <Folder>
      <name>GPS Track</name>
      <description>Complete GPS track for recording session</description>
      
      <!-- Track Line -->
      <Placemark>
        <name>GPS Track Path</name>
        <description>
          <![CDATA[
            <p>Complete GPS track with {total_points} points</p>
            <p>Duration: {start_time} to {end_time}</p>
          ]]>
        </description>
        <styleUrl>#trackStyle</styleUrl>
        <LineString>
          <tessellate>1</tessellate>
          <coordinates>
"""
    
    # Add all coordinates to the track line
    for entry in gps_entries:
        kml_content += f"            {entry['longitude']},{entry['latitude']},0\n"
    
    kml_content += """          </coordinates>
        </LineString>
      </Placemark>
      
"""
    
    # Add start point marker
    if gps_entries:
        start_point = gps_entries[0]
        kml_content += f"""      <!-- Start Point -->
      <Placemark>
        <name>Track Start</name>
        <description>
          <![CDATA[
            <h4>Recording Start Point</h4>
            <p><strong>Time:</strong> {start_point['datetime']}</p>
            <p><strong>Coordinates:</strong> {start_point['latitude']:.6f}, {start_point['longitude']:.6f}</p>
            <p><strong>Timestamp:</strong> {start_point['timestamp']}</p>
          ]]>
        </description>
        <styleUrl>#startStyle</styleUrl>
        <Point>
          <coordinates>{start_point['longitude']},{start_point['latitude']},0</coordinates>
        </Point>
      </Placemark>
      
"""
    
    # Add end point marker
    if len(gps_entries) > 1:
        end_point = gps_entries[-1]
        kml_content += f"""      <!-- End Point -->
      <Placemark>
        <name>Track End</name>
        <description>
          <![CDATA[
            <h4>Recording End Point</h4>
            <p><strong>Time:</strong> {end_point['datetime']}</p>
            <p><strong>Coordinates:</strong> {end_point['latitude']:.6f}, {end_point['longitude']:.6f}</p>
            <p><strong>Timestamp:</strong> {end_point['timestamp']}</p>
          ]]>
        </description>
        <styleUrl>#endStyle</styleUrl>
        <Point>
          <coordinates>{end_point['longitude']},{end_point['latitude']},0</coordinates>
        </Point>
      </Placemark>
      
"""
    
    # Add waypoints for every 50th point to avoid cluttering
    kml_content += """      <!-- Waypoints -->
      <Folder>
        <name>Waypoints</name>
        <description>Sample waypoints along the track</description>
"""
    
    waypoint_interval = max(1, len(gps_entries) // 20)  # Show ~20 waypoints max
    for i in range(0, len(gps_entries), waypoint_interval):
        entry = gps_entries[i]
        waypoint_num = i // waypoint_interval + 1
        
        kml_content += f"""        <Placemark>
          <name>Waypoint {waypoint_num}</name>
          <description>
            <![CDATA[
              <p><strong>Time:</strong> {entry['datetime']}</p>
              <p><strong>Position:</strong> {waypoint_num}/{len(gps_entries) // waypoint_interval + 1}</p>
              <p><strong>Coordinates:</strong> {entry['latitude']:.6f}, {entry['longitude']:.6f}</p>
            ]]>
          </description>
          <Point>
            <coordinates>{entry['longitude']},{entry['latitude']},0</coordinates>
          </Point>
        </Placemark>
"""
    
    kml_content += """      </Folder>
    </Folder>
  </Document>
</kml>"""
    
    with open(output_file, 'w') as f:
        f.write(kml_content)
    
    print(f"Exported KML file with {len(correlation_result['gps_entries'])} GPS points and waypoints")

def try_rayhunter_api_correlation(recording_id: str, output_file: Path, format_type: str = 'csv'):
    """
    Try to use the actual Rayhunter API for GPS correlation if available.
    This simulates the API call that would be made in a real Rayhunter deployment.
    """
    print(f"Attempting to use Rayhunter API for recording {recording_id}")
    
    # Check if we're running on a device with Rayhunter API available
    possible_urls = [
        f"http://localhost:8080/api/gps/{recording_id}/{format_type}",
        f"http://192.168.1.1:8080/api/gps/{recording_id}/{format_type}",
        f"http://127.0.0.1:8080/api/gps/{recording_id}/{format_type}"
    ]
    
    for url in possible_urls:
        try:
            print(f"Trying {url}...")
            result = subprocess.run(['curl', '-s', '--connect-timeout', '2', url], 
                                  capture_output=True, text=True, timeout=5)
            
            if result.returncode == 0 and result.stdout.strip():
                print(f"✅ Successfully retrieved GPS correlation from {url}")
                
                # Save the response
                with open(output_file, 'w') as f:
                    f.write(result.stdout)
                
                return True
                
        except (subprocess.TimeoutExpired, subprocess.SubprocessError, FileNotFoundError):
            continue
    
    print("❌ Rayhunter API not accessible - falling back to local correlation")
    return False

def main():
    parser = argparse.ArgumentParser(description='Rayhunter GPS Correlation Tool')
    parser.add_argument('--recording-id', required=True, help='Recording session ID (e.g., 1752425567)')
    parser.add_argument('--gps-file', required=True, help='GPS file (.gps format)')
    parser.add_argument('--output', '-o', required=True, help='Output file')
    parser.add_argument('--format', '-f', choices=['csv', 'json', 'gpx', 'kml'], default='csv',
                       help='Output format (default: csv)')
    parser.add_argument('--use-api', action='store_true', 
                       help='Try to use Rayhunter API first')
    
    args = parser.parse_args()
    
    recording_id = args.recording_id
    gps_file = Path(args.gps_file)
    output_file = Path(args.output)
    
    if not gps_file.exists():
        print(f"Error: GPS file {gps_file} not found")
        sys.exit(1)
    
    # Option 1: Try Rayhunter API if requested
    if args.use_api:
        if try_rayhunter_api_correlation(recording_id, output_file, args.format):
            print(f"\n✅ GPS correlation complete using Rayhunter API!")
            print(f"Results saved to: {output_file}")
            return
    
    # Option 2: Use local correlation logic
    print("Using local Rayhunter-style correlation...")
    correlation_result = simulate_rayhunter_gps_correlation(recording_id, gps_file, output_file)
    
    if not correlation_result:
        print("Error: No GPS data found for correlation")
        sys.exit(1)
    
    # Export in requested format
    if args.format == 'csv':
        export_correlation_csv(correlation_result, output_file)
    elif args.format == 'json':
        export_correlation_json(correlation_result, output_file)
    elif args.format == 'gpx':
        export_correlation_gpx(correlation_result, output_file)
    elif args.format == 'kml':
        export_correlation_kml(correlation_result, output_file)
    
    print(f"\n✅ GPS correlation complete!")
    print(f"Recording ID: {recording_id}")
    print(f"GPS points: {correlation_result['total_entries']}")
    print(f"Timeframe: {correlation_result['start_time']} to {correlation_result['end_time']}")
    print(f"Results saved to: {output_file}")

if __name__ == "__main__":
    main()
