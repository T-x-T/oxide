use std::net::SocketAddr;
use std::sync::Arc;

use super::*;

pub fn init(game: &mut Game) {
	game.commands.lock().unwrap().push(Command {
		name: "time".to_string(),
		permission: Permission::Operator,
		execute,
		arguments: vec![
			CommandArgument {
				name: "add".to_string(),
				properties: ParserProperty::String(0),
				next_arguments: vec![CommandArgument {
					name: "time".to_string(),
					properties: ParserProperty::Time(0),
					next_arguments: vec![],
					optional: false,
					is_literal: false,
				}],
				optional: false,
				is_literal: true,
			},
			CommandArgument {
				name: "set".to_string(),
				properties: ParserProperty::String(0),
				next_arguments: vec![CommandArgument {
					name: "time".to_string(),
					properties: ParserProperty::Time(0),
					next_arguments: vec![],
					optional: false,
					is_literal: false,
				}],
				optional: false,
				is_literal: true,
			},
			CommandArgument {
				name: "query".to_string(),
				properties: ParserProperty::String(0),
				next_arguments: vec![
					CommandArgument {
						name: "day".to_string(),
						properties: ParserProperty::String(0),
						next_arguments: vec![],
						optional: false,
						is_literal: true,
					},
					CommandArgument {
						name: "daytime".to_string(),
						properties: ParserProperty::String(0),
						next_arguments: vec![],
						optional: false,
						is_literal: true,
					},
					CommandArgument {
						name: "gametime".to_string(),
						properties: ParserProperty::String(0),
						next_arguments: vec![],
						optional: false,
						is_literal: true,
					},
				],
				optional: false,
				is_literal: true,
			},
		],
	});
}

fn execute(command: String, socket_addr: Option<SocketAddr>, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
	let mut world = game.world.lock().unwrap();

	let parts: Vec<&str> = command.split(" ").collect();

	let reply = if parts.len() < 3 {
		"missing argument".to_string()
	} else {
		match parts[1] {
			"query" => query(parts[2], &world),
			"add" => add(parse_time(parts[2]), &mut world),
			"set" => set(parse_time(parts[2]), &mut world),
			x => format!("invalid argument {x}"),
		}
	};

	if let Some(socket_addr) = socket_addr {
		game.packet_sender.send_packet_to_player(
			&socket_addr,
			lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
			lib::packets::clientbound::play::SystemChatMessage {
				content: NbtTag::Root(vec![
					NbtTag::String("type".to_string(), "text".to_string()),
					NbtTag::String("text".to_string(), reply.to_string()),
				]),
				overlay: true,
			},
		);
	} else {
		println!("{reply}");
		return Ok(());
	}

	return Ok(());
}

fn parse_time(input: &str) -> i64 {
	let mut input = input.to_string();
	let last_char = input.chars().last().unwrap_or_default();
	if !['t', 's', 'd', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(&last_char) {
		return 0;
	}
	let mode = if ['t', 's', 'd'].contains(&last_char) { input.remove(input.len() - 1) } else { 't' };
	let input_number: f32 = input.parse().unwrap_or_default();
	if mode == 's' {
		return (input_number * 20.0) as i64;
	} else if mode == 'd' {
		return (input_number * 24_000.0) as i64;
	} else {
		return input_number as i64;
	}
}

fn query(argument: &str, world: &World) -> String {
	return match argument {
		"day" => (world.time / 24_000).to_string(),
		"daytime" => world.time.to_string(),
		"gametime" => world.world_age.to_string(),
		x => format!("invalid argument {x}"),
	};
}

fn add(ticks: i64, world: &mut World) -> String {
	world.time += ticks;
	return format!("Set the time to {}", world.time);
}

fn set(ticks: i64, world: &mut World) -> String {
	world.time = ticks;
	return format!("Set the time to {}", world.time);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse_time_positive_integer() {
		assert_eq!(parse_time("1234"), 1234);
	}

	#[test]
	fn parse_time_negative_integer() {
		assert_eq!(parse_time("-1234"), -1234);
	}

	#[test]
	fn parse_time_zero() {
		assert_eq!(parse_time("0"), 0);
	}

	#[test]
	fn random_mess() {
		assert_eq!(parse_time("oeirgnoe"), 0);
	}

	#[test]
	fn with_ticks() {
		assert_eq!(parse_time("123t"), 123);
	}

	#[test]
	fn with_seconds() {
		assert_eq!(parse_time("123s"), 2460);
	}

	#[test]
	fn with_days() {
		assert_eq!(parse_time("2d"), 48_000);
	}

	#[test]
	fn with_days_fraction() {
		assert_eq!(parse_time("0.5d"), 12_000);
	}

	#[test]
	fn with_days_fraction_no_leading_zero() {
		assert_eq!(parse_time(".5d"), 12_000);
	}
}
