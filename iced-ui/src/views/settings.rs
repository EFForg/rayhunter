// src/views/settings.rs
use crate::theme::{AppTheme, RayhunterTheme, Container, Button};
use iced::{
    widget::{button, checkbox, column, container, row, text, text_input, Column, Container, Row, Text},
    Command, Element, Length, Background,
};

#[derive(Debug, Clone)]
pub enum Message {
    ServerAddressChanged(String),
    SaveSettings,
    DebugModeToggled(bool),
    ColorblindModeToggled(bool),
    DarkModeToggled(bool),
    UiLevelChanged(u8),
    ThemeChanged(AppTheme),
}

pub struct SettingsView {
    theme: RayhunterTheme,
    server_address: String,
    debug_mode: bool,
    colorblind_mode: bool,
    dark_mode: bool,
    ui_level: u8,
    is_saved: bool,
}

impl SettingsView {
    pub fn new(theme: &RayhunterTheme) -> (Self, Command<Message>) {
        let settings = Self {
            theme: theme.clone(),
            server_address: "http://localhost:8080".to_string(),
            debug_mode: false,
            colorblind_mode: false,
            dark_mode: true,
            ui_level: 1,
            is_saved: false,
        };

        (settings, Command::none())
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ServerAddressChanged(address) => {
                self.server_address = address;
                self.is_saved = false;
                Command::none()
            }
            Message::SaveSettings => {
                self.is_saved = true;
                
                // TODO: Save settings to a config file
                
                Command::none()
            }
            Message::DebugModeToggled(enabled) => {
                self.debug_mode = enabled;
                self.is_saved = false;
                Command::none()
            }
            Message::ColorblindModeToggled(enabled) => {
                self.colorblind_mode = enabled;
                self.is_saved = false;
                Command::none()
            }
            Message::DarkModeToggled(enabled) => {
                self.dark_mode = enabled;
                self.is_saved = false;
                Command::perform(async move { enabled }, |enabled| 
                    Message::ThemeChanged(if enabled { AppTheme::Dark } else { AppTheme::Light })
                )
            }
            Message::UiLevelChanged(level) => {
                self.ui_level = level;
                self.is_saved = false;
                Command::none()
            }
            Message::ThemeChanged(_) => {
                // This message is handled by the main app
                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("Settings")
            .size(32)
            .style(self.theme.accent_color());

        let server_address_row = row![
            text("Server Address:").width(Length::FillPortion(1)).style(self.theme.text_color()),
            text_input("http://localhost:8080", &self.server_address)
                .on_input(Message::ServerAddressChanged)
                .width(Length::FillPortion(3))
        ]
        .spacing(10)
        .padding(5)
        .width(Length::Fill);

        let debug_mode_row = row![
            text("Debug Mode:").width(Length::FillPortion(1)).style(self.theme.text_color()),
            checkbox("Enable debug mode", self.debug_mode, Message::DebugModeToggled)
                .width(Length::FillPortion(3))
        ]
        .spacing(10)
        .padding(5)
        .width(Length::Fill);

        let colorblind_mode_row = row![
            text("Colorblind Mode:").width(Length::FillPortion(1)).style(self.theme.text_color()),
            checkbox(
                "Enable colorblind mode (use blue instead of green)",
                self.colorblind_mode,
                Message::ColorblindModeToggled
            )
            .width(Length::FillPortion(3))
        ]
        .spacing(10)
        .padding(5)
        .width(Length::Fill);
        
        let dark_mode_row = row![
            text("Dark Mode:").width(Length::FillPortion(1)).style(self.theme.text_color()),
            checkbox(
                "Enable dark mode",
                self.dark_mode,
                Message::DarkModeToggled
            )
            .width(Length::FillPortion(3))
        ]
        .spacing(10)
        .padding(5)
        .width(Length::Fill);

        let ui_level_section = column![
            text("UI Level:").width(Length::Fill).style(self.theme.text_color()).size(16),
            row![
                checkbox(
                    "0: Invisible mode (no display)",
                    self.ui_level == 0,
                    |checked| Message::UiLevelChanged(if checked { 0 } else { 1 })
                )
            ].width(Length::Fill),
            row![
                checkbox(
                    "1: Simple line (default)",
                    self.ui_level == 1,
                    |checked| Message::UiLevelChanged(if checked { 1 } else { 0 })
                )
            ].width(Length::Fill),
            row![
                checkbox(
                    "2: Animated Orca",
                    self.ui_level == 2,
                    |checked| Message::UiLevelChanged(if checked { 2 } else { 1 })
                )
            ].width(Length::Fill),
            row![
                checkbox(
                    "3: EFF Logo",
                    self.ui_level == 3,
                    |checked| Message::UiLevelChanged(if checked { 3 } else { 1 })
                )
            ].width(Length::Fill),
            row![
                checkbox(
                    "128: Trans Pride",
                    self.ui_level == 128,
                    |checked| Message::UiLevelChanged(if checked { 128 } else { 1 })
                )
            ].width(Length::Fill)
        ]
        .spacing(5)
        .padding(10);

        let save_button = button(text("Save Settings").style(self.theme.text_color()))
            .on_press(Message::SaveSettings)
            .padding([8, 16])
            .style(Button::Primary);

        let status_text = if self.is_saved {
            text("Settings saved!").style(self.theme.success_color())
        } else {
            text("Unsaved changes").style(self.theme.selected_text_color())
        };

        let content = column![
            title,
            container(column![
                server_address_row,
                debug_mode_row,
                colorblind_mode_row,
                dark_mode_row,
                container(ui_level_section)
                    .style(Container::Section),
                row![save_button, status_text].spacing(20).padding(10),
            ].spacing(15))
            .style(Container::Card)
            .padding(20)
        ]
        .spacing(20)
        .padding(20);

        // Add the EFF logo to the bottom right
        let eff_logo_container = container(
            column![
                text("A project of the").style(self.theme.text_color()),
                text("Electronic Frontier Foundation").style(self.theme.text_color()).size(16),
                // Here you would include the SVG logo as well
            ]
            .spacing(5)
            .align_items(iced::Alignment::End)
        )
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Right)
        .padding(10);

        container(
            column![
                content,
                eff_logo_container,
            ]
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .style(Container::Transparent)
        .into()
    }
}