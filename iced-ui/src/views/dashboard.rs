// src/views/dashboard.rs
use crate::api::{ApiClient, AnalysisStatus, ManifestStats, SystemStats};
use crate::theme::{RayhunterTheme};
use crate::widgets::toolbar;
use iced::{
    widget::{button, column, container, horizontal_rule, row, text, Column, horizontal_space, svg},
    Command, Element, Length, Color, Theme, Renderer,
    theme,
};
use log::info;

#[derive(Debug, Clone)]
pub enum Message {
    StartRecording,
    StopRecording,
    RefreshData,
    RunAnalysis,
    ToolbarMessage(toolbar::Message),
}

pub struct DashboardView {
    theme: RayhunterTheme,
    system_stats: Option<SystemStats>,
    qmdl_manifest: Option<ManifestStats>,
    analysis_status: Option<AnalysisStatus>,
    is_recording: bool,
    has_warnings: bool,
}

impl DashboardView {
    pub fn new(api_client: &ApiClient, theme: &RayhunterTheme) -> (Self, Command<Message>) {
        let dashboard = Self {
            theme: theme.clone(),
            system_stats: None,
            qmdl_manifest: None,
            analysis_status: None,
            is_recording: false,
            has_warnings: false,
        };

        // Initialize with data fetching
        let cmd = Command::batch(vec![
            api_client.get_system_stats().map(|res| match res {
                Ok(stats) => Message::RefreshData,
                Err(_) => Message::RefreshData,
            }),
            api_client.get_qmdl_manifest().map(|res| match res {
                Ok(_) => Message::RefreshData,
                Err(_) => Message::RefreshData,
            }),
        ]);

        (dashboard, cmd)
    }

    pub fn update(&mut self, message: Message, api_client: &ApiClient) -> Command<Message> {
        match message {
            Message::StartRecording => {
                info!("Starting recording");
                self.is_recording = true;
                api_client.start_recording().map(|_| Message::RefreshData)
            }
            Message::StopRecording => {
                info!("Stopping recording");
                self.is_recording = false;
                api_client.stop_recording().map(|_| Message::RefreshData)
            }
            Message::RefreshData => {
                Command::batch(vec![
                    api_client.get_system_stats().map(|_| Message::RefreshData),
                    api_client.get_qmdl_manifest().map(|_| Message::RefreshData),
                    api_client.get_analysis_status().map(|_| Message::RefreshData),
                ])
            }
            Message::RunAnalysis => {
                // Run analysis on current recording if available
                if let Some(manifest) = &self.qmdl_manifest {
                    if let Some(current) = &manifest.current_entry {
                        return api_client.start_analysis(&current.name).map(|_| Message::RefreshData);
                    }
                }
                Command::none()
            }
            Message::ToolbarMessage(_) => {
                // Toolbar messages are handled by the main app
                Command::none()
            }
        }
    }

    pub fn update_system_stats(&mut self, stats: SystemStats) {
        self.system_stats = Some(stats);
    }

    pub fn update_qmdl_manifest(&mut self, manifest: &ManifestStats) {
        self.qmdl_manifest = Some(manifest.clone());
        self.is_recording = manifest.current_entry.is_some();
        
        // Check for warnings in the current recording
        self.has_warnings = false; // Reset flag
        // In a real implementation, you would check analysis results for warnings
    }

    pub fn update_analysis_status(&mut self, status: &AnalysisStatus) {
        self.analysis_status = Some(status.clone());
    }

    fn status_indicator(&self) -> Element<Message> {
        let (status_color, status_text) = if self.has_warnings {
            (self.theme.error_color(), "WARNING DETECTED")
        } else if self.is_recording {
            (self.theme.success_color(), "RECORDING")
        } else {
            (self.theme.text_color(), "IDLE")
        };
        
        let indicator = container(
            text(status_text)
                .size(18)
                .style(status_color)
        )
        .padding(10)
        .style(theme::Container::Box);
        
        indicator.into()
    }

    pub fn view(&self) -> Element<Message> {
        let logo_image: Element<Message> = match std::fs::metadata("assets/rayhunter.svg") {
            Ok(_) => {
                let logo_handle = svg::Handle::from_path("assets/rayhunter.svg");
                svg::Svg::new(logo_handle)
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(200.0))
                    .into()
            },
            Err(_) => {
                // Fallback if SVG doesn't exist
                text("Rayhunter")
                    .size(32)
                    .style(self.theme.accent_color())
                    .into()
            }
        };

        // Control buttons
        let primary_button = |label, message| {
            button(text(label).style(Color::WHITE))
                .on_press(message)
                .padding([8, 16])
                .style(theme::Button::Primary)
        };

        let secondary_button = |label, message| {
            button(text(label).style(self.theme.text_color()))
                .on_press(message)
                .padding([8, 16])
                .style(theme::Button::Secondary)
        };

        let control_row = row![
            primary_button(
                if self.is_recording { "Stop Recording" } else { "Start Recording" },
                if self.is_recording { Message::StopRecording } else { Message::StartRecording }
            ),
            secondary_button("Refresh Data", Message::RefreshData),
            secondary_button("Run Analysis", Message::RunAnalysis),
            horizontal_space(Length::Fill),
            self.status_indicator(),
        ]
        .spacing(10)
        .padding(10);

