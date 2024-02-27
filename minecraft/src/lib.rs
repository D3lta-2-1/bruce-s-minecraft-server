use common_server::selector::Selector;

use crate::server::Server;

pub mod connection_handler;
pub mod test;
pub mod packet_read_handler;
pub mod player;
pub mod protocol;
pub mod server;
pub mod session_relay;
pub mod chat;


#[test]
fn test_handshake_server() {
    println!("Server started!");
    let mut server = Server::new();
    let mut selector = Selector::bind("127.0.0.1:25565".parse().unwrap(), 256);
    selector.start_selection_loop(&mut server);
}


