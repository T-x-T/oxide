use std::collections::HashMap;
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use crate::types::*;

type CommandExecFn = fn(String, &mut TcpStream, &mut Game, &mut HashMap<SocketAddr, TcpStream>, &mut HashMap<SocketAddr, Connection>) -> Result<(), Box<dyn Error>>;

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
