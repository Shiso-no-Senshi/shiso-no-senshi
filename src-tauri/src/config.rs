use std::{
    env::current_dir,
    fs::File,
    io::{BufReader, BufWriter, ErrorKind, Write},
};

use dirs::config_local_dir;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SnSConfiguration {
    pub collections: Vec<SnSCollection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnSCollection {
    pub name: String,
    pub path: String,
}

impl SnSConfiguration {
    pub fn load() -> Self {
        let mut conf_file_path = match config_local_dir() {
            Some(conf_path) => conf_path,
            None => current_dir().unwrap(),
        };
        conf_file_path = conf_file_path.join("shiso-config.json");

        match File::open(&conf_file_path) {
            Ok(file) => {
                let buf_reader = BufReader::new(file);
                let conf: SnSConfiguration = serde_json::from_reader(buf_reader).unwrap();
                debug!("Loaded configuration from {conf_file_path:?}");
                conf
            }
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    let new_conf = SnSConfiguration {
                        collections: Vec::new(),
                    };

                    let data = serde_json::to_string(&new_conf).unwrap();

                    let new_conf_file = File::create_new(&conf_file_path).unwrap();
                    let mut writer = BufWriter::new(new_conf_file);
                    writer.write_all(data.as_bytes()).unwrap();
                    writer.flush().unwrap();

                    debug!("Created new configuration at {conf_file_path:?}");

                    new_conf
                } else {
                    panic!("{e:?}");
                }
            }
        }
    }
}
