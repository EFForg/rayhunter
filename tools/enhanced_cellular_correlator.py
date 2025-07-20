#!/usr/bin/env python3
"""
Enhanced Cellular GPS Correlator

This tool provides comprehensive cellular tower correlation with GPS data,
including detailed cellular information for visualization and analysis.
"""

import json
import csv
import sys
import argparse
from datetime import datetime, timezone
from pathlib import Path
from typing import List, Dict, Optional, Any
import struct
import binascii
from dataclasses import dataclass, asdict

@dataclass 
class EnhancedCellTower:
    # Basic identification
    cell_id: Optional[int] = None
    physical_cell_id: Optional[int] = None  # PCI
    tracking_area_code: Optional[int] = None  # TAC
    location_area_code: Optional[int] = None  # LAC
    
    # Network operator
    mobile_country_code: Optional[int] = None  # MCC
    mobile_network_code: Optional[int] = None  # MNC
    operator_name: Optional[str] = None
    
    # Technology details
    radio_access_tech: Optional[str] = None  # LTE, UMTS, GSM, NR
    frequency_band: Optional[int] = None
    channel_number: Optional[int] = None  # EARFCN/UARFCN/ARFCN
    bandwidth: Optional[str] = None
    
    # LTE Signal measurements
    rsrp_dbm: Optional[float] = None  # Reference Signal Received Power
    rsrq_db: Optional[float] = None   # Reference Signal Received Quality
    rssi_dbm: Optional[float] = None  # Received Signal Strength Indicator
    sinr_db: Optional[float] = None   # Signal to Interference plus Noise Ratio
    cqi: Optional[int] = None         # Channel Quality Indicator
    
    # UMTS/3G measurements
    rscp_dbm: Optional[float] = None  # Received Signal Code Power
    ecno_db: Optional[float] = None   # Energy per Chip to Noise ratio
    
    # GSM/2G measurements
    rxlev_dbm: Optional[float] = None # Received Level
    rxqual: Optional[int] = None      # Received Quality
    
    # 5G NR measurements
    ss_rsrp_dbm: Optional[float] = None
    ss_rsrq_db: Optional[float] = None
    ss_sinr_db: Optional[float] = None
    
    # Additional technical details
    transmission_mode: Optional[str] = None
    mimo_layers: Optional[int] = None
    carrier_aggregation: Optional[List[int]] = None
    timing_advance: Optional[int] = None
    
    # Connection state
    rrc_state: Optional[str] = None
    attach_status: Optional[str] = None
    
    # Neighbor cells
    neighbor_cells: Optional[List[Dict]] = None
    
    # Coverage quality indicators
    signal_quality: Optional[str] = None  # Excellent/Good/Fair/Poor
    estimated_distance_m: Optional[float] = None
    
    def to_html_description(self) -> str:
        """Generate detailed HTML description for KML popup"""
        html = "<h3>📡 Cell Tower Information</h3><table border='1' cellpadding='3'>"
        
        # Basic Identity
        html += "<tr bgcolor='#E8E8E8'><td colspan='2'><b>🆔 Cell Identity</b></td></tr>"
        if self.cell_id:
            html += f"<tr><td>Cell ID</td><td>{self.cell_id}</td></tr>"
        if self.physical_cell_id:
            html += f"<tr><td>Physical Cell ID (PCI)</td><td>{self.physical_cell_id}</td></tr>"
        if self.tracking_area_code:
            html += f"<tr><td>Tracking Area Code (TAC)</td><td>{self.tracking_area_code}</td></tr>"
        if self.location_area_code:
            html += f"<tr><td>Location Area Code (LAC)</td><td>{self.location_area_code}</td></tr>"
            
        # Network Operator
        if self.mobile_country_code or self.mobile_network_code:
            html += "<tr bgcolor='#E8E8E8'><td colspan='2'><b>🌐 Network Operator</b></td></tr>"
            if self.mobile_country_code:
                html += f"<tr><td>Mobile Country Code (MCC)</td><td>{self.mobile_country_code}</td></tr>"
            if self.mobile_network_code:
                html += f"<tr><td>Mobile Network Code (MNC)</td><td>{self.mobile_network_code}</td></tr>"
            if self.operator_name:
                html += f"<tr><td>Operator Name</td><td>{self.operator_name}</td></tr>"
                
        # Technology
        if self.radio_access_tech or self.frequency_band:
            html += "<tr bgcolor='#E8E8E8'><td colspan='2'><b>📶 Radio Technology</b></td></tr>"
            if self.radio_access_tech:
                html += f"<tr><td>Radio Access Technology</td><td>{self.radio_access_tech}</td></tr>"
            if self.frequency_band:
                html += f"<tr><td>Frequency Band</td><td>Band {self.frequency_band}</td></tr>"
            if self.channel_number:
                html += f"<tr><td>Channel Number</td><td>{self.channel_number}</td></tr>"
            if self.bandwidth:
                html += f"<tr><td>Bandwidth</td><td>{self.bandwidth}</td></tr>"
                
        # Signal Quality
        signal_rows = []
        if self.rsrp_dbm:
            signal_rows.append(f"<tr><td>RSRP</td><td>{self.rsrp_dbm:.1f} dBm</td></tr>")
        if self.rsrq_db:
            signal_rows.append(f"<tr><td>RSRQ</td><td>{self.rsrq_db:.1f} dB</td></tr>")
        if self.rssi_dbm:
            signal_rows.append(f"<tr><td>RSSI</td><td>{self.rssi_dbm:.1f} dBm</td></tr>")
        if self.sinr_db:
            signal_rows.append(f"<tr><td>SINR</td><td>{self.sinr_db:.1f} dB</td></tr>")
        if self.rscp_dbm:
            signal_rows.append(f"<tr><td>RSCP (3G)</td><td>{self.rscp_dbm:.1f} dBm</td></tr>")
        if self.ecno_db:
            signal_rows.append(f"<tr><td>Ec/No (3G)</td><td>{self.ecno_db:.1f} dB</td></tr>")
        if self.rxlev_dbm:
            signal_rows.append(f"<tr><td>RxLev (2G)</td><td>{self.rxlev_dbm:.1f} dBm</td></tr>")
            
        if signal_rows:
            html += "<tr bgcolor='#E8E8E8'><td colspan='2'><b>📊 Signal Measurements</b></td></tr>"
            html += "".join(signal_rows)
            
        # Quality assessment
        if self.signal_quality:
            html += f"<tr bgcolor='#E8E8E8'><td colspan='2'><b>📈 Signal Quality: {self.signal_quality}</b></td></tr>"
        if self.estimated_distance_m:
            html += f"<tr><td>Estimated Distance</td><td>{self.estimated_distance_m:.0f} meters</td></tr>"
            
        # Connection state
        if self.rrc_state or self.attach_status:
            html += "<tr bgcolor='#E8E8E8'><td colspan='2'><b>🔗 Connection State</b></td></tr>"
            if self.rrc_state:
                html += f"<tr><td>RRC State</td><td>{self.rrc_state}</td></tr>"
            if self.attach_status:
                html += f"<tr><td>Attach Status</td><td>{self.attach_status}</td></tr>"
                
        # Neighbor cells
        if self.neighbor_cells:
            html += f"<tr bgcolor='#E8E8E8'><td colspan='2'><b>🗼 Neighbor Cells ({len(self.neighbor_cells)})</b></td></tr>"
            for i, neighbor in enumerate(self.neighbor_cells[:5]):  # Show max 5 neighbors
                if 'pci' in neighbor:
                    rsrp_str = f", RSRP: {neighbor.get('rsrp_dbm', 'N/A'):.1f} dBm" if neighbor.get('rsrp_dbm') else ""
                    html += f"<tr><td>Neighbor {i+1}</td><td>PCI {neighbor['pci']}{rsrp_str}</td></tr>"
                    
        html += "</table>"
        return html

