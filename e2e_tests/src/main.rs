#![allow(clippy::needless_return)]
#![feature(fn_traits)]

mod tests;

use lib::packets::Packet;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};


fn main() {
	let server_address = "127.0.0.1:25565";
	tests::status(server_address);
	tests::join(server_address);
}

fn set_client_up(server_address: &str) -> client_lib::Client {
	let (client, read_queue) = client_lib::Client::connect(server_address, "e2e_test", 150749604613628858574844632630809034217).unwrap();

	let state = client.get_connection_state();
	let mut send_stream = client.get_send_stream_cloned();
	let packet_subscriptions = client.packet_subscriptions.clone();
	std::thread::spawn(move || {
		for packet in read_queue.iter() {
			if crate::handle_packet(packet, state.clone(), &mut send_stream, packet_subscriptions.clone()) {
				drop(read_queue);
				return;
			}
		}
	});

	return client;
}

//returns true, if connection should be terminated
fn handle_packet(
	packet: lib::Packet,
	state: Arc<Mutex<lib::ConnectionState>>,
	send_stream: &mut TcpStream,
	packet_subscriptions: Arc<Mutex<Vec<client_lib::PacketSubscription>>>,
) -> bool {
	if packet.id == 0xFF && packet.length == 0 {
		return false;
	}
	//println!("handle packet with id {}", packet.id);

	let packet_id = packet.id;
	//println!("received packet: {packet_id} {:#04x}", packet_id);

	let current_state = state.lock().unwrap().clone();
	//println!("{current_state:?}");

	packet_subscriptions.lock().unwrap().iter().filter(|x| x.0 == current_state && x.1 == packet_id).for_each(|x| x.2(packet.clone()));

	match current_state {
		lib::ConnectionState::Handshaking => {}
		lib::ConnectionState::Status => {
			if packet_id == lib::packets::clientbound::status::StatusResponse::PACKET_ID {
				return true;
			}
		}
		lib::ConnectionState::Login => {
			if packet_id == lib::packets::clientbound::login::Disconnect::PACKET_ID {
				let parsed_packet = lib::packets::clientbound::login::Disconnect::try_from(packet.data.clone()).unwrap();
				println!("parsed packet: {parsed_packet:?}");
			}
		}
		lib::ConnectionState::Configuration => {
			match packet_id {
				_ => {
					//println!("received unkown packet in clientbound::configuration with id 0x{packet_id:02x}");
				}
			};
		}
		lib::ConnectionState::Play => {
			match packet_id {
				lib::packets::clientbound::play::ClientboundKeepAlive::PACKET_ID => {
					let parsed_packet = lib::packets::clientbound::play::ClientboundKeepAlive::try_from(packet.data.clone()).unwrap();
					client_lib::actually_send_packet(
						send_stream,
						lib::packets::serverbound::play::ServerboundKeepAlive::PACKET_ID,
						lib::packets::serverbound::play::ServerboundKeepAlive {
							keep_alive_id: parsed_packet.keep_alive_id,
						}
						.try_into()
						.unwrap(),
					)
					.unwrap();
				}
				0x6f => (), //update time
				_ => {
					//println!("unkown clientbound packet received with id: 0x{packet_id:02x}");
				}
			};
		}
		lib::ConnectionState::Transfer => {}
	}

	return false;
}
