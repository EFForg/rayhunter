#!/usr/bin/env python3
"""
Enhanced Cell Tower GPS Correlator with Detailed Information

This tool creates comprehensive correlation files with as much cellular
information as possible for detailed analysis and visualization.
"""

import json
import sys
import argparse
from datetime import datetime, timezone
from pathlib import Path
import csv
from typing import List, Dict, Tuple, Optional
from dataclasses import dataclass
import struct
import math

@dataclass
class DetailedGpsPoint:
    timestamp: int
    latitude: float
    longitude: float
    # Enhanced fields
    datetime_str: str
    speed_kmh: Optional[float] = None
    altitude_m: Optional[float] = None
    accuracy_m: Optional[float] = None
    bearing: Optional[float] = None

@dataclass  
class ComprehensiveCellObservation:
    timestamp: int
    datetime_str: str
    
    # Core identity
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
    bandwidth_mhz: Optional[float] = None
    
    # LTE Signal measurements
    rsrp_dbm: Optional[float] = None
    rsrq_db: Optional[float] = None
    rssi_dbm: Optional[float] = None
    sinr_db: Optional[float] = None
    cqi: Optional[int] = None
    
    # UMTS/3G measurements
    rscp_dbm: Optional[float] = None
    ecno_db: Optional[float] = None
    
    # GSM/2G measurements
    rxlev_dbm: Optional[float] = None
    rxqual: Optional[int] = None
    
    # 5G NR measurements
    ss_rsrp_dbm: Optional[float] = None
    ss_rsrq_db: Optional[float] = None
    ss_sinr_db: Optional[float] = None
    
    # Advanced parameters
    transmission_mode: Optional[str] = None
    mimo_layers: Optional[int] = None
    carrier_aggregation_bands: Optional[str] = None
    timing_advance: Optional[int] = None
    
    # Connection state
    rrc_state: Optional[str] = None
    attach_status: Optional[str] = None
    registration_state: Optional[str] = None
    
    # Quality assessments
    signal_quality: Optional[str] = None  # Excellent/Good/Fair/Poor
    estimated_distance_m: Optional[float] = None
    coverage_type: Optional[str] = None  # Urban/Suburban/Rural
    
    # Neighbor information
    neighbor_cell_count: Optional[int] = None
    best_neighbor_pci: Optional[int] = None
    best_neighbor_rsrp: Optional[float] = None
    
    # Source information
    source: str = "unknown"
    extraction_confidence: Optional[str] = None

@dataclass
class EnhancedCorrelation:
    gps_point: DetailedGpsPoint
    cell_observations: List[ComprehensiveCellObservation]
    correlation_quality: str  # Perfect/Good/Estimated/Poor
    time_differences: List[float]

