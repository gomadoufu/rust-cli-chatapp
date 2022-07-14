mod server;

fn main() {
    let addr = "localhost:8080";
    server::start_server(addr);
}
