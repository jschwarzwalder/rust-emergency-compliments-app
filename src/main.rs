extern crate simple_server;

use simple_server::Server;

fn main() {
    let server = Server::new(|request, mut response| {
        Ok(response.body("Hello, world!".as_bytes().to_vec())?)
    });

    server.listen("localhost", "4000");
}
