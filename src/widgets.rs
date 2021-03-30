use crate::Message;
use iced::{image, Container, Element, Image, Row, Svg, Text, svg};

pub fn get_entry(name: String, icon: String) -> Element<'static, Message> {
    // println!("{}", icon);
    Row::new()
        .push(Svg::from_path(&icon)) // TODO this should automatically pick between Image and Svg
        .push(Text::new(name.to_string()).size(50))
        .into()
}
