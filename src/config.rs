use crate::{config};
use qmetaobject::*;









use log::{error, warn};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::path::{PathBuf};
use std::{fs};




use directories::ProjectDirs;
use itertools::Itertools;


pub static QUALIFIER: &str = "com.teddyheinen";
pub static ORGANIZATION: &str = "Teddy Heinen";
pub static APPLICATION: &str = "sky-menu";

#[derive(QObject, Default)]
pub struct Config {
    base: qt_base_class!(trait QObject),

    height: qt_property!(i32; NOTIFY config_changed),
    width: qt_property!(i32; NOTIFY config_changed),

    setup: qt_method!(fn(&mut self)),

    config_changed: qt_signal!(),
}

impl Config {
    // TODO make this pull from a file
    fn setup(&mut self) {
        self.height = 500;
        self.width = 400;

        self.config_changed();
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UsageCount {
    store: HashMap<String, i32>,
}

impl From<PathBuf> for UsageCount {
    fn from(x: PathBuf) -> Self {
        let json = fs::read_to_string(&x).unwrap_or("{}".to_string());
        UsageCount {
            store: serde_json::from_str(&json).unwrap_or(HashMap::new()),
        }
    }
}

impl UsageCount {
    pub fn inc(&mut self, app: &str) {
        self.set(app, self.get(app) + 1)
    }
    pub fn get(&self, app: &str) -> i32 {
        self.store.get(app).unwrap_or(&0).clone()
    }

    pub fn set(&mut self, app: &str, val: i32) {
        self.store.insert(app.to_string(), val);
        let json = match serde_json::to_string(&self.store) {
            Ok(x) => x,
            Err(e) => {
                warn!("Couldn't serialize usage database to json: {}", e);
                return;
            }
        };

        if let Some(proj_dirs) =
            ProjectDirs::from(config::QUALIFIER, config::ORGANIZATION, config::APPLICATION)
        {
            if let Err(e) = fs::create_dir_all(proj_dirs.data_dir()) {
                error!("Creating config directory failed: {}", e)
            }
            if let Err(e) = fs::write(proj_dirs.data_dir().join("usage.json"), json) {
                error!("Writing usage data failed: {}", e)
            }
        }
    }
}
