use crate::api::{ApiClient, AnalysisStatus, ManifestStats, SystemStats};
use crate::style;
use iced::{
    widget::{button, column, container, row, text, Column, Row},
    Command, Element, Length,
};
use log::info;

#[derive(Debug, Clone)]
pub enum Message {
    StartRecording,
    StopRecording,
    RefreshData,
    RunAnalysis,
}

pub struct DashboardView {
    system_stats: Option<SystemStats>,
    qmdl_manifest: Option<ManifestStats>,
    analysis_status: Option<AnalysisStatus>,
    is_recording: bool,
}

impl DashboardView {
    pub fn new(api_client: &ApiClient) -> (Self, Command<Message>) {
        let dashboard = Self {
            system_stats: None,
            qmdl_manifest: None,
            analysis_status: None,
            is_recording: false,
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
        }
    }

    pub fn update_system_stats(&mut self, stats: SystemStats) {
        self.system_stats = Some(stats);
    }

    pub fn update_qmdl_manifest(&mut self, manifest: &ManifestStats) {
        self.qmdl_manifest = Some(manifest.clone());
        self.is_recording = manifest.current_entry.is_some();
    }

    pub fn update_analysis_status(&mut self, status: &AnalysisStatus) {
        self.analysis_status = Some(status.clone());
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("Rayhunter Dashboard")
            .size(32)
            .style(iced::Color::from_rgb(0.5, 0.5, 0.9));

        // Control buttons
        let control_button = |label, message, is_primary| {
            let btn = button(text(label))
                .padding([8, 16])
                .style(if is_primary {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                });

            btn.on_press(message)
        };

        let control_row = row![
            control_button(
                if self.is_recording { "Stop Recording" } else { "Start Recording" },
                if self.is_recording { Message::StopRecording } else { Message::StartRecording },
                true
            ),
            control_button("Refresh Data", Message::RefreshData, false),
            control_button("Run Analysis", Message::RunAnalysis, false),
        ]
        .spacing(10);

        // System Stats Section
        let system_stats_section = if let Some(stats) = &self.system_stats {
            column![
                text("System Stats").size(20),
                text(format!(
                    "Disk: {} used of {} ({})",
                    stats.disk_stats.used_size,
                    stats.disk_stats.total_size,
                    stats.disk_stats.used_percent
                )),
                text(format!(
                    "Memory: {} used of {}",
                    stats.memory_stats.used,
                    stats.memory_stats.total
                )),
            ]
        } else {
            column![text("System Stats").size(20), text("Loading...")]
        }
        .spacing(5)
        .padding(10);

        // Current Recording Section
        let current_recording = if let Some(manifest) = &self.qmdl_manifest {
            if let Some(current) = &manifest.current_entry {
                column![
                    text("Current Recording").size(20),
                    text(format!("Name: {}", current.name)),
                    text(format!(
                        "Started: {}",
                        current.start_time.format("%Y-%m-%d %H:%M:%S")
                    )),
                    text(format!(
                        "Size: {} bytes",
                        current.qmdl_size_bytes.to_string()
                    )),
                ]
            } else {
                column![
                    text("Current Recording").size(20),
                    text("No active recording"),
                ]
            }
        } else {
            column![text("Current Recording").size(20), text("Loading...")]
        }
        .spacing(5)
        .padding(10);

        // Analysis Status Section
        let analysis_status = if let Some(status) = &self.analysis_status {
            let running = status.running.as_ref().map_or("None".to_string(), |s| s.clone());
            let queued = if status.queued.is_empty() {
                "None".to_string()
            } else {
                status.queued.join(", ")
            };

            column![
                text("Analysis Status").size(20),
                text(format!("Running: {}", running)),
                text(format!("Queued: {}", queued)),
            ]
        } else {
            column![text("Analysis Status").size(20), text("Loading...")]
        }
        .spacing(5)
        .padding(10);

        // Recent Recordings Section
        let recent_recordings = if let Some(manifest) = &self.qmdl_manifest {
            let recent_entries: Vec<Element<_>> = manifest
                .entries
                .iter()
                .take(5)
                .map(|entry| {
                    let timestamp = entry.start_time.format("%Y-%m-%d %H:%M:%S").to_string();
                    let row_content = row![
                        text(&entry.name).width(Length::Fill),
                        text(&timestamp).width(Length::Fill),
                        text(format!("{} bytes", entry.qmdl_size_bytes)).width(Length::Fill),
                    ]
                    .spacing(10);

                    container(row_content)
                        .padding(5)
                        .width(Length::Fill)
                        .style(iced::theme::Container::Box)
                        .into()
                })
                .collect();

            if recent_entries.is_empty() {
                column![text("Recent Recordings").size(20), text("No recordings found")]
            } else {
                let mut col = Column::new().spacing(5).push(text("Recent Recordings").size(20));
                for entry in recent_entries {
                    col = col.push(entry);
                }
                col
            }
        } else {
            column![text("Recent Recordings").size(20), text("Loading...")]
        }
        .spacing(5)
        .padding(10);

        container(
            column![
                title,
                control_row,
                row![system_stats_section, current_recording].spacing(20),
                row![analysis_status, recent_recordings].spacing(20),
            ]
            .spacing(20)
            .padding(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .into()
    }
}