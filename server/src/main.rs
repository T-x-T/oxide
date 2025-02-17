mod server;

fn main() {
  println!("Starting the oxide server");
  server::java::initialize_server();
}