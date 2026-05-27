use super::*;

pub fn process(peer_addr: SocketAddr, game: Arc<Game>, players_clone: &[Player]) {
	let mut world = game.world.lock().unwrap();
	let mut players = game.players.lock().unwrap();
	let player = players.iter_mut().find(|x| x.peer_socket_address == peer_addr).unwrap();
	let default_spawn_location = world.default_spawn_location;
	let dimension = world.dimensions.get_mut(player.get_dimension()).unwrap();
	player.respawn(players_clone, dimension, default_spawn_location, &game.packet_sender);
}