        // System Stats Section
        let system_stats_section = self.create_card(
            "System Status",
            if let Some(stats) = &self.system_stats {
                // Create rows first
                let disk_row = row![
                    text("Disk:").style(self.theme.text_color()),
                    text(format!(
                        "{} used of {} ({})",
                        stats.disk_stats.used_size,
                        stats.disk_stats.total_size,
                        stats.disk_stats.used_percent
                    )).style(self.theme.text_color()),
                ].spacing(10);
                
                let memory_row = row![
                    text("Memory:").style(self.theme.text_color()),
                    text(format!(
                        "{} used of {}",
                        stats.memory_stats.used,
                        stats.memory_stats.total
                    )).style(self.theme.text_color()),
                ].spacing(10);
                
                // Then combine into a column and convert to element
                column![
                    disk_row,
                    memory_row,
                ].spacing(5).into()
            } else {
                text("Loading...").style(self.theme.text_color()).into()
            }
        );

        // Current Recording Section
        let current_recording = self.create_card(
            "Current Recording",
            if let Some(manifest) = &self.qmdl_manifest {
                if let Some(current) = &manifest.current_entry {
                    // Create rows first
                    let name_row = row![
                        text("Name:").style(self.theme.text_color()),
                        text(&current.name).style(self.theme.text_color()),
                    ].spacing(10);
                    
                    let started_row = row![
                        text("Started:").style(self.theme.text_color()),
                        text(format!(
                            "{}",
                            current.start_time.format("%Y-%m-%d %H:%M:%S")
                        )).style(self.theme.text_color()),
                    ].spacing(10);
                    
                    let size_row = row![
                        text("Size:").style(self.theme.text_color()),
                        text(format!(
                            "{} bytes",
                            current.qmdl_size_bytes.to_string()
                        )).style(self.theme.text_color()),
                    ].spacing(10);
                    
                    // Then combine into a column and convert to element
                    column![
                        name_row,
                        started_row,
                        size_row,
                    ].spacing(5).into()
                } else {
                    text("No active recording").style(self.theme.text_color()).into()
                }
            } else {
                text("Loading...").style(self.theme.text_color()).into()
            }
        );

        // Analysis Status Section
        let analysis_status = self.create_card(
            "Analysis Status",
            if let Some(status) = &self.analysis_status {
                let running = status.running.as_ref().map_or("None".to_string(), |s| s.clone());
                let queued = if status.queued.is_empty() {
                    "None".to_string()
                } else {
                    status.queued.join(", ")
                };

                let running_row = row![
                    text("Running:").style(self.theme.text_color()),
                    text(running).style(self.theme.text_color()),
                ].spacing(10);
                
                let queued_row = row![
                    text("Queued:").style(self.theme.text_color()),
                    text(queued).style(self.theme.text_color()),
                ].spacing(10);
                
                column![
                    running_row,
                    queued_row,
                ].spacing(5).into()
            } else {
                text("Loading...").style(self.theme.text_color()).into()
            }
        );

        // Recent Recordings Section
        let recent_recordings = self.create_card(
            "Recent Recordings",
            if let Some(manifest) = &self.qmdl_manifest {
                if manifest.entries.is_empty() {
                    text("No recordings found").style(self.theme.text_color()).into()
                } else {
                    let mut col = Column::new().spacing(5);
                    
                    // Add headers
                    let headers = row![
                        text("Name").width(Length::Fill).style(self.theme.text_color()),
                        text("Date").width(Length::Fill).style(self.theme.text_color()),
                        text("Size").width(Length::Fill).style(self.theme.text_color()),
                    ]
                    .spacing(10)
                    .padding(5);
                    
                    col = col.push(headers);
                    col = col.push(horizontal_rule(1));
                    
                    // Add entries (limit to 5)
                    for entry in manifest.entries.iter().take(5) {
                        let timestamp = entry.start_time.format("%Y-%m-%d %H:%M:%S").to_string();
                        let size_text = bytesize::to_string(entry.qmdl_size_bytes as u64, true);
                        
                        let row_content = row![
                            text(&entry.name).width(Length::Fill).style(self.theme.text_color()),
                            text(&timestamp).width(Length::Fill).style(self.theme.text_color()),
                            text(&size_text).width(Length::Fill).style(self.theme.text_color()),
                        ]
                        .spacing(10)
                        .padding(5);

                        col = col.push(row_content);
                    }
                    
                    // Convert column to Element
                    col.into()
                }
            } else {
                text("Loading...").style(self.theme.text_color()).into()
            }
        );

        // Main layout
        let content = column![
            control_row,
            row![system_stats_section, current_recording].spacing(20),
            row![analysis_status, recent_recordings].spacing(20),
        ]
        .spacing(20)
        .padding(20);
        
        // Wrap in a scroll container
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Container::Box)
            .into()
    }
    
    // Helper for creating styled card containers
    fn create_card<'a>(&self, title: &str, content: Element<'a, Message>) -> container::Container<'a, Message> {
        let title_text = text(title)
            .size(18)
            .style(self.theme.accent_color());
            
        let card_content = column![
            title_text,
            horizontal_rule(1),
            content,
        ]
        .spacing(10)
        .padding(10);
        
        container(card_content)
            .style(theme::Container::Box)
            .width(Length::Fill)
            .padding(10)
    }
}