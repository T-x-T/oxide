use super::*;

pub fn init(game: &mut Game) {
    game.commands.lock().unwrap().push(Command {
        name: "deop".to_string(),
        permission: Permission::Admin,
        execute,
        arguments: vec![CommandArgument {
            name: "player".to_string(),
            properties: ParserProperty::String(2),
            next_arguments: Vec::new(),
            optional: false
        }]
    });
}

fn execute(command: String, socket_addr: Option<SocketAddr>, game: Arc<Game>) -> Result<(), Box<dyn Error>> {
    let mut players = game.players.lock().unwrap();
    let Some(target_player) = players.iter_mut().find(|x| x.display_name == command.split(" ").nth(1).unwrap_or_default()) else {
		let Some(socket_addr) = socket_addr else {
			println!("Couldn't find that player :(");
			return Ok(());
		};

		game.packet_sender.send_packet_to_player(
			&socket_addr,
			lib::packets::clientbound::play::SystemChatMessage::PACKET_ID,
			lib::packets::clientbound::play::SystemChatMessage {
				content: NbtTag::Root(vec![
					NbtTag::String("type".to_string(), "text".to_string()),
					NbtTag::String("text".to_string(), "Couldn't find that player :(".to_string()),
				]),
				overlay: false,
			},
		);
        
		return Ok(());
	};
    target_player.permission = Permission::Everyone;
    lib::types::config::ops::remove_permission_from_file(target_player.uuid);
    game.packet_sender.send_packet_to_player(
        &target_player.peer_socket_address, 
        lib::packets::clientbound::play::Commands::PACKET_ID, 
        lib::packets::clientbound::play::Commands {
            nodes: crate::command::get_command_packet_data(game.clone(), Permission::Everyone),
            root_index: 0,
        });
    Ok(())
}