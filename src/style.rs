use iced::widget::container;
use iced::widget::container::Style;
use iced::{button, Background, Color};

pub enum Styles {
    TransparentDark,
    Transparent,
    Highlighted,
}

impl container::StyleSheet for Styles {
    fn style(&self) -> Style {
        Style {
            background: Some(Background::Color(match self {
                Styles::Highlighted => Color::from_rgb8(0x34, 0x98, 0xdb),
                Styles::TransparentDark => Color::from_rgba8(27, 29, 36, 1.0f32),
                Styles::Transparent => Color::from_rgba8(0, 0, 0, 0.0f32),
            })),
            text_color: Some(match self {
                Styles::Highlighted => Color::from_rgb8(0xD3, 0xDA, 0xE3),
                Styles::TransparentDark => Color::from_rgb8(0xD3, 0xDA, 0xE3),
                Styles::Transparent => Color::from_rgb8(0xD3, 0xDA, 0xE3),
            }),
            ..Style::default()
        }
    }
}
