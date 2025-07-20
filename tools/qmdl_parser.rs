#!/usr/bin/env cargo
//! QMDL Parser Tool for Cellular Information Extraction
//! 
//! This tool parses QMDL files and extracts cellular network information
//! including MCC, MNC, Cell ID, LAC, TAC, and other parameters needed
//! for OpenCellID lookups.

use std::env;
use std::fs::File;
use std::path::Path;
use tokio::io::BufReader;
use futures::TryStreamExt;
use serde_json;
use chrono::{DateTime, Utc};

// Add the rayhunter lib path
use rayhunter_lib::qmdl::QmdlReader;
use rayhunter_lib::gsmtap_parser::parse_with_cellular_info;
use rayhunter_lib::cellular_info::CellularNetworkInfo;
use rayhunter_lib::diag::{MessagesContainer, parse_diag_log_message};

#[derive(Debug, serde::Serialize)]
struct ParsedCellInfo {
    timestamp: String,
    cellular_info: Vec<CellularNetworkInfo>,
    total_messages: usize,
    cellular_messages: usize,
}

#[derive(Debug, serde::Serialize)]
struct OpenCellIdCompatibleInfo {
    mcc: Option<u16>,
    mnc: Option<u16>,
    lac: Option<u16>,
    cell_id: Option<u32>,
    tac: Option<u16>,
    pci: Option<u16>,
    earfcn: Option<u32>,
    rat: String,
    timestamp: String,
    signal_strength: Option<i16>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <qmdl_file>", args[0]);
        std::process::exit(1);
    }

    let qmdl_path = &args[1];
    
    if !Path::new(qmdl_path).exists() {
        eprintln!("Error: File {} does not exist", qmdl_path);
        std::process::exit(1);
    }

    println!("Parsing QMDL file: {}", qmdl_path);
    
    let file = File::open(qmdl_path)?;
    let file_size = file.metadata()?.len();
    println!("File size: {} bytes", file_size);
    
    let buf_reader = BufReader::new(file);
    let mut qmdl_reader = QmdlReader::new(buf_reader, Some(file_size as usize));
    
    let mut cellular_info_list = Vec::new();
    let mut total_messages = 0;
    let mut cellular_messages = 0;
    let mut opencellid_compatible = Vec::new();
    
    // Parse all message containers
    let mut stream = qmdl_reader.as_stream();
    while let Some(container) = stream.try_next().await? {
        total_messages += container.messages.len();
        
        for message in &container.messages {
            // Try to parse as diagnostic message
            if let Ok(diag_msg) = parse_diag_log_message(&message.data) {
                // Try to extract cellular information
                if let Some(cellular_info) = parse_with_cellular_info(&diag_msg, Utc::now().into()) {
                    cellular_info_list.push(cellular_info.clone());
                    cellular_messages += 1;
                    
                    // Extract OpenCellID compatible information
                    let compatible_info = OpenCellIdCompatibleInfo {
                        mcc: cellular_info.plmn_info.as_ref().and_then(|p| p.mcc),
                        mnc: cellular_info.plmn_info.as_ref().and_then(|p| p.mnc),
                        lac: cellular_info.location_info.as_ref().and_then(|l| l.lac),
                        cell_id: cellular_info.cell_info.as_ref().and_then(|c| c.global_cell_id),
                        tac: cellular_info.location_info.as_ref().and_then(|l| l.tac),
                        pci: cellular_info.cell_info.as_ref().and_then(|c| c.physical_cell_id),
                        earfcn: None, // Would need to extract from frequency info
                        rat: format!("{:?}", cellular_info.rat),
                        timestamp: cellular_info.timestamp.to_rfc3339(),
                        signal_strength: cellular_info.signal_info.as_ref().and_then(|s| s.rsrp),
                    };
                    opencellid_compatible.push(compatible_info);
                }
            }
        }
    }
    
    let parsed_info = ParsedCellInfo {
        timestamp: Utc::now().to_rfc3339(),
        cellular_info: cellular_info_list,
        total_messages,
        cellular_messages,
    };
    
    // Output text summary
    println!("\n=== QMDL PARSE SUMMARY ===");
    println!("Total messages processed: {}", total_messages);
    println!("Messages with cellular info: {}", cellular_messages);
    println!("Percentage with cellular data: {:.2}%", 
             (cellular_messages as f64 / total_messages as f64) * 100.0);
    
    // Check for OpenCellID required fields
    let mut has_mcc = 0;
    let mut has_mnc = 0;
    let mut has_cell_id = 0;
    let mut has_lac_or_tac = 0;
    
    for info in &opencellid_compatible {
        if info.mcc.is_some() { has_mcc += 1; }
        if info.mnc.is_some() { has_mnc += 1; }
        if info.cell_id.is_some() { has_cell_id += 1; }
        if info.lac.is_some() || info.tac.is_some() { has_lac_or_tac += 1; }
    }
    
    println!("\n=== OPENCELLID COMPATIBILITY ===");
    println!("Records with MCC: {} ({:.1}%)", has_mcc, 
             (has_mcc as f64 / cellular_messages as f64) * 100.0);
    println!("Records with MNC: {} ({:.1}%)", has_mnc, 
             (has_mnc as f64 / cellular_messages as f64) * 100.0);
    println!("Records with Cell ID: {} ({:.1}%)", has_cell_id, 
             (has_cell_id as f64 / cellular_messages as f64) * 100.0);
    println!("Records with LAC/TAC: {} ({:.1}%)", has_lac_or_tac, 
             (has_lac_or_tac as f64 / cellular_messages as f64) * 100.0);
    
    // Show sample of extracted data
    if !opencellid_compatible.is_empty() {
        println!("\n=== SAMPLE EXTRACTED DATA ===");
        for (i, info) in opencellid_compatible.iter().take(5).enumerate() {
            println!("Sample {}: MCC={:?}, MNC={:?}, CellID={:?}, LAC={:?}, TAC={:?}, RAT={}", 
                     i+1, info.mcc, info.mnc, info.cell_id, info.lac, info.tac, info.rat);
        }
    }
    
    // Write JSON output
    let json_output = serde_json::to_string_pretty(&parsed_info)?;
    let json_file = format!("{}.json", qmdl_path);
    std::fs::write(&json_file, json_output)?;
    println!("\nJSON output written to: {}", json_file);
    
    // Write OpenCellID compatible CSV
    let csv_file = format!("{}.opencellid.csv", qmdl_path);
    let mut csv_content = String::from("timestamp,rat,mcc,mnc,lac,tac,cell_id,pci,earfcn,rsrp\n");
    for info in &opencellid_compatible {
        csv_content.push_str(&format!("{},{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}\n",
                                      info.timestamp, info.rat, info.mcc, info.mnc, 
                                      info.lac, info.tac, info.cell_id, info.pci, 
                                      info.earfcn, info.signal_strength));
    }
    std::fs::write(&csv_file, csv_content)?;
    println!("OpenCellID CSV written to: {}", csv_file);
    
    Ok(())
}
