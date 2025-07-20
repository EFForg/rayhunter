#!/usr/bin/env python3
"""
Rayhunter GPS Correlation Results Summary

This tool provides a comprehensive summary of GPS correlation results
using the built-in Rayhunter correlation system.
"""

import json
import os
from pathlib import Path
import math

def analyze_gps_data(json_file: Path):
    """Analyze GPS correlation data and provide detailed summary"""
    
    with open(json_file, 'r') as f:
        data = json.load(f)
    
    gps_entries = data['gps_entries']
    lats = [entry['latitude'] for entry in gps_entries]
    lons = [entry['longitude'] for entry in gps_entries]
    
    def haversine_distance(lat1, lon1, lat2, lon2):
        """Calculate distance between two GPS points"""
        R = 6371000  # Earth's radius in meters
        lat1, lon1, lat2, lon2 = map(math.radians, [lat1, lon1, lat2, lon2])
        dlat = lat2 - lat1
        dlon = lon2 - lon1
        a = math.sin(dlat/2)**2 + math.cos(lat1) * math.cos(lat2) * math.sin(dlon/2)**2
        return 2 * R * math.asin(math.sqrt(a))
    
    # Calculate total distance traveled
    total_distance = 0
    for i in range(1, len(gps_entries)):
        prev = gps_entries[i-1]
        curr = gps_entries[i]
        total_distance += haversine_distance(
            prev['latitude'], prev['longitude'],
            curr['latitude'], curr['longitude']
        )
    
    return {
        'recording_id': data['recording_id'],
        'start_time': data['start_time'],
        'end_time': data['end_time'],
        'total_points': data['total_entries'],
        'duration_hours': (gps_entries[-1]['timestamp'] - gps_entries[0]['timestamp']) / 3600,
        'lat_range': (min(lats), max(lats)),
        'lon_range': (min(lons), max(lons)),
        'coverage_area_km2': abs((max(lats) - min(lats)) * (max(lons) - min(lons))) * 111.32 * 111.32,  # Rough approximation
        'total_distance_km': total_distance / 1000,
        'average_speed_kmh': (total_distance / 1000) / ((gps_entries[-1]['timestamp'] - gps_entries[0]['timestamp']) / 3600),
        'first_location': (gps_entries[0]['latitude'], gps_entries[0]['longitude']),
        'last_location': (gps_entries[-1]['latitude'], gps_entries[-1]['longitude'])
    }

def show_file_info(base_path: Path):
    """Show information about generated files"""
    files = {
        'CSV': base_path / 'rayhunter_correlation.csv',
        'JSON': base_path / 'rayhunter_correlation.json', 
        'GPX': base_path / 'rayhunter_correlation.gpx',
        'KML': base_path / 'rayhunter_correlation.kml'
    }
    
    print("📁 Generated Files:")
    print("=" * 50)
    
    for format_name, file_path in files.items():
        if file_path.exists():
            size_bytes = file_path.stat().st_size
            size_kb = size_bytes / 1024
            
            print(f"✅ {format_name}: {file_path.name}")
            print(f"   Size: {size_kb:.1f} KB ({size_bytes:,} bytes)")
            
            if format_name == 'CSV':
                with open(file_path, 'r') as f:
                    lines = len(f.readlines()) - 1  # Subtract header
                print(f"   Rows: {lines:,} GPS points")
            elif format_name == 'GPX':
                print(f"   Use with: Google Earth, QGIS, or other mapping software")
            elif format_name == 'KML':
                print(f"   Use with: Google Earth, Google Maps, or web mapping")
            elif format_name == 'JSON':
                print(f"   Use with: Programming, API integration, data analysis")
            print()

def main():
    tmp_dir = Path('/Users/beisenmann/rayhunter-enhanced/tmp')
    json_file = tmp_dir / 'rayhunter_correlation.json'
    
    if not json_file.exists():
        print("❌ GPS correlation results not found. Please run the correlation tool first.")
        return
    
    print("🎯 RAYHUNTER GPS CORRELATION RESULTS (Option 1)")
    print("=" * 60)
    print()
    
    # Analyze GPS data
    analysis = analyze_gps_data(json_file)
    
    print("📍 Recording Session Summary:")
    print("-" * 30)
    print(f"Recording ID: {analysis['recording_id']}")
    print(f"Start Time:   {analysis['start_time']}")
    print(f"End Time:     {analysis['end_time']}")
    print(f"Duration:     {analysis['duration_hours']:.1f} hours")
    print(f"GPS Points:   {analysis['total_points']:,}")
    print()
    
    print("🗺️  Geographic Coverage:")
    print("-" * 25)
    print(f"Latitude Range:  {analysis['lat_range'][0]:.6f} to {analysis['lat_range'][1]:.6f}")
    print(f"Longitude Range: {analysis['lon_range'][0]:.6f} to {analysis['lon_range'][1]:.6f}")
    print(f"Coverage Area:   ~{analysis['coverage_area_km2']:.2f} km²")
    print()
    
    print("🚗 Movement Analysis:")
    print("-" * 20)
    print(f"Start Location: {analysis['first_location'][0]:.6f}, {analysis['first_location'][1]:.6f}")
    print(f"End Location:   {analysis['last_location'][0]:.6f}, {analysis['last_location'][1]:.6f}")
    print(f"Total Distance: {analysis['total_distance_km']:.2f} km")
    print(f"Average Speed:  {analysis['average_speed_kmh']:.1f} km/h")
    print()
    
    # Show file information
    show_file_info(tmp_dir)
    
    print("🔧 Usage Examples:")
    print("=" * 50)
    print("📊 Data Analysis:")
    print("   import pandas as pd")
    print("   df = pd.read_csv('tmp/rayhunter_correlation.csv')")
    print("   df['timestamp'] = pd.to_datetime(df['timestamp'])")
    print()
    
    print("🗺️  Mapping Visualization:")
    print("   # Load KML file in Google Earth for best experience")
    print("   # Load GPX file in QGIS for advanced analysis")
    print("   # Use with mapping libraries (folium, plotly)")
    print()
    
    print("🔬 Cell Tower Correlation:")
    print("   # Next step: Correlate with cellular data")
    print("   # Match GPS timestamps with QMDL cellular observations")
    print("   # Analyze cell tower handoffs during movement")
    print()
    
    print("✅ SUCCESS: Rayhunter GPS correlation complete!")
    print(f"   All {analysis['total_points']} GPS points are now available in 4 formats")
    print("   Ready for cellular data correlation and analysis")

if __name__ == "__main__":
    main()
