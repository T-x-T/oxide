#![allow(clippy::needless_return)]

use dashmap::DashMap;
use lib::packets::Packet;
use lib::types::*;
use std::error::Error;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};

mod command;
mod packet_handlers;
mod terminal_input;
mod tickloop;

fn main() {
	println!("Starting the oxide server");
	initialize_server();
}

fn initialize_server() {
	let listener = TcpListener::bind(std::env::var("OXIDE_LISTEN_ON").unwrap_or("0.0.0.0:25565".to_string())).unwrap();
	println!("oxide listening on {}", listener.local_addr().unwrap());

	let block_states = data::blocks::get_blocks();

	let world_loader = lib::world::loader::vanilla::Loader {
		path: Path::new(&std::env::var("OXIDE_WORLD_PATH").unwrap_or("./world".to_string())).to_owned(),
	};

	let default_gamemode = match std::env::var("OXIDE_DEFAULT_GAMEMODE").unwrap_or("survival".to_string()).as_str() {
		"survival" => Gamemode::Survival,
		"creative" => Gamemode::Creative,
		"adventure" => Gamemode::Adventure,
		"spectator" => Gamemode::Spectator,
		_ => Gamemode::Survival,
	};

	let entity_id_manager = EntityIdManager::default();
	let mut game = Game {
		players: Mutex::new(Vec::new()),
		world: Mutex::new(World::new(world_loader, &entity_id_manager, &block_states)),
		entity_id_manager,
		commands: Mutex::new(Vec::new()),
		last_save_all_timestamp: Mutex::new(std::time::Instant::now()),
		last_player_keepalive_timestamp: Mutex::new(std::time::Instant::now()),
		block_state_data: block_states,
		connections: DashMap::new(),
		packet_handler_actions: Mutex::new(Vec::new()),
		packet_send_queues: DashMap::new(),
		default_gamemode,
	};

	command::init(&mut game);

	let game: Arc<Game> = Arc::new(game);

	terminal_input::init(game.clone());

	let game_clone = game.clone();
	std::thread::spawn(move || main_loop(game_clone));

	for stream in listener.incoming() {
		let stream = stream.unwrap();

		game.connections.entry(stream.peer_addr().unwrap()).or_insert(Connection {
			state: lib::ConnectionState::Handshaking,
			player_name: None,
			player_uuid: None,
		});

		//RX
		println!("New Connection from {}", stream.peer_addr().unwrap());
		let game_clone = game.clone();
		let stream_clone = stream.try_clone().unwrap();
		std::thread::spawn(move || {
			let mut stream = stream_clone;
			let game = game_clone.clone();
			let peer_addr = stream.peer_addr().unwrap();
			loop {
				let mut peek_buf = [0; 1];

				match stream.peek(&mut peek_buf) {
					Ok(0) => {
						println!("client disconnected.");
						disconnect_player(&peer_addr, game.clone());
						break;
					}
					Err(e) => {
						eprintln!("error reading from client: {e}");
						disconnect_player(&peer_addr, game.clone());
						break;
					}
					_ => {}
				}

				let packet = lib::utils::read_packet(&stream);

				// if stream.peer_addr().is_err() {
				//   disconnect_player(&peer_addr, game.clone());
				//   break;
				// }

				let packet_handler_result = packet_handlers::handle_packet(packet, &mut stream, game.clone());
				if packet_handler_result.is_err() {
					println!("got error, so lets disconnect: {}", packet_handler_result.err().unwrap());
					disconnect_player(&peer_addr, game);
					break;
				}
				if let Ok(packet_handler_result) = packet_handler_result
					&& let Some(packet_handler_result) = packet_handler_result
				{
					game.packet_handler_actions.lock().unwrap().push(packet_handler_result);
				}

				std::thread::sleep(std::time::Duration::from_millis(1));
			}
		});


		//TX
		let game_clone = game.clone();
		std::thread::spawn(move || {
			let stream = stream.try_clone().unwrap();
			let game = game_clone.clone();
			let Ok(peer_addr) = stream.peer_addr() else {
				return;
			};

			loop {
				if !game.connections.contains_key(&peer_addr) {
					break;
				}

				let Some(mut queue) = game.packet_send_queues.get_mut(&peer_addr) else {
					continue;
				};
				if !queue.is_empty() {
					let packet = queue.remove(0);
					drop(queue);
					//println!("{}", packet.0);
					if send_packet(&stream, packet.0, packet.1).is_err() {
						return;
					}
				} else {
					drop(queue);
				};

				std::thread::sleep(std::time::Duration::from_millis(1));
			}
		});
	}
}

//copy of this function exists in proxy too
fn send_packet(mut stream: &TcpStream, packet_id: u8, mut data: Vec<u8>) -> Result<(), Box<dyn Error>> {
	let mut serialized_id: Vec<u8> = lib::serialize::varint(packet_id as i32);
	let mut packet: Vec<u8> = lib::serialize::varint((data.len() + serialized_id.len()) as i32);
	packet.append(&mut serialized_id);
	packet.append(&mut data);

	stream.write_all(packet.as_slice())?;
	stream.flush()?;

	return Ok(());
}

fn disconnect_player(peer_addr: &SocketAddr, game: Arc<Game>) {
	let mut players = game.players.lock().unwrap();
	let player_to_remove = players.iter().find(|x| x.peer_socket_address == *peer_addr);
	if let Some(player_to_remove) = player_to_remove {
		player_to_remove.save_to_disk();
		players.iter().for_each(|x| {
			game.send_packet(
				&x.peer_socket_address,
				lib::packets::clientbound::play::PlayerInfoRemove::PACKET_ID,
				lib::packets::clientbound::play::PlayerInfoRemove {
					uuids: vec![player_to_remove.uuid],
				}
				.try_into()
				.unwrap(),
			);

			game.send_packet(
				&x.peer_socket_address,
				lib::packets::clientbound::play::RemoveEntities::PACKET_ID,
				lib::packets::clientbound::play::RemoveEntities {
					entity_ids: vec![player_to_remove.entity_id],
				}
				.try_into()
				.unwrap(),
			);
		});
	}

	game.connections.remove(peer_addr);

	game.packet_send_queues.remove(peer_addr);
	players.retain(|x| x.peer_socket_address != *peer_addr);
}

fn main_loop(game: Arc<Game>) {
	loop {
		let start_time = std::time::Instant::now();

		let tick_timings = tickloop::tick(game.clone());

		let end_time = std::time::Instant::now();
		let tick_duration = end_time - start_time;

		if std::time::Duration::from_millis(50) > tick_duration {
			std::thread::sleep(std::time::Duration::from_millis(50) - tick_duration);
		} else {
			println!("tick took longer than 50ms! It finished in {tick_duration:.2?}\n{tick_timings:?}");
		}
	}
}
