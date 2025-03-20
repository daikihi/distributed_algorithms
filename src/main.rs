mod ping_pong;

fn main() {
    ping_pong::simple_ping_pong::run();
    println!("switch to multiple ping pong");
    ping_pong::multiple_ping_pong::run();
}
