use iced::{
    widget::{button, checkbox, column, container, row, text, text_input, Column, Text},
    Command, Element, Length,
};

#[derive(Debug, Clone)]
pub enum Message {
    ServerAddressChanged(String),
    SaveSettings,
    DebugModeToggled(bool),
    ColorblindModeToggled(bool),
    UiLevelChanged(u8),
}

pub struct SettingsView {
    server_address: String,
    debug_mode: bool,
    colorblind_mode: bool,
    ui_level: u8,
    is_saved: bool,
}

impl SettingsView {
    pub fn new() -> (Self, Command<Message>) {
        let settings = Self {
            server_address: "http://localhost:8080".to_string(),
            debug_mode: false,
            colorblind_mode: false,
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
            Message::UiLevelChanged(level) => {
                self.ui_level = level;
                self.is_saved = false;
                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("Settings")
            .size(32)
            .style(iced::Color::from_rgb(0.5, 0.5, 0.9));

        let server_address_row = row![
            text("Server Address:").width(Length::FillPortion(1)),
            text_input("http://localhost:8080", &self.server_address)
                .on_input(Message::ServerAddressChanged)
                .width(Length::FillPortion(3))
        ]
        .spacing(10)
        .padding(5)
        .width(Length::Fill);

        let debug_mode_row = row![
            text("Debug Mode:").width(Length::FillPortion(1)),
            checkbox("Enable debug mode", self.debug_mode, Message::DebugModeToggled)
                .width(Length::FillPortion(3))
        ]
        .spacing(10)
        .padding(5)
        .width(Length::Fill);

        let colorblind_mode_row = row![
            text("Colorblind Mode:").width(Length::FillPortion(1)),
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

        let ui_level_section = column![
            text("UI Level:").width(Length::Fill),
            row![
                checkbox(
                    "0: Invisible mode (no display)",
                    self.ui_level == 0,
                    |checked| Message::UiLevelChanged(if checked { 0 } else { 1 })
                )
            ],
            row![
                checkbox(
                    "1: Simple line (default)",
                    self.ui_level == 1,
                    |checked| Message::UiLevelChanged(if checked { 1 } else { 0 })
                )
            ],
            row![
                checkbox(
                    "2: Animated Orca",
                    self.ui_level == 2,
                    |checked| Message::UiLevelChanged(if checked { 2 } else { 1 })
                )
            ],
            row![
                checkbox(
                    "3: EFF Logo",
                    self.ui_level == 3,
                    |checked| Message::UiLevelChanged(if checked { 3 } else { 1 })
                )
            ],
            row![
                checkbox(
                    "128: Trans Pride",
                    self.ui_level == 128,
                    |checked| Message::UiLevelChanged(if checked { 128 } else { 1 })
                )
            ]
        ]
        .spacing(5)
        .padding(5);

        let save_button = button(text("Save Settings"))
            .on_press(Message::SaveSettings)
            .padding([8, 16])
            .style(iced::theme::Button::Primary);

        let status_text = if self.is_saved {
            text("Settings saved!").style(iced::Color::from_rgb(0.0, 0.5, 0.0))
        } else {
            text("Unsaved changes").style(iced::Color::from_rgb(0.5, 0.5, 0.5))
        };

        container(
            column![
                title,
                server_address_row,
                debug_mode_row,
                colorblind_mode_row,
                ui_level_section,
                row![save_button, status_text].spacing(20),
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