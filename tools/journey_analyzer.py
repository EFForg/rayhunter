#!/usr/bin/env python3
"""
Journey Analyzer

Comprehensive analysis tool for GPS journey data with cellular correlation capabilities.
Analyzes movement patterns, cell handoffs, and network behavior during travels.
"""

import pandas as pd
import json
import sys
import argparse
from pathlib import Path
from datetime import datetime, timezone
import math
from typing import List, Dict, Tuple, Optional
import matplotlib.pyplot as plt
import seaborn as sns
from dataclasses import dataclass

@dataclass
class JourneyStats:
    total_distance_km: float
    duration_hours: float
    avg_speed_kmh: float
    max_speed_kmh: float
    start_time: datetime
    end_time: datetime
    total_points: int
    
@dataclass
class CellHandoff:
    timestamp: int
    from_cell: Optional[str]
    to_cell: str
    latitude: float
    longitude: float
    distance_km: float
    speed_kmh: float

class JourneyAnalyzer:
    def __init__(self):
        self.gps_data = None
        self.correlation_data = None
        
    def load_csv_data(self, csv_file: Path) -> pd.DataFrame:
        """Load correlation CSV data"""
        print(f"Loading correlation data from {csv_file}")
        df = pd.read_csv(csv_file)
        print(f"Loaded {len(df)} correlation records")
        return df
        
    def load_json_data(self, json_file: Path) -> dict:
        """Load correlation JSON data"""
        print(f"Loading correlation data from {json_file}")
        with open(json_file, 'r') as f:
            data = json.load(f)
        print(f"Loaded correlation data with {len(data.get('gps_track', []))} GPS points")
        return data
        
    def calculate_distance(self, lat1: float, lon1: float, lat2: float, lon2: float) -> float:
        """Calculate distance between two GPS points in kilometers"""
        R = 6371  # Earth's radius in kilometers
        
        lat1_rad = math.radians(lat1)
        lon1_rad = math.radians(lon1)
        lat2_rad = math.radians(lat2)
        lon2_rad = math.radians(lon2)
        
        dlat = lat2_rad - lat1_rad
        dlon = lon2_rad - lon1_rad
        
        a = math.sin(dlat/2)**2 + math.cos(lat1_rad) * math.cos(lat2_rad) * math.sin(dlon/2)**2
        c = 2 * math.atan2(math.sqrt(a), math.sqrt(1-a))
        
        return R * c
        
    def analyze_journey_stats(self, gps_data: List[dict]) -> JourneyStats:
        """Calculate comprehensive journey statistics"""
        if len(gps_data) < 2:
            raise ValueError("Need at least 2 GPS points for analysis")
            
        # Sort by timestamp
        gps_data = sorted(gps_data, key=lambda x: x['timestamp'])
        
        total_distance = 0.0
        max_speed = 0.0
        speeds = []
        
        for i in range(1, len(gps_data)):
            prev_point = gps_data[i-1]
            curr_point = gps_data[i]
            
            # Calculate distance
            distance = self.calculate_distance(
                prev_point['latitude'], prev_point['longitude'],
                curr_point['latitude'], curr_point['longitude']
            )
            total_distance += distance
            
            # Calculate speed
            time_diff = curr_point['timestamp'] - prev_point['timestamp']
            if time_diff > 0:
                speed_ms = (distance * 1000) / time_diff  # m/s
                speed_kmh = speed_ms * 3.6  # km/h
                speeds.append(speed_kmh)
                max_speed = max(max_speed, speed_kmh)
        
        start_time = datetime.fromtimestamp(gps_data[0]['timestamp'], tz=timezone.utc)
        end_time = datetime.fromtimestamp(gps_data[-1]['timestamp'], tz=timezone.utc)
        duration_seconds = gps_data[-1]['timestamp'] - gps_data[0]['timestamp']
        duration_hours = duration_seconds / 3600
        
        avg_speed = total_distance / duration_hours if duration_hours > 0 else 0
        
        return JourneyStats(
            total_distance_km=total_distance,
            duration_hours=duration_hours,
            avg_speed_kmh=avg_speed,
            max_speed_kmh=max_speed,
            start_time=start_time,
            end_time=end_time,
            total_points=len(gps_data)
        )
        
    def detect_cell_handoffs(self, correlation_data: pd.DataFrame) -> List[CellHandoff]:
        """Detect cell tower handoffs during the journey"""
        handoffs = []
        
        if 'cell_id' not in correlation_data.columns:
            print("Warning: No cell_id data available for handoff analysis")
            return handoffs
            
        # Sort by timestamp
        df = correlation_data.sort_values('gps_timestamp').copy()
        df = df.dropna(subset=['cell_id', 'latitude', 'longitude'])
        
        if len(df) < 2:
            print("Warning: Insufficient cell data for handoff analysis")
            return handoffs
            
        prev_cell = None
        
        for idx, row in df.iterrows():
            current_cell = str(row['cell_id'])
            
            if prev_cell and prev_cell != current_cell:
                # Calculate distance and speed at handoff point
                if idx > 0:
                    prev_row = df.iloc[df.index.get_loc(idx) - 1]
                    distance = self.calculate_distance(
                        prev_row['latitude'], prev_row['longitude'],
                        row['latitude'], row['longitude']
                    )
                    time_diff = row['gps_timestamp'] - prev_row['gps_timestamp']
                    speed = (distance * 3600) / time_diff if time_diff > 0 else 0
                else:
                    distance = 0
                    speed = 0
                    
                handoffs.append(CellHandoff(
                    timestamp=int(row['gps_timestamp']),
                    from_cell=prev_cell,
                    to_cell=current_cell,
                    latitude=row['latitude'],
                    longitude=row['longitude'],
                    distance_km=distance,
                    speed_kmh=speed
                ))
                
            prev_cell = current_cell
            
        print(f"Detected {len(handoffs)} cell handoffs during journey")
        return handoffs
        
    def analyze_network_behavior(self, correlation_data: pd.DataFrame) -> Dict:
        """Analyze cellular network behavior patterns"""
        analysis = {
            'unique_cells': 0,
            'cell_dwell_times': {},
            'signal_strength_stats': {},
            'rat_distribution': {},
            'operator_analysis': {}
        }
        
        if len(correlation_data) == 0:
            return analysis
            
        # Unique cells encountered
        if 'cell_id' in correlation_data.columns:
            unique_cells = correlation_data['cell_id'].dropna().nunique()
            analysis['unique_cells'] = unique_cells
            
            # Cell dwell times (time spent connected to each cell)
            cell_times = {}
            df_sorted = correlation_data.sort_values('gps_timestamp')
            
            for cell_id in df_sorted['cell_id'].dropna().unique():
                cell_data = df_sorted[df_sorted['cell_id'] == cell_id]
                if len(cell_data) > 1:
                    total_time = cell_data['gps_timestamp'].max() - cell_data['gps_timestamp'].min()
                    cell_times[str(cell_id)] = total_time / 60  # Convert to minutes
                    
            analysis['cell_dwell_times'] = cell_times
            
        # Signal strength analysis
        signal_cols = ['rsrp', 'rsrq', 'rssi']
        for col in signal_cols:
            if col in correlation_data.columns:
                signal_data = correlation_data[col].dropna()
                if len(signal_data) > 0:
                    analysis['signal_strength_stats'][col] = {
                        'mean': float(signal_data.mean()),
                        'std': float(signal_data.std()),
                        'min': float(signal_data.min()),
                        'max': float(signal_data.max()),
                        'median': float(signal_data.median())
                    }
                    
        # RAT (Radio Access Technology) distribution
        if 'rat' in correlation_data.columns:
            rat_counts = correlation_data['rat'].value_counts()
            analysis['rat_distribution'] = rat_counts.to_dict()
            
        # Operator analysis (MCC/MNC)
        if 'mcc' in correlation_data.columns and 'mnc' in correlation_data.columns:
            operators = correlation_data.dropna(subset=['mcc', 'mnc'])
            if len(operators) > 0:
                operator_combos = operators.groupby(['mcc', 'mnc']).size()
                analysis['operator_analysis'] = operator_combos.to_dict()
                
        return analysis
        
    def generate_journey_report(self, json_file: Path, csv_file: Path, output_file: Path) -> None:
        """Generate comprehensive journey analysis report"""
        print(f"Generating comprehensive journey report...")
        
        # Load data
        json_data = self.load_json_data(json_file)
        csv_data = self.load_csv_data(csv_file)
        
        # Analyze journey
        gps_track = json_data.get('gps_track', [])
        journey_stats = self.analyze_journey_stats(gps_track)
        
        # Analyze cellular behavior
        handoffs = self.detect_cell_handoffs(csv_data)
        network_analysis = self.analyze_network_behavior(csv_data)
        
        # Generate report
        report = {
            'analysis_timestamp': datetime.now(timezone.utc).isoformat(),
            'journey_overview': {
                'total_distance_km': round(journey_stats.total_distance_km, 2),
                'duration_hours': round(journey_stats.duration_hours, 2),
                'average_speed_kmh': round(journey_stats.avg_speed_kmh, 2),
                'max_speed_kmh': round(journey_stats.max_speed_kmh, 2),
                'start_time': journey_stats.start_time.isoformat(),
                'end_time': journey_stats.end_time.isoformat(),
                'total_gps_points': journey_stats.total_points
            },
            'cellular_analysis': {
                'total_handoffs': len(handoffs),
                'unique_cells_encountered': network_analysis['unique_cells'],
                'handoff_frequency_per_km': len(handoffs) / journey_stats.total_distance_km if journey_stats.total_distance_km > 0 else 0,
                'signal_strength_analysis': network_analysis['signal_strength_stats'],
                'radio_technology_distribution': network_analysis['rat_distribution'],
                'operator_distribution': network_analysis['operator_analysis']
            },
            'handoff_details': [
                {
                    'timestamp': handoff.timestamp,
                    'datetime': datetime.fromtimestamp(handoff.timestamp, tz=timezone.utc).isoformat(),
                    'from_cell': handoff.from_cell,
                    'to_cell': handoff.to_cell,
                    'latitude': handoff.latitude,
                    'longitude': handoff.longitude,
                    'speed_kmh': round(handoff.speed_kmh, 2)
                }
                for handoff in handoffs
            ],
            'cell_dwell_times_minutes': network_analysis['cell_dwell_times']
        }
        
        # Save report
        with open(output_file, 'w') as f:
            json.dump(report, f, indent=2)
            
        print(f"Journey analysis report saved to {output_file}")
        
        # Print summary
        print("\n" + "="*60)
        print("JOURNEY ANALYSIS SUMMARY")
        print("="*60)
        print(f"📍 Distance: {journey_stats.total_distance_km:.2f} km")
        print(f"⏱️  Duration: {journey_stats.duration_hours:.2f} hours")
        print(f"🚗 Average Speed: {journey_stats.avg_speed_kmh:.2f} km/h")
        print(f"⚡ Max Speed: {journey_stats.max_speed_kmh:.2f} km/h")
        print(f"📶 Cell Handoffs: {len(handoffs)}")
        print(f"🗼 Unique Cells: {network_analysis['unique_cells']}")
        print(f"📱 Handoffs per km: {len(handoffs) / journey_stats.total_distance_km:.2f}")
        
        if network_analysis['signal_strength_stats']:
            print(f"\n📊 Signal Strength (RSRP):")
            rsrp = network_analysis['signal_strength_stats'].get('rsrp', {})
            if rsrp:
                print(f"   Average: {rsrp.get('mean', 0):.1f} dBm")
                print(f"   Range: {rsrp.get('min', 0):.1f} to {rsrp.get('max', 0):.1f} dBm")
                
        print("\n🎯 Next Steps:")
        print("1. Open KML file in Google Earth for visualization")
        print("2. Import CSV into Excel/Python for detailed analysis")
        print("3. Review handoff patterns for network optimization insights")
        print("4. Analyze signal strength trends along the route")
        
def main():
    parser = argparse.ArgumentParser(description='Analyze GPS journey with cellular correlation')
    parser.add_argument('--json', required=True, help='JSON correlation file')
    parser.add_argument('--csv', required=True, help='CSV correlation file')
    parser.add_argument('--output', '-o', default='journey_analysis.json', help='Output analysis file')
    
    args = parser.parse_args()
    
    analyzer = JourneyAnalyzer()
    
    json_file = Path(args.json)
    csv_file = Path(args.csv)
    output_file = Path(args.output)
    
    if not json_file.exists():
        print(f"Error: JSON file {json_file} not found")
        sys.exit(1)
        
    if not csv_file.exists():
        print(f"Error: CSV file {csv_file} not found")
        sys.exit(1)
        
    analyzer.generate_journey_report(json_file, csv_file, output_file)

if __name__ == "__main__":
    main()
