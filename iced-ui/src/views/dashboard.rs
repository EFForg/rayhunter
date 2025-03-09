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
        // TODO: Implement warning detection
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
        // Title section
        let title = text("Rayhunter")
            .size(32)
            .style(self.theme.accent_color());
        
        let subtitle = text("IMSI Catcher Catcher")
            .size(16)
            .style(self.theme.text_color());
                
        // Control buttons
        let control_row = row![
            button(
                text(if self.is_recording { "Stop Recording" } else { "Start Recording" })
                    .style(Color::WHITE)
            )
            .on_press(if self.is_recording { Message::StopRecording } else { Message::StartRecording })
            .padding([8, 16])
            .style(theme::Button::Primary),
            
            button(text("Refresh Data").style(self.theme.text_color()))
                .on_press(Message::RefreshData)
                .padding([8, 16])
                .style(theme::Button::Secondary),
                
            button(text("Run Analysis").style(self.theme.text_color()))
                .on_press(Message::RunAnalysis)
                .padding([8, 16])
                .style(theme::Button::Secondary),
                
            horizontal_space(Length::Fill),
            
            self.status_indicator(),
        ]
        .spacing(10)
        .padding(10);

        // System Stats Section
        let system_stats_section = self.create_card(
            "System Status",
            if let Some(stats) = &self.system_stats {
                column![
                    row![
                        text("Disk:").style(self.theme.text_color()),
                        text(format!(
                            "{} used of {} ({})",
                            stats.disk_stats.used_size,
                            stats.disk_stats.total_size,
                            stats.disk_stats.used_percent
                        )).style(self.theme.text_color()),
                    ].spacing(10),
                    
                    row![
                        text("Memory:").style(self.theme.text_color()),
                        text(format!(
                            "{} used of {}",
                            stats.memory_stats.used,
                            stats.memory_stats.total
                        )).style(self.theme.text_color()),
                    ].spacing(10),
                ].spacing(5).into()
            } else {
                text("Waiting for system data...").style(self.theme.text_color()).into()
            }
        );

        // Remove the current recording section that has the logo
        let current_recording = self.create_card(
            "Current Recording",
            if let Some(manifest) = &self.qmdl_manifest {
                if let Some(current) = &manifest.current_entry {
                    column![
                        row![
                            text("Name:").style(self.theme.text_color()),
                            text(&current.name).style(self.theme.text_color()),
                        ].spacing(10),
                        
                        row![
                            text("Started:").style(self.theme.text_color()),
                            text(format!(
                                "{}",
                                current.start_time.format("%Y-%m-%d %H:%M:%S")
                            )).style(self.theme.text_color()),
                        ].spacing(10),
                        
                        row![
                            text("Size:").style(self.theme.text_color()),
                            text(format!(
                                "{} bytes",
                                current.qmdl_size_bytes.to_string()
                            )).style(self.theme.text_color()),
                        ].spacing(10),
                    ].spacing(5).into()
                } else {
                    text("No active recording").style(self.theme.text_color()).into()
                }
            } else {
                text("Waiting for recording data...").style(self.theme.text_color()).into()
            }
        );

        // Header without the SVG logo
        let header_section = row![
            column![
                title,
                subtitle,
            ].spacing(5),
            horizontal_space(Length::Fill),
            self.status_indicator(),
        ]
        .spacing(20)
        .padding(10)
        .align_items(iced::Alignment::Center);
        
        // Main layout
        let content = column![
            header_section,
            control_row,
            row![system_stats_section, current_recording].spacing(20),
        ]
        .spacing(20)
        .padding(20);
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Container::Box)
            .into()
    }

    fn fetch_data_with_retry(&self, api_client: &ApiClient) -> Command<Message> {
        Command::batch(vec![
            api_client.get_system_stats().map(|_| Message::RefreshData),
            api_client.get_qmdl_manifest().map(|_| Message::RefreshData),
            api_client.get_analysis_status().map(|_| Message::RefreshData),
        ])
    }
    
    // Helper for creating styled card containers
    fn create_card<'a>(&self, title: &str, content: Element<'a, Message>) -> container::Container<'a, Message> {
        let title_text = text(title)
            .size(20)
            .style(self.theme.accent_color());
            
        let card_content = column![
            title_text,
            horizontal_rule(2), // Make the divider thicker
            container(content).padding(15), // Add more internal padding
        ]
        .spacing(15)
        .padding(15);
        
        container(card_content)
            .style(theme::Container::Box)
            .width(Length::Fill)
            .padding(15)
    }
}