class EnhancedCellularCorrelator:
    def __init__(self):
        self.operator_names = {
            (310, 260): "T-Mobile US",
            (310, 410): "AT&T",
            (311, 480): "Verizon",
            (310, 120): "Sprint",
            (302, 220): "Telus (Canada)",
            (302, 610): "Bell (Canada)",
            (302, 720): "Rogers (Canada)",
            # Add more as needed
        }
        
    def analyze_signal_quality(self, cell: EnhancedCellTower) -> str:
        """Analyze signal quality based on measurements"""
        if cell.rsrp_dbm:
            if cell.rsrp_dbm >= -80:
                return "Excellent"
            elif cell.rsrp_dbm >= -90:
                return "Good" 
            elif cell.rsrp_dbm >= -100:
                return "Fair"
            else:
                return "Poor"
        elif cell.rscp_dbm:  # 3G
            if cell.rscp_dbm >= -70:
                return "Excellent"
            elif cell.rscp_dbm >= -85:
                return "Good"
            elif cell.rscp_dbm >= -95:
                return "Fair"
            else:
                return "Poor"
        elif cell.rxlev_dbm:  # 2G
            if cell.rxlev_dbm >= -70:
                return "Excellent"
            elif cell.rxlev_dbm >= -85:
                return "Good"
            elif cell.rxlev_dbm >= -95:
                return "Fair"
            else:
                return "Poor"
        return "Unknown"
        
    def estimate_distance(self, cell: EnhancedCellTower) -> Optional[float]:
        """Estimate distance to cell tower based on signal strength"""
        if cell.rsrp_dbm:
            # Simplified path loss model for LTE
            # Free space path loss: RSRP = TxPower - PathLoss
            # Assuming TxPower ~43 dBm, frequency ~2 GHz
            path_loss = 43 - cell.rsrp_dbm
            if path_loss > 32:  # Minimum free space loss
                # Distance = 10^((PathLoss - 32.45 - 20*log10(freq_MHz)) / 20)
                # For 2000 MHz: constant = 32.45 + 20*log10(2000) = 98.5
                distance_km = 10 ** ((path_loss - 98.5) / 20)
                return distance_km * 1000  # Convert to meters
        return None
        
    def correlate_with_enhanced_data(self, gps_file: Path, cellular_file: Path, 
                                   correlation_window: int = 300) -> Dict:
        """Correlate GPS data with enhanced cellular information"""
        print(f"Loading GPS data from {gps_file}")
        
        # Load GPS data
        gps_data = []
        with open(gps_file, 'r') as f:
            for line in f:
                line = line.strip()
                if line:
                    try:
                        parts = line.split(',')
                        if len(parts) == 3:
                            timestamp = int(float(parts[0]))
                            latitude = float(parts[1])
                            longitude = float(parts[2])
                            gps_data.append({
                                'timestamp': timestamp,
                                'latitude': latitude,
                                'longitude': longitude,
                                'datetime': datetime.fromtimestamp(timestamp, tz=timezone.utc).isoformat()
                            })
                    except (ValueError, IndexError):
                        continue
                        
        print(f"Loaded {len(gps_data)} GPS points")
        
        # Load cellular data (if available)
        cellular_data = []
        if cellular_file.exists():
            try:
                with open(cellular_file, 'r') as f:
                    cellular_json = json.load(f)
                    cellular_data = cellular_json.get('cellular_records', [])
                    print(f"Loaded {len(cellular_data)} cellular records")
            except (json.JSONDecodeError, FileNotFoundError):
                print("No detailed cellular data available, using simulated data")
                
        # Create enhanced correlation
        correlation_result = {
            'metadata': {
                'correlation_time': datetime.now(timezone.utc).isoformat(),
                'gps_points': len(gps_data),
                'cellular_records': len(cellular_data),
                'correlation_window_seconds': correlation_window
            },
            'enhanced_tracks': []
        }
        
        # Correlate each GPS point with available cellular data
        for gps_point in gps_data:
            enhanced_point = {
                'timestamp': gps_point['timestamp'],
                'datetime': gps_point['datetime'],
                'latitude': gps_point['latitude'],
                'longitude': gps_point['longitude'],
                'cell_towers': []
            }
            
            # Find cellular data within time window
            for cell_record in cellular_data:
                time_diff = abs(gps_point['timestamp'] - cell_record.get('timestamp', 0))
                if time_diff <= correlation_window:
                    # Create enhanced cell tower info
                    cell_tower = EnhancedCellTower()
                    
                    # Copy available data
                    for field in ['cell_id', 'physical_cell_id', 'tracking_area_code',
                                'mobile_country_code', 'mobile_network_code',
                                'radio_access_tech', 'frequency_band', 'channel_number',
                                'rsrp_dbm', 'rsrq_db', 'rssi_dbm', 'sinr_db',
                                'rscp_dbm', 'ecno_db', 'rxlev_dbm', 'neighbor_cells']:
                        if field in cell_record:
                            setattr(cell_tower, field, cell_record[field])
                            
                    # Add operator name
                    if cell_tower.mobile_country_code and cell_tower.mobile_network_code:
                        key = (cell_tower.mobile_country_code, cell_tower.mobile_network_code)
                        cell_tower.operator_name = self.operator_names.get(key, "Unknown Operator")
                        
                    # Analyze signal quality
                    cell_tower.signal_quality = self.analyze_signal_quality(cell_tower)
                    cell_tower.estimated_distance_m = self.estimate_distance(cell_tower)
                    
                    enhanced_point['cell_towers'].append(asdict(cell_tower))
                    
            # If no cellular data, create a placeholder
            if not enhanced_point['cell_towers']:
                placeholder_tower = EnhancedCellTower(
                    radio_access_tech="Unknown",
                    signal_quality="No Data Available"
                )
                enhanced_point['cell_towers'].append(asdict(placeholder_tower))
                
            correlation_result['enhanced_tracks'].append(enhanced_point)
            
        return correlation_result
        
    def export_enhanced_kml(self, correlation_data: Dict, output_file: Path) -> None:
        """Export enhanced correlation data to KML with detailed cellular info"""
        print(f"Exporting enhanced KML with detailed cellular information to {output_file}")
        
        tracks = correlation_data['enhanced_tracks']
        
        kml_content = f"""<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
  <Document>
    <name>Enhanced Cellular GPS Correlation</name>
    <description>
      <![CDATA[
        <h2>🗼 Enhanced Cellular GPS Correlation</h2>
        <p><strong>Total GPS Points:</strong> {len(tracks)}</p>
        <p><strong>Correlation Time:</strong> {correlation_data['metadata']['correlation_time']}</p>
        <p><strong>Generated by:</strong> Enhanced Cellular GPS Correlator</p>
        <p>Click on any point to see detailed cellular tower information!</p>
      ]]>
    </description>
    
    <!-- Track Line Style -->
    <Style id="trackStyle">
      <LineStyle>
        <color>ff0000ff</color>
        <width>4</width>
      </LineStyle>
    </Style>
    
    <!-- Enhanced Point Styles by Signal Quality -->
    <Style id="excellentSignal">
      <IconStyle>
        <color>ff00ff00</color>
        <scale>1.0</scale>
        <Icon><href>http://maps.google.com/mapfiles/kml/paddle/grn-circle.png</href></Icon>
      </IconStyle>
    </Style>
    
    <Style id="goodSignal">
      <IconStyle>
        <color>ff00ffff</color>
        <scale>1.0</scale>
        <Icon><href>http://maps.google.com/mapfiles/kml/paddle/ylw-circle.png</href></Icon>
      </IconStyle>
    </Style>
    
    <Style id="fairSignal">
      <IconStyle>
        <color>ff0080ff</color>
        <scale>1.0</scale>
        <Icon><href>http://maps.google.com/mapfiles/kml/paddle/orange-circle.png</href></Icon>
      </IconStyle>
    </Style>
    
    <Style id="poorSignal">
      <IconStyle>
        <color>ff0000ff</color>
        <scale>1.0</scale>
        <Icon><href>http://maps.google.com/mapfiles/kml/paddle/red-circle.png</href></Icon>
      </IconStyle>
    </Style>
    
    <Style id="noData">
      <IconStyle>
        <color>ff808080</color>
        <scale>0.8</scale>
        <Icon><href>http://maps.google.com/mapfiles/kml/paddle/wht-circle.png</href></Icon>
      </IconStyle>
    </Style>
    
    <!-- GPS Track Line -->
    <Placemark>
      <name>GPS Track</name>
      <description>Complete GPS movement track</description>
      <styleUrl>#trackStyle</styleUrl>
      <LineString>
        <tessellate>1</tessellate>
        <coordinates>"""
        
        # Add track coordinates
        for point in tracks:
            kml_content += f"{point['longitude']},{point['latitude']},0 "
            
        kml_content += """
        </coordinates>
      </LineString>
    </Placemark>
    
    <!-- Enhanced GPS Points with Cellular Data -->
    <Folder>
      <name>📡 Enhanced GPS Points with Cellular Data</name>
      <description>GPS points with detailed cellular tower information</description>"""
      
        # Add detailed points
        for i, point in enumerate(tracks[::10]):  # Every 10th point to avoid clutter
            # Determine style based on signal quality
            style = "noData"
            if point['cell_towers'] and point['cell_towers'][0].get('signal_quality'):
                quality = point['cell_towers'][0]['signal_quality']
                if quality == "Excellent":
                    style = "excellentSignal"
                elif quality == "Good":
                    style = "goodSignal"
                elif quality == "Fair":
                    style = "fairSignal"
                elif quality == "Poor":
                    style = "poorSignal"
                    
            # Create description with all cellular towers at this location
            description = f"<h3>📍 GPS Point {i+1}</h3>"
            description += f"<p><strong>Time:</strong> {point['datetime']}</p>"
            description += f"<p><strong>Coordinates:</strong> {point['latitude']:.6f}, {point['longitude']:.6f}</p>"
            
            if point['cell_towers']:
                for j, tower_data in enumerate(point['cell_towers'][:3]):  # Max 3 towers
                    tower = EnhancedCellTower(**tower_data)
                    description += f"<h4>📡 Cell Tower {j+1}</h4>"
                    description += tower.to_html_description()
            else:
                description += "<p>No cellular data available for this location.</p>"
            
            kml_content += f"""
      <Placemark>
        <name>📡 Point {i+1}</name>
        <description><![CDATA[{description}]]></description>
        <styleUrl>#{style}</styleUrl>
        <Point>
          <coordinates>{point['longitude']},{point['latitude']},0</coordinates>
        </Point>
      </Placemark>"""
      
        kml_content += """
    </Folder>
  </Document>
</kml>"""

        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(kml_content)
            
        print(f"Enhanced KML exported with {len(tracks)} GPS points and detailed cellular data")

