use std::error::Error;
use std::net::TcpStream;
use lib::types::*;

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
