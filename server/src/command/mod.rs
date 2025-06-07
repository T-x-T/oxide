mod ping;
mod hi;
mod tp;

use crate::types::*;
use std::net::TcpStream;
use std::error::Error;
use lib::packets::Packet;

pub fn init(game: &mut Game) {
	ping::init(game);
	hi::init(game);
	tp::init(game);
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
