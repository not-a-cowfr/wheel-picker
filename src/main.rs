use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use clap::{Command, arg};
use rand::Rng;
use rand::seq::IndexedRandom;

use crate::config::Config;

mod config;

fn cli() -> Command {
	Command::new("wheel")
		.about("Random wheel picker")
		.subcommand_required(true)
		.arg_required_else_help(true)
		.allow_external_subcommands(true)
		.subcommand(
			Command::new("add")
				.about("adds an entry to the wheel")
				.arg(arg!(<NAME> "The name of the entry"))
				.arg_required_else_help(true),
		)
		.subcommand(
			Command::new("remove")
				.about("removes an entry from the wheel")
				.arg(arg!(<NAME> "The name of the entry"))
				.arg_required_else_help(true),
		)
		.subcommand(Command::new("list").about("lists all entries currently in the wheel"))
		.subcommand(Command::new("clear").about("clears all entries from the wheel"))
		.subcommand(
			Command::new("pick")
				.about("picks a random entry from the wheel")
				.arg(arg!(-i --instant "skip the animation, pick an entry instantly"))
				.arg(
					arg!([AMOUNT] "the amount of entries to pick, default 1")
						.value_parser(clap::value_parser!(usize)),
				),
		)
}

fn main() {
	let matches = cli().get_matches();

	config::create_config();

	let mut config = Config::get();

	match matches.subcommand() {
		| Some(("add", sub_matches)) => {
			let entry = sub_matches.get_one::<String>("NAME").expect("required");

			if config.current_pool.is_none() {
				config.current_pool = Some(vec![]);
			}

			config.current_pool.as_mut().unwrap().push(entry.to_owned());

			config.update();

			println!(
				"added entry {} to wheel\n\nrun `wheel-picker list` to see all entries",
				entry
			);
		},
		| Some(("remove", sub_matches)) => {
			let entry = sub_matches.get_one::<String>("NAME").expect("required");

			if let Some(pool) = &mut config.current_pool {
				if !pool.is_empty() {
					if let Some(pos) = pool.iter().position(|e| e == entry) {
						pool.remove(pos);
						config.update();

						println!(
							"removed entry {} from wheel\n\nrun `wheel-picker list` to see all entries",
							entry
						);
					} else {
						println!("entry {} not found in wheel", entry);
					}
					return;
				}
			}
			println!("wheel has no entries")
		},
		| Some(("list", _)) => {
			if let Some(pool) = config.current_pool {
				if !pool.is_empty() {
					println!("current wheel pool:\n{}", pool.join("\n"));
					return;
				}
			}
			println!("wheel has no entries");
		},
		| Some(("clear", _)) => {
			print!("Are you sure? (y/N): ");
			io::stdout().flush().unwrap();

			let mut input = String::new();
			io::stdin().read_line(&mut input).unwrap();

			let input = input.trim().to_lowercase();
			if input == "y" || input == "yes" {
				config.current_pool = Some(vec![]);
				config.update();

				println!("cleared wheel");
			} else {
				println!("cancelled operation");
			}
		},
		| Some(("pick", sub_matches)) => {
			let is_instant = sub_matches.get_one::<bool>("instant").unwrap_or(&false);
			let amount = sub_matches.get_one::<usize>("AMOUNT").unwrap_or(&1);

			if let Some(pool) = config.current_pool {
				if !pool.is_empty() {
					let mut rng = rand::rng();

					if *is_instant {
						let mut picked: Vec<&str> = vec![];
						for _ in 0..*amount {
							picked.push(pool.choose(&mut rng).unwrap());
						}

						println!("picked entries: {}", picked.join(", "));

						return;
					} else {
						let mut picked: Vec<&str> = vec![];

						for i in 0..*amount {
							println!("\nSpin {}:", i + 1);

							let spin_time = rng.random_range(1..3);
							let start = std::time::Instant::now();

							let mut current = "";

							while start.elapsed().as_secs_f32() < spin_time as f32 {
								current = pool.choose(&mut rng).unwrap();
								print!("\r\x1B[2KSpinning... [{}]", current);
								std::io::Write::flush(&mut std::io::stdout()).unwrap();
								thread::sleep(Duration::from_millis(rng.random_range(50..150)));
							}

							picked.push(current);
							println!("\r\x1B[2Kpicked: {}", current);
						}

						println!("\npicked entries: {}", picked.join(", "));
						return;
					}
				}
			}
			println!("wheel has no entries\n\ntry adding some with `wheel-picker add <entry>`");
		},
		| _ => unreachable!(),
	}
}
