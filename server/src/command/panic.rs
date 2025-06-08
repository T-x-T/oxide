use std::{collections::HashMap, net::SocketAddr, process};

use super::*;

pub fn init(game: &mut Game) {
	game.commands.push(Command {
		name: "panic".to_string(),
		execute,
		arguments: Vec::new(),
	});
}

fn execute(_command: String, _stream: &mut TcpStream, _game: &mut Game, _connection_streams: &mut HashMap<SocketAddr, TcpStream>, _connections: &mut HashMap<SocketAddr, Connection>) -> Result<(), Box<dyn Error>> {
	eprintln!("panic command was used");
	process::exit(1000);
}
