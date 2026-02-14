use super::*;

pub fn process(peer_addr: SocketAddr, game: Arc<Game>, players_clone: &[Player]) {
	let mut world = game.world.lock().unwrap();
	let mut players = game.players.lock().unwrap();
	let player = players.iter_mut().find(|x| x.peer_socket_address == peer_addr).unwrap();

	player.respawn(game.clone(), players_clone, &mut world);
}
