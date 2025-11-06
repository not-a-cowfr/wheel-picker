use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

use nanoserde::{DeRon, SerRon};

#[cfg(any(target_os = "linux", target_os = "macos"))]
const CONFIG_DIR: &str = ".config/wheel-picker";

#[cfg(target_os = "windows")]
const CONFIG_DIR: &str = "wheel-picker";

pub fn create_config() {
	let config_dir = get_config_dir();
	let config_path = get_config_path();

	fs::create_dir_all(config_dir).unwrap();
	let mut file = fs::OpenOptions::new()
		.write(true)
		.create(true)
		.open(config_path)
		.unwrap();

	// make default config if file is empty
	if file.metadata().unwrap().len() == 0 {
		file.write_all(Config::serialize_ron(&Config::default()).as_bytes())
			.unwrap();
	}
}

fn get_config_path() -> PathBuf {
	let config_dir = get_config_dir();
	config_dir.join("config.ron")
}

fn get_config_dir() -> PathBuf {
	let home_dir = env::home_dir().unwrap();
	home_dir.join(CONFIG_DIR)
}

#[derive(Debug, Default, DeRon, SerRon)]
pub struct Config {
	#[nserde(default)]
	pub current_pool: Vec<String>,
}

impl Config {
	pub fn get() -> Config {
		let config_raw = fs::read_to_string(get_config_path()).unwrap();

		Config::deserialize_ron(&config_raw).unwrap()
	}

	pub fn update(&mut self) {
		let config_path = get_config_path();
		fs::write(config_path, Config::serialize_ron(&self)).unwrap();
	}
}
