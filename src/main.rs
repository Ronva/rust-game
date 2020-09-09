mod constants;
mod game;
mod net;
mod structs;
mod utils;

fn main() {
  let socket = net::connect_to_server();
  // net::send_to_server(&socket, b"disconnect");

  let _session = game::run(socket);
}
