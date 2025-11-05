use std::path::PathBuf;
use std::{env, fs};

use serde::{Deserialize, Serialize};

#[cfg(any(target_os = "linux", target_os = "macos"))]
const CONFIG_DIR: &str = ".config/wheel-picker";

#[cfg(target_os = "windows")]
const CONFIG_DIR: &str = "wheel-picker";

pub fn create_config() {
	let config_dir = get_config_dir();
	let config_path = get_config_path();

	fs::create_dir_all(config_dir).unwrap();
	let _ = fs::OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(config_path);
}

fn get_config_path() -> PathBuf {
	let config_dir = get_config_dir();
	config_dir.join("config.toml")
}

fn get_config_dir() -> PathBuf {
	let home_dir = env::home_dir().unwrap();
	home_dir.join(CONFIG_DIR)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub current_pool: Option<Vec<String>>,
}

impl Config {
	pub fn get() -> Config {
		let config_raw = fs::read_to_string(get_config_path()).unwrap();

		toml::from_str::<Config>(&config_raw).unwrap()
	}

	pub fn update(&mut self) {
		let config_path = get_config_path();
		fs::write(config_path, toml::to_string(&self).unwrap()).unwrap();
	}
}