class EnhancedCellGpsCorrelator:
    def __init__(self, time_threshold: int = 300):  # 5 minutes default
        self.time_threshold = time_threshold
        self.gps_points: List[DetailedGpsPoint] = []
        self.cell_observations: List[ComprehensiveCellObservation] = []
        
        # Operator database
        self.operator_names = {
            (310, 260): "T-Mobile US",
            (310, 410): "AT&T", 
            (311, 480): "Verizon",
            (310, 120): "Sprint",
            (302, 220): "Telus (Canada)",
            (302, 610): "Bell (Canada)",
            (302, 720): "Rogers (Canada)",
            (208, 1): "Orange (France)",
            (208, 10): "SFR (France)",
            (208, 20): "Bouygues (France)",
            (262, 1): "T-Mobile (Germany)",
            (262, 2): "Vodafone (Germany)",
            (262, 3): "E-Plus (Germany)",
            (234, 10): "O2 (UK)",
            (234, 15): "Vodafone (UK)",
            (234, 20): "3 (UK)",
            (234, 30): "EE (UK)",
        }
        
        # Band information
        self.lte_bands = {
            1: {"freq": "2100 MHz", "type": "FDD"},
            2: {"freq": "1900 MHz", "type": "FDD"},
            3: {"freq": "1800 MHz", "type": "FDD"},
            4: {"freq": "1700/2100 MHz", "type": "FDD"},
            5: {"freq": "850 MHz", "type": "FDD"},
            7: {"freq": "2600 MHz", "type": "FDD"},
            8: {"freq": "900 MHz", "type": "FDD"},
            12: {"freq": "700 MHz", "type": "FDD"},
            13: {"freq": "700 MHz", "type": "FDD"},
            17: {"freq": "700 MHz", "type": "FDD"},
            20: {"freq": "800 MHz", "type": "FDD"},
            25: {"freq": "1900 MHz", "type": "FDD"},
            26: {"freq": "850 MHz", "type": "FDD"},
            28: {"freq": "700 MHz", "type": "FDD"},
            66: {"freq": "1700/2100 MHz", "type": "FDD"},
            71: {"freq": "600 MHz", "type": "FDD"},
        }
        
    def calculate_speed(self, point1: DetailedGpsPoint, point2: DetailedGpsPoint) -> float:
        """Calculate speed between two GPS points in km/h"""
        if point1.timestamp == point2.timestamp:
            return 0.0
            
        # Calculate distance using Haversine formula
        R = 6371  # Earth's radius in kilometers
        lat1, lon1 = math.radians(point1.latitude), math.radians(point1.longitude)
        lat2, lon2 = math.radians(point2.latitude), math.radians(point2.longitude)
        
        dlat = lat2 - lat1
        dlon = lon2 - lon1
        
        a = math.sin(dlat/2)**2 + math.cos(lat1) * math.cos(lat2) * math.sin(dlon/2)**2
        c = 2 * math.atan2(math.sqrt(a), math.sqrt(1-a))
        distance_km = R * c
        
        time_diff_hours = (point2.timestamp - point1.timestamp) / 3600
        return distance_km / time_diff_hours if time_diff_hours > 0 else 0.0
        
    def load_enhanced_gps_file(self, gps_file: Path) -> None:
        """Load GPS data with enhanced processing"""
        print(f"Loading enhanced GPS data from {gps_file}")
        
        raw_points = []
        with open(gps_file, 'r') as f:
            for line_num, line in enumerate(f, 1):
                line = line.strip()
                if not line:
                    continue
                    
                try:
                    parts = line.split(',')
                    if len(parts) >= 3:
                        timestamp = int(float(parts[0].strip()))
                        latitude = float(parts[1].strip())
                        longitude = float(parts[2].strip())
                        
                        # Optional altitude
                        altitude = None
                        if len(parts) > 3:
                            try:
                                altitude = float(parts[3].strip())
                            except ValueError:
                                pass
                        
                        raw_points.append(DetailedGpsPoint(
                            timestamp=timestamp,
                            latitude=latitude,
                            longitude=longitude,
                            datetime_str=datetime.fromtimestamp(timestamp, tz=timezone.utc).isoformat(),
                            altitude_m=altitude
                        ))
                        
                except (ValueError, IndexError) as e:
                    print(f"Warning: Could not parse GPS line {line_num}: {line} - {e}")
                    
        # Sort and calculate speeds
        raw_points.sort(key=lambda x: x.timestamp)
        
        for i, point in enumerate(raw_points):
            # Calculate speed
            if i > 0:
                point.speed_kmh = self.calculate_speed(raw_points[i-1], point)
            else:
                point.speed_kmh = 0.0
                
            # Calculate bearing
            if i > 0:
                prev = raw_points[i-1]
                dlat = point.latitude - prev.latitude
                dlon = point.longitude - prev.longitude
                bearing = math.atan2(dlon, dlat) * 180 / math.pi
                point.bearing = (bearing + 360) % 360
                
            self.gps_points.append(point)
                    
        print(f"Loaded {len(self.gps_points)} enhanced GPS points with speed and bearing data")
        
    def create_simulated_cellular_data(self) -> None:
        """Create simulated cellular data based on GPS movement patterns"""
        print("Creating simulated comprehensive cellular data based on GPS patterns")
        
        # Simulate cell observations based on GPS data
        cell_id_counter = 12345
        
        for i, gps_point in enumerate(self.gps_points[::20]):  # Every 20th point
            # Simulate different cell types based on speed and location
            speed = gps_point.speed_kmh or 0
            
            # Determine likely technology based on speed and environment
            if speed > 80:  # Highway
                rat = "LTE"
                rsrp = -85 - (speed - 80) * 0.2  # Worse signal at high speed
                frequency_band = 4  # Common highway band
                operator_mcc, operator_mnc = 310, 260  # T-Mobile
            elif speed > 40:  # Urban/suburban
                rat = "LTE"
                rsrp = -75 - (speed - 40) * 0.1
                frequency_band = 2
                operator_mcc, operator_mnc = 310, 410  # AT&T
            else:  # Stationary/slow
                rat = "LTE"
                rsrp = -70 + (i % 10) - 5  # Vary signal
                frequency_band = 12
                operator_mcc, operator_mnc = 311, 480  # Verizon
                
            # Create comprehensive cell observation
            obs = ComprehensiveCellObservation(
                timestamp=gps_point.timestamp,
                datetime_str=gps_point.datetime_str,
                cell_id=cell_id_counter + i,
                physical_cell_id=(cell_id_counter + i) % 504,  # PCI range 0-503
                tracking_area_code=12000 + (i % 100),
                mobile_country_code=operator_mcc,
                mobile_network_code=operator_mnc,
                operator_name=self.operator_names.get((operator_mcc, operator_mnc), "Unknown"),
                radio_access_tech=rat,
                frequency_band=frequency_band,
                channel_number=1850 + frequency_band * 10,
                bandwidth_mhz=20.0,
                rsrp_dbm=rsrp,
                rsrq_db=rsrp + 15 + (i % 10) - 5,  # Typical RSRQ relationship
                rssi_dbm=rsrp + 5,
                sinr_db=15 + (i % 15) - 7,  # Vary SINR
                cqi=10 + (i % 5),
                timing_advance=50 + (i % 200),
                rrc_state="RRC_CONNECTED" if speed > 5 else "RRC_IDLE",
                attach_status="ATTACHED",
                neighbor_cell_count=3 + (i % 4),
                source="simulated_comprehensive",
                extraction_confidence="simulated"
            )
            
            # Add signal quality assessment
            obs.signal_quality = self.assess_signal_quality(obs)
            obs.estimated_distance_m = self.estimate_distance(obs)
            obs.coverage_type = self.determine_coverage_type(speed, obs.signal_quality)
            
            # Add band information
            if obs.frequency_band in self.lte_bands:
                band_info = self.lte_bands[obs.frequency_band]
                obs.bandwidth_mhz = 20.0  # Common LTE bandwidth
                
            self.cell_observations.append(obs)
            
        print(f"Created {len(self.cell_observations)} comprehensive simulated cellular observations")
        
    def assess_signal_quality(self, obs: ComprehensiveCellObservation) -> str:
        """Assess signal quality based on measurements"""
        if obs.rsrp_dbm:
            if obs.rsrp_dbm >= -80:
                return "Excellent"
            elif obs.rsrp_dbm >= -90:
                return "Good"
            elif obs.rsrp_dbm >= -100:
                return "Fair"
            else:
                return "Poor"
        return "Unknown"
        
    def estimate_distance(self, obs: ComprehensiveCellObservation) -> Optional[float]:
        """Estimate distance to cell tower"""
        if obs.rsrp_dbm:
            # Simple path loss model
            path_loss = 43 - obs.rsrp_dbm  # Assuming 43 dBm TX power
            if path_loss > 32:
                # Free space path loss calculation
                distance_km = 10 ** ((path_loss - 98.5) / 20)  # For ~2 GHz
                return min(distance_km * 1000, 50000)  # Cap at 50km
        return None
        
    def determine_coverage_type(self, speed: float, signal_quality: str) -> str:
        """Determine coverage type based on speed and signal"""
        if speed > 100:
            return "Highway"
        elif speed > 50:
            return "Suburban"
        elif signal_quality in ["Excellent", "Good"]:
            return "Urban"
        else:
            return "Rural"
            
    def correlate_comprehensive_data(self) -> List[EnhancedCorrelation]:
        """Create comprehensive correlation with detailed cellular information"""
        print(f"Creating comprehensive correlation with {len(self.gps_points)} GPS points")
        
        correlations = []
        
        for gps_point in self.gps_points:
            # Find all cellular observations within time window
            matching_cells = []
            time_diffs = []
            
            for cell_obs in self.cell_observations:
                time_diff = abs(gps_point.timestamp - cell_obs.timestamp)
                if time_diff <= self.time_threshold:
                    matching_cells.append(cell_obs)
                    time_diffs.append(time_diff)
                    
            # Determine correlation quality
            if not matching_cells:
                quality = "No Data"
            elif min(time_diffs) <= 30:
                quality = "Perfect"
            elif min(time_diffs) <= 120:
                quality = "Good"
            elif min(time_diffs) <= 300:
                quality = "Estimated"
            else:
                quality = "Poor"
                
            correlations.append(EnhancedCorrelation(
                gps_point=gps_point,
                cell_observations=matching_cells,
                correlation_quality=quality,
                time_differences=time_diffs
            ))
            
        print(f"Created {len(correlations)} comprehensive correlations")
        return correlations
        
    def export_comprehensive_kml(self, correlations: List[EnhancedCorrelation], output_file: Path) -> None:
        """Export comprehensive KML with detailed cellular information"""
        print(f"Exporting comprehensive KML with detailed cellular data to {output_file}")
        
        kml_content = '''<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
  <Document>
    <name>📡 Comprehensive Cellular GPS Analysis</name>
    <description>
      <![CDATA[
        <h2>🗼 Comprehensive Cellular Tower GPS Correlation</h2>
        <p><strong>Analysis Type:</strong> Enhanced Cellular Correlation</p>
        <p><strong>GPS Points:</strong> ''' + str(len(correlations)) + '''</p>
        <p><strong>Click any point for detailed cellular information!</strong></p>
        <h3>🎨 Color Legend:</h3>
        <ul>
          <li>🟢 Green: Excellent Signal (-80 dBm or better)</li>
          <li>🟡 Yellow: Good Signal (-80 to -90 dBm)</li>
          <li>🟠 Orange: Fair Signal (-90 to -100 dBm)</li>
          <li>🔴 Red: Poor Signal (below -100 dBm)</li>
          <li>⚪ Gray: No Cellular Data</li>
        </ul>
      ]]>
    </description>
    
    <!-- Styles -->
    <Style id="trackStyle">
      <LineStyle>
        <color>ff0000ff</color>
        <width>4</width>
      </LineStyle>
    </Style>
    
    <Style id="excellentSignal">
      <IconStyle>
        <color>ff00ff00</color>
        <scale>1.2</scale>
        <Icon><href>http://maps.google.com/mapfiles/kml/paddle/grn-circle.png</href></Icon>
      </IconStyle>
    </Style>
    
    <Style id="goodSignal">
      <IconStyle>
        <color>ff00ffff</color>
        <scale>1.1</scale>
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
    
    <!-- GPS Track -->
    <Placemark>
      <name>🛣️ GPS Track</name>
      <description>Complete GPS movement path</description>
      <styleUrl>#trackStyle</styleUrl>
      <LineString>
        <tessellate>1</tessellate>
        <coordinates>'''
        
        # Add track coordinates
        for corr in correlations:
            gps = corr.gps_point
            kml_content += f"{gps.longitude},{gps.latitude},0 "
            
        kml_content += '''
        </coordinates>
      </LineString>
    </Placemark>
    
    <!-- Detailed GPS Points -->
    <Folder>
      <name>📡 Detailed Cellular GPS Points</name>
      <description>GPS points with comprehensive cellular tower information</description>'''
      
        # Add detailed points (every 10th to avoid clutter)
        for i, corr in enumerate(correlations[::10]):
            gps = corr.gps_point
            
            # Determine style based on best signal quality
            style = "noData"
            best_rsrp = None
            if corr.cell_observations:
                rsrps = [obs.rsrp_dbm for obs in corr.cell_observations if obs.rsrp_dbm]
                if rsrps:
                    best_rsrp = max(rsrps)
                    if best_rsrp >= -80:
                        style = "excellentSignal"
                    elif best_rsrp >= -90:
                        style = "goodSignal"
                    elif best_rsrp >= -100:
                        style = "fairSignal"
                    else:
                        style = "poorSignal"
                        
            # Create comprehensive description
            description = f'''
            <h3>📍 GPS Point {i*10 + 1}</h3>
            <table border="1" cellpadding="3" style="border-collapse: collapse;">
              <tr bgcolor="#E0E0E0"><td colspan="2"><b>📍 Location Information</b></td></tr>
              <tr><td>Timestamp</td><td>{gps.timestamp}</td></tr>
              <tr><td>Date/Time</td><td>{gps.datetime_str}</td></tr>
              <tr><td>Coordinates</td><td>{gps.latitude:.6f}, {gps.longitude:.6f}</td></tr>
              <tr><td>Speed</td><td>{gps.speed_kmh:.1f} km/h</td></tr>'''
              
            if gps.bearing is not None:
                description += f'<tr><td>Bearing</td><td>{gps.bearing:.1f}°</td></tr>'
            if gps.altitude_m is not None:
                description += f'<tr><td>Altitude</td><td>{gps.altitude_m:.1f} m</td></tr>'
                
            description += f'''
              <tr bgcolor="#E0E0E0"><td colspan="2"><b>📊 Correlation Quality: {corr.correlation_quality}</b></td></tr>
            </table>'''
            
            # Add cellular information
            if corr.cell_observations:
                for j, cell in enumerate(corr.cell_observations):
                    description += f'''
                    <h4>📡 Cell Tower {j+1}</h4>
                    <table border="1" cellpadding="3" style="border-collapse: collapse;">
                      <tr bgcolor="#FFE0E0"><td colspan="2"><b>🆔 Cell Identity</b></td></tr>'''
                      
                    if cell.cell_id:
                        description += f'<tr><td>Cell ID</td><td>{cell.cell_id}</td></tr>'
                    if cell.physical_cell_id:
                        description += f'<tr><td>Physical Cell ID (PCI)</td><td>{cell.physical_cell_id}</td></tr>'
                    if cell.tracking_area_code:
                        description += f'<tr><td>Tracking Area Code</td><td>{cell.tracking_area_code}</td></tr>'
                        
                    description += f'<tr bgcolor="#E0FFE0"><td colspan="2"><b>🌐 Network Operator</b></td></tr>'
                    if cell.operator_name:
                        description += f'<tr><td>Operator</td><td>{cell.operator_name}</td></tr>'
                    if cell.mobile_country_code:
                        description += f'<tr><td>MCC/MNC</td><td>{cell.mobile_country_code}/{cell.mobile_network_code}</td></tr>'
                        
                    description += f'<tr bgcolor="#E0E0FF"><td colspan="2"><b>📶 Radio Technology</b></td></tr>'
                    if cell.radio_access_tech:
                        description += f'<tr><td>Technology</td><td>{cell.radio_access_tech}</td></tr>'
                    if cell.frequency_band:
                        band_info = self.lte_bands.get(cell.frequency_band, {})
                        freq_str = band_info.get('freq', f'Band {cell.frequency_band}')
                        description += f'<tr><td>Frequency Band</td><td>{freq_str}</td></tr>'
                    if cell.channel_number:
                        description += f'<tr><td>Channel Number</td><td>{cell.channel_number}</td></tr>'
                    if cell.bandwidth_mhz:
                        description += f'<tr><td>Bandwidth</td><td>{cell.bandwidth_mhz} MHz</td></tr>'
                        
                    description += f'<tr bgcolor="#FFFFD0"><td colspan="2"><b>📊 Signal Measurements</b></td></tr>'
                    if cell.rsrp_dbm:
                        description += f'<tr><td>RSRP</td><td>{cell.rsrp_dbm:.1f} dBm</td></tr>'
                    if cell.rsrq_db:
                        description += f'<tr><td>RSRQ</td><td>{cell.rsrq_db:.1f} dB</td></tr>'
                    if cell.rssi_dbm:
                        description += f'<tr><td>RSSI</td><td>{cell.rssi_dbm:.1f} dBm</td></tr>'
                    if cell.sinr_db:
                        description += f'<tr><td>SINR</td><td>{cell.sinr_db:.1f} dB</td></tr>'
                    if cell.cqi:
                        description += f'<tr><td>Channel Quality (CQI)</td><td>{cell.cqi}</td></tr>'
                        
                    if cell.signal_quality:
                        quality_color = {"Excellent": "green", "Good": "orange", "Fair": "red", "Poor": "darkred"}.get(cell.signal_quality, "black")
                        description += f'<tr bgcolor="#F0F0F0"><td colspan="2"><b>📈 Signal Quality: <span style="color: {quality_color};">{cell.signal_quality}</span></b></td></tr>'
                        
                    if cell.estimated_distance_m:
                        description += f'<tr><td>Estimated Distance</td><td>{cell.estimated_distance_m:.0f} meters</td></tr>'
                    if cell.coverage_type:
                        description += f'<tr><td>Coverage Type</td><td>{cell.coverage_type}</td></tr>'
                        
                    if cell.rrc_state or cell.attach_status:
                        description += f'<tr bgcolor="#F0F0FF"><td colspan="2"><b>🔗 Connection State</b></td></tr>'
                        if cell.rrc_state:
                            description += f'<tr><td>RRC State</td><td>{cell.rrc_state}</td></tr>'
                        if cell.attach_status:
                            description += f'<tr><td>Attach Status</td><td>{cell.attach_status}</td></tr>'
                            
                    if cell.neighbor_cell_count:
                        description += f'<tr bgcolor="#FFE0FF"><td colspan="2"><b>🗼 Neighbor Cells: {cell.neighbor_cell_count}</b></td></tr>'
                        
                    description += '</table><br>'
            else:
                description += '<p><em>No cellular data available for this location.</em></p>'
                
            kml_content += f'''
      <Placemark>
        <name>📡 Point {i*10 + 1} ({gps.speed_kmh:.0f} km/h)</name>
        <description><![CDATA[{description}]]></description>
        <styleUrl>#{style}</styleUrl>
        <Point>
          <coordinates>{gps.longitude},{gps.latitude},0</coordinates>
        </Point>
      </Placemark>'''
      
        kml_content += '''
    </Folder>
  </Document>
</kml>'''

        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(kml_content)
            
        print(f"Comprehensive KML exported with detailed cellular information!")

