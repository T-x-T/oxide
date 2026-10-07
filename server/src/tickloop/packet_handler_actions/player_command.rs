use super::*;

pub fn process(
	peer_addr: SocketAddr,
	game: Arc<Game>,
	players_clone: &[Player],
	parsed_packet: lib::packets::serverbound::play::PlayerCommand,
) {
	if matches!(parsed_packet.action_id, lib::packets::serverbound::play::PlayerCommandAction::LeaveBed) {
		let mut players = game.players.lock().unwrap();
		let world = game.world.lock().unwrap();
		let player = players.iter_mut().find(|x| x.peer_socket_address == peer_addr).unwrap();
		let dimension = world.dimensions.get(player.get_dimension()).unwrap();
		player.set_is_sleeping(players_clone, &game.packet_sender, false, None, dimension).unwrap();
	}
}
