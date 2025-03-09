// src/views/recordings.rs
use crate::api::{AnalysisStatus, ApiClient, ManifestEntry, ManifestStats};
use crate::theme::{RayhunterTheme, Container, Button};
use iced::{
    widget::{button, column, container, row, scrollable, text, Column,},
    Command, Element, Length, Color,
};
use log;
use std::collections::HashMap;
use std::process::Command as ProcessCommand;

// Helper function to open URLs in the default browser
fn open_url(url: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        ProcessCommand::new("cmd")
            .args(["/c", "start", url])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        ProcessCommand::new("open")
            .arg(url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        ProcessCommand::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectRecording(String),
    AnalyzeRecording(String),
    DownloadPcap(String),
    DownloadQmdl(String),
    ViewAnalysisReport(String),
    RefreshList,
    RecordingAnalysisReport(Result<String, String>),
}

pub struct RecordingsView {
    theme: RayhunterTheme,
    recordings: Vec<ManifestEntry>,
    current_recording: Option<ManifestEntry>,
    selected_recording: Option<String>,
    analysis_status: AnalysisStatus,
    analysis_report: Option<String>,
    recording_warnings: HashMap<String, usize>,
}

impl RecordingsView {
    pub fn new(api_client: &ApiClient, theme: &RayhunterTheme) -> (Self, Command<Message>) {
        let recordings_view = Self {
            theme: theme.clone(),
            recordings: Vec::new(),
            current_recording: None,
            selected_recording: None,
            analysis_status: AnalysisStatus::default(),
            analysis_report: None,
            recording_warnings: HashMap::new(),
        };

        let cmd = api_client.get_qmdl_manifest().map(|result| match result {
            Ok(_) => Message::RefreshList,
            Err(_) => Message::RefreshList,
        });

        (recordings_view, cmd)
    }

    pub fn update(&mut self, message: Message, api_client: &ApiClient) -> Command<Message> {
        match message {
            Message::SelectRecording(name) => {
                self.selected_recording = Some(name.clone());
                api_client.get_analysis_report(&name).map(Message::RecordingAnalysisReport)
            }
            Message::AnalyzeRecording(name) => {
                api_client.start_analysis(&name).map(|_| Message::RefreshList)
            }
            Message::DownloadPcap(name) => {
                // Open URL in browser
                let url = api_client.get_pcap_url(&name);
                if let Err(e) = open_url(&url) {
                    log::error!("Failed to open PCAP URL: {}", e);
                }
                Command::none()
            }
            Message::DownloadQmdl(name) => {
                // Open URL in browser
                let url = api_client.get_qmdl_url(&name);
                if let Err(e) = open_url(&url) {
                    log::error!("Failed to open QMDL URL: {}", e);
                }
                Command::none()
            }
            Message::ViewAnalysisReport(name) => {
                self.selected_recording = Some(name.clone());
                api_client.get_analysis_report(&name).map(Message::RecordingAnalysisReport)
            }
            Message::RefreshList => {
                Command::batch(vec![
                    api_client.get_qmdl_manifest().map(|_| Message::RefreshList),
                    api_client.get_analysis_status().map(|_| Message::RefreshList),
                ])
            }
            Message::RecordingAnalysisReport(result) => {
                match result {
                    Ok(report) => {
                        self.analysis_report = Some(report);
                    }
                    Err(e) => {
                        log::error!("Error fetching analysis report: {}", e);
                    }
                }
                Command::none()
            }
        }
    }

    pub fn update_qmdl_manifest(&mut self, manifest: &ManifestStats) {
        self.recordings = manifest.entries.clone();
        self.current_recording = manifest.current_entry.clone();
        
        // Update selected recording if needed
        if self.selected_recording.is_none() && !self.recordings.is_empty() {
            self.selected_recording = Some(self.recordings[0].name.clone());
        }
    }
    
    pub fn update_analysis_status(&mut self, status: &AnalysisStatus) {
        self.analysis_status = status.clone();
    }
    
    pub fn view(&self) -> Element<Message> {
        let title = text("Recordings")
            .size(32)
            .style(self.theme.accent_color());
            
        let refresh_button = button(text("Refresh").style(self.theme.text_color()))
            .on_press(Message::RefreshList)
            .padding([5, 10])
            .style(Button::Secondary);
            
        // Create the list of recordings
        let mut recordings_list = Column::new().spacing(5);
        
        // Add column headers
        let headers = row![
            text("Name").width(Length::FillPortion(2)).style(self.theme.text_color()),
            text("Date").width(Length::FillPortion(3)).style(self.theme.text_color()),
            text("Size").width(Length::FillPortion(1)).style(self.theme.text_color()),
            text("Actions").width(Length::FillPortion(3)).style(self.theme.text_color()),
        ]
        .spacing(10)
        .padding(5);
        
        recordings_list = recordings_list.push(
            container(headers)
                .style(Container::Section)
        );
        
        // Add recording entries
        for entry in &self.recordings {
            let is_selected = self.selected_recording.as_ref().map_or(false, |s| s == &entry.name);
            let is_being_analyzed = self.analysis_status.running.as_ref().map_or(false, |r| r == &entry.name)
                || self.analysis_status.queued.contains(&entry.name);
                
            let time_str = entry.start_time.format("%Y-%m-%d %H:%M:%S").to_string();
            let size_str = bytesize::to_string(entry.qmdl_size_bytes as u64, true);
            
            let analyze_btn = button(
                text("Analyze").style(if is_being_analyzed {
                    self.theme.text_color()
                } else {
                    Color::WHITE
                })
            )
            .on_press(Message::AnalyzeRecording(entry.name.clone()))
            .style(if is_being_analyzed { Button::Secondary } else { Button::Primary });
                
            let view_btn = button(text("View").style(self.theme.text_color()))
                .on_press(Message::ViewAnalysisReport(entry.name.clone()))
                .style(Button::Secondary);
                
            let download_pcap = button(text("PCAP").style(self.theme.text_color()))
                .on_press(Message::DownloadPcap(entry.name.clone()))
                .style(Button::Secondary);
                
            let download_qmdl = button(text("QMDL").style(self.theme.text_color()))
                .on_press(Message::DownloadQmdl(entry.name.clone()))
                .style(Button::Secondary);
                
            let buttons = row![analyze_btn, view_btn, download_pcap, download_qmdl].spacing(5);
            
            // For selected recordings:
            // 1. Name text changes color (via theme)
            // 2. Left padding is increased to create an indentation effect
            let name_text = text(&entry.name).style(
                if is_selected {
                    self.theme.selected_text_color()
                } else {
                    self.theme.text_color()
                }
            );
            
            let row_content = row![
                name_text.width(Length::FillPortion(2)),
                text(time_str).width(Length::FillPortion(3)).style(self.theme.text_color()),
                text(size_str).width(Length::FillPortion(1)).style(self.theme.text_color()),
                buttons.width(Length::FillPortion(3)),
            ]
            .spacing(10)
            .padding(5);
            
            // Wrap in a container with styles
            let row_container = container(row_content)
                .width(Length::Fill)
                .style(if is_selected { Container::Card } else { Container::Section });
            
            recordings_list = recordings_list.push(row_container);
        }
        
        let recordings_scrollable = scrollable(
            container(recordings_list)
                .width(Length::Fill)
                .padding(10)
                .style(Container::Card)
        )
        .height(Length::FillPortion(2));
        
        // Analysis report section
        let analysis_section = if let Some(report) = &self.analysis_report {
            column![
                text("Analysis Report").size(20).style(self.theme.accent_color()),
                scrollable(
                    container(text(report).size(14).style(self.theme.text_color()))
                        .width(Length::Fill)
                        .padding(10)
                        .style(Container::Card)
                )
                .height(Length::FillPortion(2))
            ]
        } else if let Some(selected) = &self.selected_recording {
            column![
                text("Analysis Report").size(20).style(self.theme.accent_color()),
                text(format!("Select 'View' on recording '{}' to see its analysis", selected))
                    .style(self.theme.text_color())
            ]
        } else {
            column![
                text("Analysis Report").size(20).style(self.theme.accent_color()),
                text("Select a recording to view its analysis").style(self.theme.text_color())
            ]
        };
        
        container(
            column![
                row![title, refresh_button.width(Length::Shrink)],
                recordings_scrollable,
                analysis_section,
            ]
            .spacing(20)
            .padding(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(Container::Transparent)
        .into()
    }
}