use std::{collections::HashMap, net::{SocketAddr, TcpStream}, sync::{Arc, Mutex}};
use lib::packets::Packet;
use lib::types::*;

pub fn init(connection_streams: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>, game: Arc<Mutex<Game>>, connections: Arc<Mutex<HashMap<SocketAddr, Connection>>>) {
	std::thread::spawn(move || {
		loop {
			let mut input = String::new();
			std::io::stdin().read_line(&mut input).unwrap();

			if input.is_empty() {
				continue;
			}
			let mut connection_streams = connection_streams.lock().unwrap();
			let mut connections = connections.lock().unwrap();
			let mut game = game.lock().unwrap();

			if input.chars().next().unwrap_or_default() == '/' {
				let input = input.chars().skip(1).collect::<String>().replace("\n", "");
				println!("{}", input.split(" ").next().unwrap_or_default());
				let Some(command) = game.commands.iter().find(|x| x.name == input.split(" ").next().unwrap_or_default()) else {
	  			println!("command not found");
		    	continue;
	    	};

	    	let _ = (command.execute)(input, None, &mut game, &mut connection_streams, &mut connections);
			} else {
				for stream in connection_streams.iter() {
					lib::utils::send_packet(stream.1, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
					  content: NbtTag::Root(vec![
							NbtTag::String("type".to_string(), "text".to_string()),
							NbtTag::String("text".to_string(), input.clone().replace("\n", "")),
						]),
					  overlay: false,
		    	}.try_into().unwrap()).unwrap();
				}
			}
		}
	});
}
