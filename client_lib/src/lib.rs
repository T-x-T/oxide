#![allow(clippy::needless_return)]

use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, channel};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use lib::packets::Packet;

pub type PacketSubscription = (lib::ConnectionState, u8, Box<dyn Fn(lib::Packet) + Send + Sync>);

pub struct Client {
	send_stream: TcpStream,
	read_stream: TcpStream,
	connection_state: Arc<Mutex<lib::ConnectionState>>,
	read_thread_join_handle: JoinHandle<()>,
	username: String,
	uuid: u128,
	pub packet_subscriptions: Arc<Mutex<Vec<PacketSubscription>>>,
}

impl Client {
	pub fn connect(connection_address: &str, username: &str, uuid: u128) -> Result<(Client, Receiver<lib::Packet>), Box<dyn Error>> {
		let send_stream = TcpStream::connect(connection_address)?;
		let read_stream = send_stream.try_clone()?;
		let state = Arc::new(Mutex::new(lib::ConnectionState::Handshaking));
		let packet_subscriptions: Arc<Mutex<Vec<PacketSubscription>>> = Arc::new(Mutex::new(Vec::new()));

		let (tx, rx) = channel::<lib::Packet>();
		//Handle packets coming in on the client side
		let read_stream_clone = read_stream.try_clone()?;
		let join_handle = std::thread::spawn(move || {
			let read_stream = read_stream_clone.try_clone().unwrap();
			loop {
				if tx
					.send(lib::Packet {
						id: 0xFF,
						length: 0,
						data: Vec::new(),
						raw_data: Vec::new(),
					})
					.is_err()
				{
					//println!("couldnt send fake packet down the channel");
					return;
				}


				let mut peek_buf = [0; 1];

				//otherwise peek would block for ever if no new packets arrive
				if read_stream.set_read_timeout(Some(std::time::Duration::from_millis(1))).is_err() {
					return;
				};
				match read_stream.peek(&mut peek_buf) {
					Ok(0) => {
						println!("server disconnected.");
						return;
					}
					Err(e) => {
						if e.to_string() == "Resource temporarily unavailable (os error 35)" {
							continue;
						}
						eprintln!("error reading from server: {e}");
						return;
					}
					_ => {
						//println!("stream still healthy");
					}
				}
				read_stream.set_read_timeout(None).unwrap();

				let packet = lib::utils::read_packet(&read_stream);
				if tx.send(packet).is_err() {
					println!("couldnt send received packet down the channel");
					return;
				}
			}
		});

		let client = Client {
			send_stream,
			read_stream,
			connection_state: state,
			read_thread_join_handle: join_handle,
			username: username.to_string(),
			uuid,
			packet_subscriptions,
		};

		return Ok((client, rx));
	}

	pub fn send_packet(&mut self, packet_id: u8, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
		return actually_send_packet(&mut self.send_stream, packet_id, data);
	}

	pub fn switch_state_to_status(&mut self) {
		*self.connection_state.lock().unwrap() = lib::ConnectionState::Status;
	}

	pub fn switch_state_to_login(&mut self) {
		*self.connection_state.lock().unwrap() = lib::ConnectionState::Login;
	}

	pub fn switch_state_to_configuration(&mut self) {
		*self.connection_state.lock().unwrap() = lib::ConnectionState::Configuration;
	}

	pub fn switch_state_to_play(&mut self) {
		*self.connection_state.lock().unwrap() = lib::ConnectionState::Play;
	}

	pub fn disconnect(self) {
		self.read_stream.shutdown(std::net::Shutdown::Both).unwrap();
		self.read_thread_join_handle.join().unwrap();
	}

	pub fn get_connection_state(&self) -> Arc<Mutex<lib::ConnectionState>> {
		return self.connection_state.clone();
	}

	pub fn update_username(&mut self, username: &str) {
		self.username = username.to_string();
	}

	pub fn update_uuid(&mut self, uuid: u128) {
		self.uuid = uuid;
	}

	pub fn get_send_stream_cloned(&self) -> TcpStream {
		return self.send_stream.try_clone().unwrap();
	}

	pub fn get_username(&self) -> String {
		return self.username.clone();
	}

	pub fn get_uuid(&self) -> u128 {
		return self.uuid;
	}

	pub fn login(&mut self) {
		self
			.send_packet(
				lib::packets::serverbound::handshaking::Handshake::PACKET_ID,
				lib::packets::serverbound::handshaking::Handshake {
					protocol_version: lib::packets::get_protocol_version(),
					server_address: "127.0.0.1".to_string(),
					server_port: 25565,
					next_state: lib::packets::serverbound::handshaking::HandshakeNextStates::Login,
				}
				.try_into()
				.unwrap(),
			)
			.unwrap();

		self.switch_state_to_login();

		self
			.send_packet(
				lib::packets::serverbound::login::LoginStart::PACKET_ID,
				lib::packets::serverbound::login::LoginStart {
					name: self.username.clone(),
					uuid: self.uuid,
				}
				.try_into()
				.unwrap(),
			)
			.unwrap();
		std::thread::sleep(std::time::Duration::from_millis(1000));

		self
			.send_packet(
				lib::packets::serverbound::login::LoginAcknowledged::PACKET_ID,
				lib::packets::serverbound::login::LoginAcknowledged {}.try_into().unwrap(),
			)
			.unwrap();
		std::thread::sleep(std::time::Duration::from_millis(1000));

		self.switch_state_to_configuration();

		self
			.send_packet(
				lib::packets::serverbound::configuration::ServerboundKnownPackets::PACKET_ID,
				lib::packets::serverbound::configuration::ServerboundKnownPackets {
					known_packs: vec![lib::Datapack {
						namespace: "minecraft".to_string(),
						id: "core".to_string(),
						version: lib::packets::get_version_string(),
					}],
				}
				.try_into()
				.unwrap(),
			)
			.unwrap();
		std::thread::sleep(std::time::Duration::from_millis(1000));

		self
			.send_packet(
				lib::packets::serverbound::configuration::AcknowledgeFinishConfiguration::PACKET_ID,
				lib::packets::serverbound::configuration::AcknowledgeFinishConfiguration {}.try_into().unwrap(),
			)
			.unwrap();
		std::thread::sleep(std::time::Duration::from_millis(1000));

		self.switch_state_to_play();
	}
}


pub fn actually_send_packet(stream: &mut TcpStream, packet_id: u8, mut data: Vec<u8>) -> Result<(), Box<dyn Error>> {
	let mut serialized_id: Vec<u8> = lib::serialize::varint(packet_id as i32);
	let mut packet: Vec<u8> = lib::serialize::varint((data.len() + serialized_id.len()) as i32);
	packet.append(&mut serialized_id);
	packet.append(&mut data);

	stream.write_all(packet.as_slice())?;
	stream.flush()?;

	return Ok(());
}
