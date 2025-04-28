use std::collections::HashMap;
use std::net::{TcpListener, SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use lib::ConnectionState;

pub mod packet;

pub fn initialize_server() {
  let listener = TcpListener::bind("0.0.0.0:25565").unwrap();

  let connections: Arc<Mutex<HashMap<SocketAddr, Connection>>> = Arc::new(Mutex::new(HashMap::new()));
  let game: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game {
    players: Vec::new(),
    last_created_entity_id: 0,
  }));

  let connection_streams: Arc<Mutex<HashMap<SocketAddr, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

  for stream in listener.incoming() {
    let stream = stream.unwrap();


    println!("New Connection from {}", stream.peer_addr().unwrap());
    let connections_clone = connections.clone();
    let connection_streams_clone = connection_streams.clone();
    let game_clone = game.clone();
    std::thread::spawn(move || {
      loop {
        let mut stream = stream.try_clone().unwrap();

        let mut peek_buf = [0; 1];

        match stream.peek(&mut peek_buf) {
          Ok(0) => {
            println!("client disconnected.");
            connections_clone.lock().unwrap().remove(&stream.peer_addr().unwrap());
            connection_streams_clone.lock().unwrap().remove(&stream.peer_addr().unwrap());
            let all_players = game_clone.lock().unwrap().players.clone();
            game_clone.lock().unwrap().players.retain(|x| x.peer_socket_address != stream.peer_addr().unwrap());
            packet::handlers::update_players(&mut connection_streams_clone.lock().unwrap(), &mut connections_clone.lock().unwrap(), game_clone.lock().unwrap().players.clone(), Some(all_players.iter().find(|x| x.peer_socket_address == stream.peer_addr().unwrap()).unwrap()));
            break;
          }
          Err(e) => {
            eprintln!("error reading from client: {}", e);
            connections_clone.lock().unwrap().remove(&stream.peer_addr().unwrap());
            connection_streams_clone.lock().unwrap().remove(&stream.peer_addr().unwrap());
            let all_players = game_clone.lock().unwrap().players.clone();
            game_clone.lock().unwrap().players.retain(|x| x.peer_socket_address != stream.peer_addr().unwrap());
            packet::handlers::update_players(&mut connection_streams_clone.lock().unwrap(), &mut connections_clone.lock().unwrap(), game_clone.lock().unwrap().players.clone(), Some(all_players.iter().find(|x| x.peer_socket_address == stream.peer_addr().unwrap()).unwrap()));
            break;
          }
          _ => {}
        }

        let packet = lib::utils::read_packet(&mut stream);
      
        if packet::handlers::handle_packet(packet, &mut stream, &mut connections_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap()) {
          connections_clone.lock().unwrap().remove(&stream.peer_addr().unwrap());
          break;
        }
      }
    });
  }
}

#[derive(Debug, Clone)]
pub struct Game {
  pub players: Vec<Player>,
  pub last_created_entity_id: i32,
}

#[derive(Debug, Clone)]
pub struct Player {
  pub x: f64,
  pub y_feet: f64,
  pub z: f64,
  pub yaw: f32,
  pub pitch: f32,
  pub display_name: String,
  pub uuid: u128,
  pub peer_socket_address: SocketAddr,
  pub entity_id: i32,
  pub waiting_for_confirm_teleportation: bool,
  pub current_teleport_id: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct Connection {
  pub state: ConnectionState,
  pub peer_address: SocketAddr,
  pub player_name: Option<String>,
  pub player_uuid: Option<u128>,
}

impl Default for Connection {
  fn default() -> Self {
    Self {
      state: Default::default(),
      peer_address: SocketAddr::V4(std::net::SocketAddrV4::new(std::net::Ipv4Addr::from_bits(0), 0)),
      player_name: Default::default(),
      player_uuid: Default::default(),
    }
  }
}