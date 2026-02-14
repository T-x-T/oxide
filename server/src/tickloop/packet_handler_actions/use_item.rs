use super::*;

pub fn process(peer_addr: SocketAddr, _parsed_packet: lib::packets::serverbound::play::UseItem, game: Arc<Game>) {
	let mut players = game.players.lock().unwrap();
	let player = players.iter_mut().find(|x| x.peer_socket_address == peer_addr).unwrap();

	player.eat();
}