def main():
    parser = argparse.ArgumentParser(description='Enhanced cellular GPS correlation with detailed tower information')
    parser.add_argument('--gps', required=True, help='GPS file (.gps format)')
    parser.add_argument('--cellular', help='Enhanced cellular data JSON file (optional)')
    parser.add_argument('--output', '-o', default='enhanced_correlation', help='Output file prefix')
    parser.add_argument('--format', choices=['kml', 'json', 'csv'], default='kml', help='Output format')
    
    args = parser.parse_args()
    
    gps_file = Path(args.gps)
    cellular_file = Path(args.cellular) if args.cellular else Path('detailed_cellular_data.json')
    
    if not gps_file.exists():
        print(f"Error: GPS file {gps_file} not found")
        sys.exit(1)
        
    correlator = EnhancedCellularCorrelator()
    correlation_data = correlator.correlate_with_enhanced_data(gps_file, cellular_file)
    
    if args.format == 'kml':
        output_file = Path(f"{args.output}.kml")
        correlator.export_enhanced_kml(correlation_data, output_file)
    elif args.format == 'json':
        output_file = Path(f"{args.output}.json")
        with open(output_file, 'w') as f:
            json.dump(correlation_data, f, indent=2, default=str)
        print(f"Enhanced correlation data exported to {output_file}")
    
    print(f"\nEnhanced correlation complete!")
    print(f"Generated detailed cellular correlation with {len(correlation_data['enhanced_tracks'])} GPS points")

if __name__ == "__main__":
    main()
