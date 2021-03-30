mod style;
mod widgets;

use iced::{
    button, executor, keyboard, window, Align, Application, Button, Color, Column, Command,
    Container, Element, HorizontalAlignment, Length, Sandbox, Settings, Subscription, Text,
};
use iced_native::{event, subscription, Event};

pub fn main() -> iced::Result {
    Launcher::run(Settings {
        window: window::Settings {
            size: (400, 500),
            resizable: false,
            transparent: true,
            decorations: false,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct Launcher {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    items: Vec<String>,
    selected: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    MoveSelectedUp,
    MoveSelectedDown,
    ResetSelected,
}

impl Application for Launcher {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                items: vec!["howdy".to_owned(), "world".to_owned()],
                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("sky-menu")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MoveSelectedUp => {
                self.selected =
                    (self.selected as isize - 1).rem_euclid(self.items.len() as isize) as usize
            }
            Message::MoveSelectedDown => {
                self.selected = (self.selected + 1).rem_euclid(self.items.len())
            }
            Message::ResetSelected => {
                self.selected = 0;
            }
        };
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| {
            if let event::Status::Captured = status {
                return None;
            }

            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    modifiers,
                    key_code,
                }) => handle_hotkey(key_code),
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<Message> {
        Container::new(self.items.iter().enumerate().fold(
            Column::new().align_items(Align::Start).width(Length::Fill),
            |sum, (idx, val)| {
                sum.push(
                    Container::new(crate::widgets::get_entry(
                        val.to_owned(),
                        "/usr/share/icons/Tela-purple/scalable/apps/firefox.svg".to_owned(),
                    ))
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .style(if self.selected == idx {
                        crate::style::Styles::Highlighted
                    } else {
                        crate::style::Styles::Transparent
                    }),
                )
            },
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(crate::style::Styles::TransparentDark)
        .into()
    }
}

fn handle_hotkey(key_code: keyboard::KeyCode) -> Option<Message> {
    println!("{:?}", key_code);
    match key_code {
        keyboard::KeyCode::Up => Some(Message::MoveSelectedUp),
        keyboard::KeyCode::Down => Some(Message::MoveSelectedDown),
        _ => None,
    }
}
