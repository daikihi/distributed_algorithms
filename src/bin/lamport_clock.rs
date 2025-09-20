fn main() {
    println!("Running Lamport Clock...");
    let result = distributed_algorithms::logical_clock::lamport_clock::simple_lamport_clock::run();
    println!("Result: {:?}", result);
}
