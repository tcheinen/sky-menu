use crate::application::Entry;
use i3ipc::reply::{Node, WindowProperty};
use i3ipc::I3Connection;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub fn get_xdg_data_dirs() -> impl Iterator<Item = PathBuf> {
    env::var("XDG_DATA_DIRS")
        .unwrap_or("/usr/local/share/:/usr/share/".into())
        .split(":")
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>() // FIXME why do i need to collect here
        .into_iter()
}

pub fn get_xdg_application_dirs() -> impl Iterator<Item = PathBuf> {
    get_xdg_data_dirs()
        .into_iter()
        .map(|x| PathBuf::from(x).join("applications"))
        .filter(|x| x.exists())
}

fn workspace(node: Node) -> Vec<Node> {
    if node.window.is_some() {
        return vec![node.clone()];
    }
    node.nodes
        .into_iter()
        .flat_map(workspace)
        .collect::<Vec<Node>>()
}

pub fn get_running_applications() -> Vec<Entry> {
    let mut conn = I3Connection::connect().expect("I3 IPC connection failed");
    let root = conn.get_tree().expect("couldn't get I3 tree");

    root.nodes[1..]
        .iter()
        .flat_map(|x| {x.nodes.clone()})
        .filter(|x| x.name == Some("content".into()))
        .flat_map(|x| x.nodes.into_iter())
        .flat_map(workspace)
        .map(|node| (
            node.name.unwrap_or("".into()),
            node.window.unwrap_or(0),
            node.window_properties
                .unwrap_or(HashMap::new())
                .get( &WindowProperty::Class)
                .unwrap_or(&"".to_string())
                .clone()))
        .map(|(name, id, icon)| {
            Entry::new(
                name,
                icon,
                "".into(),
                format!("i3-msg  [id={}] focus;i3-msg [title=Launcher] move workspace current; i3-msg [title=Launcher] focus;", id),
            )
        })
        .collect()
}
