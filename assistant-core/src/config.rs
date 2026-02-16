use serde::Deserialize;
use dirs::config_dir;
use std::fs;
use std::path::PathBuf;
use std::io::Write;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub model: String,
    pub info_dump_path: String,
}

fn get_config_path() -> PathBuf {
    let base = config_dir().expect("Could not get config directory");
    base.join("ark/config.json")
}

fn ensure_config_exists(path: &PathBuf) {
    let dir = path.parent().unwrap();

    if !dir.exists() {
        fs::create_dir_all(dir).expect("Failed creating config directory");
    }

   if !path.exists() {
        let default_cfg = r#"
        {
            "model": "llama3.2:3b",
            "info_dump_path": data_dir.to_string_lossy()
        }
        "#;
        let mut file = fs::File::create(path).expect("Failed to create default config");
        file.write_all(default_cfg.as_bytes()).unwrap();
    }
}

pub fn load_config() -> Config {
    let path = get_config_path();

    ensure_config_exists(&path);

    let data = fs::read_to_string(path).expect("Failed to read config file");
    serde_json::from_str(&data).expect("Invalid config.json format")
}
