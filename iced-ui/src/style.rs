use iced::{theme, Color, Background, Theme};

pub const BRAND_COLOR: Color = Color {
    r: 0.5,
    g: 0.5,
    b: 0.9,
    a: 1.0,
};

pub const ERROR_COLOR: Color = Color {
    r: 0.9,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};

pub const SUCCESS_COLOR: Color = Color {
    r: 0.2,
    g: 0.8,
    b: 0.2,
    a: 1.0,
};

pub const WARNING_COLOR: Color = Color {
    r: 0.9,
    g: 0.7,
    b: 0.0,
    a: 1.0,
};

pub const ACCENT_COLOR: Color = Color {
    r: 0.3,
    g: 0.65,
    b: 0.9,
    a: 1.0,
};

pub const SELECTION_COLOR: Color = Color {
    r: 0.1,
    g: 0.4,
    b: 0.9,
    a: 1.0,
};

// Helper functions for consistent styling
pub fn get_text_style(selected: bool) -> Color {
    if selected {
        SELECTION_COLOR
    } else {
        Color::BLACK
    }
}

// Custom button styles using standard theme
pub enum Button {
    Primary,
    Secondary,
    Destructive,
}

impl From<Button> for theme::Button {
    fn from(button: Button) -> Self {
        match button {
            Button::Primary => theme::Button::Primary,
            Button::Secondary => theme::Button::Secondary,
            Button::Destructive => theme::Button::Destructive,
        }
    }
}