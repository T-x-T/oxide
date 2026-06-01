use super::*;

pub fn init(game: &mut Game) {
    game.commands.lock().unwrap().push(Command {
        name: "op".to_string(),
        permission: Permission::Admin,
        execute,
        arguments: vec![CommandArgument {
            name: "player".to_string(),
            properties: ParserProperty::String(2),
            next_arguments: vec![CommandArgument {
                name: "level".to_string(),
                properties: ParserProperty::Integer(2, Some(0), Some(4)),
                next_arguments: Vec::new(),
                optional:true,
            }],
            optional: false,
        }],
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
    let new_permission = match command.split(" ").nth(2).unwrap_or("nil") {
        "0" => Permission::Operator,
        "1" => Permission::Moderator,
        "2" => Permission::Gamemaster,
        "3" => Permission::Admin,
        "4" => Permission::Owner,
        _ => Permission::Owner,
    };
    target_player.permission = new_permission;
    game.packet_sender.send_packet_to_player(
        &target_player.peer_socket_address,
        lib::packets::clientbound::play::Commands::PACKET_ID,
        lib::packets::clientbound::play::Commands {
            nodes: crate::command::get_command_packet_data(game.clone(), new_permission),
            root_index: 0
        }
    );
    Ok(())
}