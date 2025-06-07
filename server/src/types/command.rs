use std::error::Error;
use std::net::TcpStream;
use lib::types::*;
use lib::packets::Packet;

use crate::types::Game;

type CommandExecFn = fn(command: String, stream: &mut TcpStream) -> Result<(), Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Command {
	pub name: String,
	pub execute: CommandExecFn,
	pub arguments: Vec<CommandArgument>,
}

#[derive(Debug, Clone)]
pub struct CommandArgument {
	pub name: String,
	pub properties: ParserProperty,
	pub next_arguments: Vec<CommandArgument>,
	pub optional: bool,
}

pub fn get_command_packet_data(game: &Game) -> Vec<CommandNode> {
	let root_node = CommandNode {
		flags: 0,
    children: Vec::new(),
    redirect_node: None,
    name: None,
    properties: None,
    suggestions_type: None,
  };

	let mut nodes = vec![root_node];

	for command in &game.commands {
		nodes.push(CommandNode {
			flags: if command.arguments.is_empty() { 1 } else { 5 },
			children: Vec::new(),
			redirect_node: None,
			name: Some(command.name.clone()),
			properties: None,
			suggestions_type: None,
		});

		let command_index = nodes.len() - 1;

		nodes[0].children.push(command_index as i32);

		process_arguments(&command.arguments, &mut nodes, command_index);
	};
	println!("{nodes:?}");
	return nodes;
}

fn process_arguments(arguments: &Vec<CommandArgument>, nodes: &mut Vec<CommandNode>, command_index: usize) {
	for argument in arguments {
		let is_executable = argument.next_arguments.is_empty() || argument.next_arguments.iter().any(|x| x.optional);
		nodes.push(CommandNode {
			flags: if is_executable { 6 } else { 2 },
			children: Vec::new(),
			redirect_node: None,
			name: Some(argument.name.clone()),
			properties: Some(argument.properties.clone()),
			suggestions_type: None,
		});

		let argument_index = nodes.len() - 1;

		nodes[command_index].children.push(argument_index as i32);

		if !argument.next_arguments.is_empty() {
			process_arguments(&argument.next_arguments, nodes, argument_index);
		}
	}
}

pub fn init(game: &mut Game) {
	game.commands.push(Command {
		name: "ping".to_string(),
		execute: execute_ping,
		arguments: vec![
			CommandArgument {
				name: "message".to_string(),
				properties: ParserProperty::String(1),
				next_arguments: Vec::new(),
				optional: true,
			},
			CommandArgument {
				name: "first_arg".to_string(),
				properties: ParserProperty::String(1),
				next_arguments: vec![
					CommandArgument {
						name: "second_arg".to_string(),
						properties: ParserProperty::Bool,
						next_arguments: vec![
							CommandArgument {
								name: "third_arg".to_string(),
								properties: ParserProperty::Gamemode,
								next_arguments: Vec::new(),
								optional: true,
							}
						],
						optional: false,
					}
				],
				optional: false,
			},
		],
	});

	game.commands.push(Command {
		name: "hi".to_string(),
		execute: execute_hi,
		arguments: Vec::new(),
	});
}

fn execute_ping(command: String, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
	let reply_msg = if command.as_str() == "ping" {
    	"pong".to_string()
    } else {
   		command.replace("ping ", "")
    };

  	lib::utils::send_packet(stream, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
			  content: NbtTag::TagCompound(None, vec![
				NbtTag::String(Some("type".to_string()), "text".to_string()),
				NbtTag::String(Some("text".to_string()), reply_msg),
			]),
		  overlay: false,
   	}.try_into()?)?;

	return Ok(());
}

fn execute_hi(_command: String, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
  	lib::utils::send_packet(stream, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
			  content: NbtTag::TagCompound(None, vec![
				NbtTag::String(Some("type".to_string()), "text".to_string()),
				NbtTag::String(Some("text".to_string()), "Hi back :)".to_string()),
			]),
		  overlay: true,
   	}.try_into()?)?;

	return Ok(());
}
