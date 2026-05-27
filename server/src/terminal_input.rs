use lib::packets::Packet;
use lib::types::*;
use std::sync::Arc;

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
				game.packet_sender.send_packet_to_everyone(
					&game.players.lock().unwrap(),
					lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
					lib::packets::clientbound::play::SystemChatMessage {
						content: NbtTag::Root(vec![
							NbtTag::String("type".to_string(), "text".to_string()),
							NbtTag::String("text".to_string(), input.clone().replace("\n", "")),
						]),
						overlay: false,
					},
				);
			}
		}
	});
}
