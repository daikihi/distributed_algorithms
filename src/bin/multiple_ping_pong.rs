fn main(){
    println!("Running Lamport Clock...");
    let result = distributed_algorithms::ping_pong::multiple_ping_pong::run();
    println!("Result: {:?}", result);
}