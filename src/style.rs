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
                Styles::Highlighted => Color::from_rgb8(47, 137, 197),
                Styles::TransparentDark => Color::from_rgba8(27, 29, 36, 1.0f32),
                Styles::Transparent => Color::from_rgba8(0, 0, 0, 0.0f32),
            })),
            ..Style::default()
        }
    }
}
