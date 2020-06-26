use i3ipc::reply::Node;
use i3ipc::I3Connection;
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

fn workspace(node: &Node) -> Vec<Node> {
    if node.window.is_some() {
        return vec![node.clone()];
    }
    node.nodes.iter().flat_map(workspace).collect::<Vec<Node>>()
}

pub fn get_running_applications() {
    let mut conn = I3Connection::connect().expect("I3 IPC connection failed");
    let root = conn.get_tree().expect("couldn't get I3 tree");
    root.nodes[1]
        .nodes
        .iter()
        .find(|x| x.name == Some("content".into()))
        .unwrap()
        .nodes
        .iter()
        .flat_map(workspace)
        .for_each(|x| println!("{:#?}", x));
}
