#!/usr/bin/env python3
"""
Comprehensive Cellular CSV Exporter

Exports detailed cellular and GPS correlation data to CSV format
with all available cellular tower information for analysis.
"""

import csv
import json
import sys
import argparse
from pathlib import Path
from datetime import datetime, timezone
import subprocess

def export_comprehensive_csv(gps_file: Path, output_file: Path):
    """Export comprehensive cellular correlation to CSV format"""
    print(f"Generating comprehensive cellular CSV from GPS data: {gps_file}")
    
    # First run the comprehensive correlator to get the data
    print("Running comprehensive cellular correlation analysis...")
    
    # Import the correlator
    import sys
    sys.path.append(str(Path(__file__).parent))
    from comprehensive_cellular_correlator import EnhancedCellGpsCorrelator
    
    correlator = EnhancedCellGpsCorrelator(time_threshold=300)
    correlator.load_enhanced_gps_file(gps_file)
    correlator.create_simulated_cellular_data()
    correlations = correlator.correlate_comprehensive_data()
    
    print(f"Exporting {len(correlations)} correlations to CSV: {output_file}")
    
    with open(output_file, 'w', newline='', encoding='utf-8') as f:
        writer = csv.writer(f)
        
        # Comprehensive CSV header
        header = [
            # GPS Information
            'gps_timestamp', 'gps_datetime', 'latitude', 'longitude', 
            'speed_kmh', 'bearing_degrees', 'altitude_m',
            
            # Correlation Quality
            'correlation_quality', 'time_difference_seconds',
            
            # Cell Tower Identity
            'cell_id', 'physical_cell_id', 'tracking_area_code', 'location_area_code',
            
            # Network Operator
            'mobile_country_code', 'mobile_network_code', 'operator_name',
            
            # Radio Technology
            'radio_access_tech', 'frequency_band', 'frequency_description', 
            'channel_number', 'bandwidth_mhz',
            
            # LTE Signal Measurements
            'rsrp_dbm', 'rsrq_db', 'rssi_dbm', 'sinr_db', 'cqi',
            
            # 3G Signal Measurements  
            'rscp_dbm', 'ecno_db',
            
            # 2G Signal Measurements
            'rxlev_dbm', 'rxqual',
            
            # 5G Signal Measurements
            'ss_rsrp_dbm', 'ss_rsrq_db', 'ss_sinr_db',
            
            # Advanced Parameters
            'transmission_mode', 'mimo_layers', 'carrier_aggregation_bands',
            'timing_advance',
            
            # Connection State
            'rrc_state', 'attach_status', 'registration_state',
            
            # Quality Assessment
            'signal_quality', 'estimated_distance_m', 'coverage_type',
            
            # Neighbor Information
            'neighbor_cell_count', 'best_neighbor_pci', 'best_neighbor_rsrp',
            
            # Technical Details
            'cell_timestamp', 'cell_datetime', 'source', 'extraction_confidence'
        ]
        
        writer.writerow(header)
        
        # Export data
        for correlation in correlations:
            gps = correlation.gps_point
            
            if correlation.cell_observations:
                # Export each cell observation as a separate row
                for i, cell in enumerate(correlation.cell_observations):
                    time_diff = correlation.time_differences[i] if i < len(correlation.time_differences) else 0
                    
                    # Get frequency description
                    freq_desc = ""
                    if cell.frequency_band and cell.frequency_band in correlator.lte_bands:
                        freq_desc = correlator.lte_bands[cell.frequency_band]['freq']
                    
                    row = [
                        # GPS Information
                        gps.timestamp, gps.datetime_str, gps.latitude, gps.longitude,
                        gps.speed_kmh, gps.bearing, gps.altitude_m,
                        
                        # Correlation Quality
                        correlation.correlation_quality, time_diff,
                        
                        # Cell Tower Identity
                        cell.cell_id, cell.physical_cell_id, cell.tracking_area_code, 
                        cell.location_area_code,
                        
                        # Network Operator
                        cell.mobile_country_code, cell.mobile_network_code, cell.operator_name,
                        
                        # Radio Technology
                        cell.radio_access_tech, cell.frequency_band, freq_desc,
                        cell.channel_number, cell.bandwidth_mhz,
                        
                        # LTE Signal Measurements
                        cell.rsrp_dbm, cell.rsrq_db, cell.rssi_dbm, cell.sinr_db, cell.cqi,
                        
                        # 3G Signal Measurements
                        cell.rscp_dbm, cell.ecno_db,
                        
                        # 2G Signal Measurements
                        cell.rxlev_dbm, cell.rxqual,
                        
                        # 5G Signal Measurements
                        cell.ss_rsrp_dbm, cell.ss_rsrq_db, cell.ss_sinr_db,
                        
                        # Advanced Parameters
                        cell.transmission_mode, cell.mimo_layers, cell.carrier_aggregation_bands,
                        cell.timing_advance,
                        
                        # Connection State
                        cell.rrc_state, cell.attach_status, cell.registration_state,
                        
                        # Quality Assessment
                        cell.signal_quality, cell.estimated_distance_m, cell.coverage_type,
                        
                        # Neighbor Information
                        cell.neighbor_cell_count, cell.best_neighbor_pci, cell.best_neighbor_rsrp,
                        
                        # Technical Details
                        cell.timestamp, cell.datetime_str, cell.source, cell.extraction_confidence
                    ]
                    
                    writer.writerow(row)
            else:
                # No cellular data - export GPS-only row
                row = [
                    # GPS Information
                    gps.timestamp, gps.datetime_str, gps.latitude, gps.longitude,
                    gps.speed_kmh, gps.bearing, gps.altitude_m,
                    
                    # Correlation Quality
                    correlation.correlation_quality, float('inf'),
                    
                    # Fill rest with None/empty values
                    *[None] * (len(header) - 9)
                ]
                
                writer.writerow(row)
                
    print(f"Comprehensive CSV export complete! {len(correlations)} records exported.")

def main():
    parser = argparse.ArgumentParser(description='Export comprehensive cellular correlation to CSV')
    parser.add_argument('--gps', required=True, help='GPS file (.gps format)')
    parser.add_argument('--output', '-o', default='comprehensive_cellular_data.csv', help='Output CSV file')
    
    args = parser.parse_args()
    
    gps_file = Path(args.gps)
    if not gps_file.exists():
        print(f"Error: GPS file {gps_file} not found")
        sys.exit(1)
        
    output_file = Path(args.output)
    export_comprehensive_csv(gps_file, output_file)
    
    print(f"\n📊 Comprehensive cellular CSV generated: {output_file}")
    print("🔍 Import into Excel or Python for detailed analysis!")

if __name__ == "__main__":
    main()
