use std::sync::Arc;
use lib::packets::Packet;
use lib::types::*;

pub fn init(game: Arc<Game>) {
	std::thread::spawn(move || {
		loop {
			let mut input = String::new();
			std::io::stdin().read_line(&mut input).unwrap();

			if input.is_empty() {
				continue;
			}

			if input.chars().next().unwrap_or_default() == '/' {
				let input = input.chars().skip(1).collect::<String>().replace("\n", "");
				println!("{}", input.split(" ").next().unwrap_or_default());
				let commands = game.commands.lock().unwrap().clone();
				let Some(command) = commands.iter().find(|x| x.name == input.split(" ").next().unwrap_or_default()) else {
	  			println!("command not found");
		    	continue;
	    	};

	    	let _ = (command.execute)(input, None, game.clone());
			} else {
				for player in game.players.lock().unwrap().iter() {
					game.send_packet(&player.peer_socket_address, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
					  content: NbtTag::Root(vec![
							NbtTag::String("type".to_string(), "text".to_string()),
							NbtTag::String("text".to_string(), input.clone().replace("\n", "")),
						]),
					  overlay: false,
		    	}.try_into().unwrap());
				}
			}
		}
	});
}
