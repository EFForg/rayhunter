mod api;
mod config;
mod style;
mod views;
mod widgets;

use anyhow::Result;
use iced::{
    Application, Command, Element, Settings, Subscription, Theme, 
    window, executor,
};
use log::{error, info};

use crate::config::Config;
use crate::views::{dashboard::DashboardView, recordings::RecordingsView, settings::SettingsView};

pub fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting Rayhunter UI");

    // Start the Iced application
    RayhunterUI::run(Settings {
        window: window::Settings {
            size: (1024, 768),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    })?;

    Ok(())
}

#[derive(Debug, Clone)]
pub enum Page {
    Dashboard,
    Recordings,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(Page),
    ApiError(String),
    
    // Dashboard messages
    Dashboard(views::dashboard::Message),
    
    // Recordings messages
    Recordings(views::recordings::Message),
    
    // Settings messages
    Settings(views::settings::Message),
    
    // API response messages
    SystemStatsResponse(Result<api::SystemStats, String>),
    QmdlManifestResponse(Result<api::ManifestStats, String>),
    AnalysisStatusResponse(Result<api::AnalysisStatus, String>),
    StartRecordingResponse(Result<(), String>),
    StopRecordingResponse(Result<(), String>),
}

pub struct RayhunterUI {
    api_client: api::ApiClient,
    current_page: Page,
    dashboard: views::dashboard::DashboardView,
    recordings: views::recordings::RecordingsView,
    settings: views::settings::SettingsView,
    config: Config,
}

impl Application for RayhunterUI {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Config;

    fn new(flags: Config) -> (Self, Command<Message>) {
        let api_client = api::ApiClient::new(&flags.server_address);
        
        let (dashboard, dashboard_cmd) = DashboardView::new(&api_client);
        let (recordings, recordings_cmd) = RecordingsView::new(&api_client);
        let (settings, settings_cmd) = SettingsView::new();
        
        // Combine commands
        let cmd = Command::batch(vec![
            dashboard_cmd.map(Message::Dashboard),
            recordings_cmd.map(Message::Recordings),
            settings_cmd.map(Message::Settings),
        ]);
        
        (
            Self {
                api_client,
                current_page: Page::Dashboard,
                dashboard,
                recordings,
                settings,
                config: flags,
            },
            cmd,
        )
    }

    fn title(&self) -> String {
        match self.current_page {
            Page::Dashboard => "Rayhunter - Dashboard",
            Page::Recordings => "Rayhunter - Recordings",
            Page::Settings => "Rayhunter - Settings",
        }
        .to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NavigateTo(page) => {
                self.current_page = page;
                Command::none()
            }
            
            Message::ApiError(error) => {
                error!("API error: {}", error);
                Command::none()
            }
            
            Message::Dashboard(msg) => {
                self.dashboard.update(msg, &self.api_client).map(Message::Dashboard)
            }
            
            Message::Recordings(msg) => {
                self.recordings.update(msg, &self.api_client).map(Message::Recordings)
            }
            
            Message::Settings(msg) => {
                self.settings.update(msg).map(Message::Settings)
            }
            
            Message::SystemStatsResponse(result) => {
                match result {
                    Ok(stats) => {
                        self.dashboard.update_system_stats(stats);
                        Command::none()
                    }
                    Err(e) => {
                        error!("Failed to fetch system stats: {}", e);
                        Command::none()
                    }
                }
            }
            
            Message::QmdlManifestResponse(result) => {
                match result {
                    Ok(manifest) => {
                        self.dashboard.update_qmdl_manifest(&manifest);
                        self.recordings.update_qmdl_manifest(&manifest);
                        Command::none()
                    }
                    Err(e) => {
                        error!("Failed to fetch QMDL manifest: {}", e);
                        Command::none()
                    }
                }
            }
            
            Message::AnalysisStatusResponse(result) => {
                match result {
                    Ok(status) => {
                        self.dashboard.update_analysis_status(&status);
                        self.recordings.update_analysis_status(&status);
                        Command::none()
                    }
                    Err(e) => {
                        error!("Failed to fetch analysis status: {}", e);
                        Command::none()
                    }
                }
            }
            
            Message::StartRecordingResponse(result) => {
                match result {
                    Ok(_) => {
                        // Refresh data after starting recording
                        Command::batch(vec![
                            self.api_client.get_qmdl_manifest().map(Message::QmdlManifestResponse),
                            self.api_client.get_analysis_status().map(Message::AnalysisStatusResponse),
                        ])
                    }
                    Err(e) => {
                        error!("Failed to start recording: {}", e);
                        Command::none()
                    }
                }
            }
            
            Message::StopRecordingResponse(result) => {
                match result {
                    Ok(_) => {
                        // Refresh data after stopping recording
                        Command::batch(vec![
                            self.api_client.get_qmdl_manifest().map(Message::QmdlManifestResponse),
                            self.api_client.get_analysis_status().map(Message::AnalysisStatusResponse),
                        ])
                    }
                    Err(e) => {
                        error!("Failed to stop recording: {}", e);
                        Command::none()
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self.current_page {
            Page::Dashboard => self.dashboard.view().map(Message::Dashboard),
            Page::Recordings => self.recordings.view().map(Message::Recordings),
            Page::Settings => self.settings.view().map(Message::Settings),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        // For periodic data refresh or other background tasks
        // iced::time::every(std::time::Duration::from_secs(1))
        //     .map(|_| Message::Tick)
        Subscription::none()
    }
}