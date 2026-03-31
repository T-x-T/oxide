use lib::packets::Packet;

pub fn status(server_address: &str) {
	println!("run test status");
	let mut client = crate::set_client_up(server_address);

	client
		.send_packet(
			lib::packets::serverbound::handshaking::Handshake::PACKET_ID,
			lib::packets::serverbound::handshaking::Handshake {
				protocol_version: lib::packets::get_protocol_version(),
				server_address: server_address.to_string(),
				server_port: 25565,
				next_state: lib::packets::serverbound::handshaking::HandshakeNextStates::Status,
			}
			.try_into()
			.unwrap(),
		)
		.unwrap();

	client.switch_state_to_status();

	client.packet_subscriptions.lock().unwrap().push((
		lib::ConnectionState::Status,
		lib::packets::clientbound::status::StatusResponse::PACKET_ID,
		Box::new(|packet| {
			let parsed_packet = lib::packets::clientbound::status::StatusResponse::try_from(packet.data).unwrap();
			if parsed_packet.status.contains(lib::packets::get_protocol_version().to_string().as_str())
				&& parsed_packet.status.contains(lib::packets::get_version_string().as_str())
			{
				println!("test passed");
			} else {
				println!("test failed");
			}
		}),
	));
	client
		.send_packet(
			lib::packets::serverbound::status::StatusRequest::PACKET_ID,
			lib::packets::serverbound::status::StatusRequest {}.try_into().unwrap(),
		)
		.unwrap();

	std::thread::sleep(std::time::Duration::from_millis(100));
	client.disconnect();
}

pub fn join(server_address: &str) {
	println!("run test join");
	let mut client = crate::set_client_up(server_address);

	let username = client.get_username();
	let uuid = client.get_uuid();
	client.packet_subscriptions.lock().unwrap().push((
		lib::ConnectionState::Login,
		lib::packets::clientbound::login::LoginSuccess::PACKET_ID,
		Box::new(move |packet| {
			let parsed_packet = lib::packets::clientbound::login::LoginSuccess::try_from(packet.data).unwrap();
			if parsed_packet.username == username && parsed_packet.uuid == uuid {
				println!("test passed");
			} else {
				println!("test failed");
			}
		}),
	));

	client.login();

	std::thread::sleep(std::time::Duration::from_millis(100));
	client.disconnect();
}
