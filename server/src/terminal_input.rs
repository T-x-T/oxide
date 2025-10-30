use std::sync::{Arc, Mutex};
use lib::packets::Packet;
use lib::types::*;

pub fn init(game: Arc<Mutex<Game>>) {
	std::thread::spawn(move || {
		loop {
			let mut input = String::new();
			std::io::stdin().read_line(&mut input).unwrap();

			if input.is_empty() {
				continue;
			}

			let mut game = game.lock().unwrap();

			if input.chars().next().unwrap_or_default() == '/' {
				let input = input.chars().skip(1).collect::<String>().replace("\n", "");
				println!("{}", input.split(" ").next().unwrap_or_default());
				let commands = game.commands.lock().unwrap().clone();
				let Some(command) = commands.iter().find(|x| x.name == input.split(" ").next().unwrap_or_default()) else {
	  			println!("command not found");
		    	continue;
	    	};

	    	let _ = (command.execute)(input, None, &mut game);
			} else {
				for player in &game.players {
					lib::utils::send_packet(&player.connection_stream, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
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