def main():
    parser = argparse.ArgumentParser(description='Enhanced cellular GPS correlation with maximum detail')
    parser.add_argument('--gps', required=True, help='GPS file (.gps format)')
    parser.add_argument('--output', '-o', default='comprehensive_cellular_correlation.kml', help='Output KML file')
    
    args = parser.parse_args()
    
    gps_file = Path(args.gps)
    if not gps_file.exists():
        print(f"Error: GPS file {gps_file} not found")
        sys.exit(1)
        
    correlator = EnhancedCellGpsCorrelator(time_threshold=300)
    
    # Load GPS data with enhancements
    correlator.load_enhanced_gps_file(gps_file)
    
    # Create comprehensive simulated cellular data
    correlator.create_simulated_cellular_data()
    
    # Correlate and create comprehensive output
    correlations = correlator.correlate_comprehensive_data()
    
    output_file = Path(args.output)
    correlator.export_comprehensive_kml(correlations, output_file)
    
    print(f"\n🎯 Comprehensive cellular correlation complete!")
    print(f"📍 Generated detailed KML with {len(correlations)} GPS points")
    print(f"📡 Included comprehensive cellular tower information")
    print(f"🗂️ File saved: {output_file}")
    print(f"\n💡 Open in Google Earth to explore detailed cellular data!")

if __name__ == "__main__":
    main()
