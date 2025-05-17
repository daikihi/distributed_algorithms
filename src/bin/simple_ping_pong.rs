fn main() {
    println!("Running Ping Pong...");
    let result = distributed_algorithms::ping_pong::simple_ping_pong::run();
    println!("Result: {:?}", result);
}