#![allow(clippy::needless_return)]

mod server;

fn main() {
  println!("Starting the oxide server");
  server::initialize_server();
}
