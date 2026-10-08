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
	return input.parse().unwrap();
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
