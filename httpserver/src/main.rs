mod handler;
mod router;
mod server;

use server::Server;
fn main() {
    // Start a server
    let server = Server::new("localhost:3000");
    //Run the server
    server.run();
}

//https://livebook.manning.com/book/rust-servers-services-and-apps/chapter-2/v-2/140
