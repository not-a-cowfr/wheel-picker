use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use lexopt::{Arg, Parser};

use crate::config::Config;

mod config;

fn main() {
	config::create_config();
	let mut config = Config::get();

	let mut parser = Parser::from_env();
	let cmd = match parser.next() {
		| Ok(Some(Arg::Value(v))) => v.to_string_lossy().into_owned(),
		| _ => {
			eprintln!(
				"wheel-picker v{}\n\nusage: wheel <add|remove|list|clear|pick> [...]",
				env!("CARGO_PKG_VERSION")
			);
			return;
		},
	};

	match cmd.as_str() {
		| "add" => {
			let name = match parser.next() {
				| Ok(Some(Arg::Value(v))) => v.to_string_lossy().into_owned(),
				| _ => {
					eprintln!("usage: wheel add <NAME>");
					return;
				},
			};

			config.current_pool.push(name.clone());
			config.update();
			println!("added entry {name} to wheel\n\nrun `wheel-picker list` to see all entries");
		},

		| "remove" => {
			let name = match parser.next() {
				| Ok(Some(Arg::Value(v))) => v.to_string_lossy().into_owned(),
				| _ => {
					eprintln!("usage: wheel remove <NAME>");
					return;
				},
			};

			if let Some(pos) = config.current_pool.iter().position(|e| e == &name) {
				config.current_pool.remove(pos);
				config.update();
				println!(
					"removed entry {name} from wheel\n\nrun `wheel-picker list` to see all entries"
				);
			} else {
				println!("entry {name} not found in wheel");
			}
		},

		| "list" => {
			if config.current_pool.is_empty() {
				println!("wheel has no entries\n\ntry adding some with `wheel-picker add <entry>`");
			} else {
				println!("current wheel pool:\n{}", config.current_pool.join("\n"));
			}
		},

		| "clear" => {
			print!("Are you sure? (y/N): ");
			io::stdout().flush().unwrap();

			let mut input = String::new();
			io::stdin().read_line(&mut input).unwrap();

			let input = input.trim().to_lowercase();
			if input == "y" || input == "yes" {
				config.current_pool.clear();
				config.update();
				println!("cleared wheel");
			} else {
				println!("cancelled operation");
			}
		},

		| "pick" => {
			let mut instant = false;
			let mut amount: usize = 1;

			while let Some(Ok(arg)) = parser.next().transpose() {
				match arg {
					| Arg::Long("instant") | Arg::Short('i') => instant = true,

					| Arg::Value(v) => {
						amount = v.to_string_lossy().parse().unwrap_or(1);
					},
					| _ => {},
				}
			}

			if config.current_pool.is_empty() {
				println!("wheel has no entries\n\ntry adding some with `wheel-picker add <entry>`");
				return;
			}

			if instant {
				let mut picked: Vec<&str> = Vec::new();
				for _ in 0..amount {
					picked.push(fastrand::choice(&config.current_pool).unwrap());
				}
				println!("picked entries: {}", picked.join(", "));
			} else {
				let mut picked = Vec::new();
				for i in 0..amount {
					println!("\nSpin {}:", i + 1);

					let spin_time = fastrand::usize(1..3);
					let start = std::time::Instant::now();
					let mut current = "";

					while start.elapsed().as_secs_f32() < spin_time as f32 {
						current = fastrand::choice(&config.current_pool).unwrap();
						print!("\r\x1B[2KSpinning... [{}]", current);
						io::stdout().flush().unwrap();
						thread::sleep(Duration::from_millis(fastrand::u64(50..150)));
					}

					picked.push(current);
					println!("\r\x1B[2Kpicked: {}", current);
				}

				println!("\npicked entries: {}", picked.join(", "));
			}
		},

		| _ => eprintln!("unknown command {cmd}"),
	}
}
