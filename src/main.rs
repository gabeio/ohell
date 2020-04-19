mod cards;
mod ohhell;
mod rooms;
mod wsserver;

fn main() {
    wsserver::run_websocket_server();
}
