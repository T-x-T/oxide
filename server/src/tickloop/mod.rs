use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;

use lib::game::PacketHandlerAction;
use lib::packets::Packet;
use lib::types::*;

mod packet_handler_actions;
mod process_entity_tick_outcome;
mod send_keepalives;
mod tick_blockentities;
mod tick_entities;

#[allow(dead_code)]
#[derive(Debug)]
pub struct TickTimings {
	pub save_all: std::time::Duration,
	pub clone_players: std::time::Duration,
	pub send_keepalives: std::time::Duration,
	pub tick_blockentities: std::time::Duration,
	pub tick_entities: std::time::Duration,
	pub packet_handler_actions: std::time::Duration,
}

pub fn tick(game: Arc<Game>) -> TickTimings {
	let now = std::time::Instant::now();
	// TODO: Make a better way to handle configuration to avoid repeated and complex code
	// all this just to get the number to a u64
	// also shouldn't be run every tick but just once in initialize_server()
	let save_interval: u64 = std::env::var("OXIDE_SAVE_SECONDS").unwrap_or("60".to_string()).parse::<u64>().unwrap_or(60);

	if std::time::Instant::now() > *game.last_save_all_timestamp.lock().unwrap() + std::time::Duration::from_secs(save_interval) {
		println!("run save-all");
		game.save_all();
	}
	let duration_save_all = std::time::Instant::now() - now;

	let now = std::time::Instant::now();
	let players_clone = game.players.lock().unwrap().clone();
	let duration_clone_players = std::time::Instant::now() - now;


	let now = std::time::Instant::now();
	packet_handler_actions::process(game.clone(), &players_clone);
	let duration_packet_handler_actions = std::time::Instant::now() - now;

	let now = std::time::Instant::now();
	send_keepalives::process(game.clone(), players_clone.clone());
	let duration_send_keepalives = std::time::Instant::now() - now;

	let now = std::time::Instant::now();
	tick_blockentities::process(game.clone(), &players_clone);
	let duration_tick_blockentities = std::time::Instant::now() - now;

	let now = std::time::Instant::now();
	tick_entities::process(game.clone(), &players_clone);
	let duration_tick_entities = std::time::Instant::now() - now;

	let entity_tick_outcomes: Vec<(i32, EntityTickOutcome)> = game
		.players
		.lock()
		.unwrap()
		.iter_mut()
		.map(|player| {
			(
				player.entity_id,
				player.tick(game.world.lock().unwrap().dimensions.get("minecraft:overworld").unwrap(), &players_clone, game.clone()),
			)
		})
		.collect();

	process_entity_tick_outcome::process(
		entity_tick_outcomes,
		game.clone(),
		&players_clone,
		game.world.lock().unwrap().dimensions.get_mut("minecraft:overworld").unwrap(),
	);

	return TickTimings {
		save_all: duration_save_all,
		clone_players: duration_clone_players,
		send_keepalives: duration_send_keepalives,
		tick_blockentities: duration_tick_blockentities,
		tick_entities: duration_tick_entities,
		packet_handler_actions: duration_packet_handler_actions,
	};
}
