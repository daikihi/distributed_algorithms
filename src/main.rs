mod ping_pong;
mod lamport_clock;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <mode>", args[0]);
        println!("Modes:");
        println!("  simple   - Run simple ping pong");
        println!("  multiple - Run multiple ping pong");
        return;
    }

    match args[1].as_str() {
        "simple" => ping_pong::simple_ping_pong::run(),
        "multiple" => ping_pong::multiple_ping_pong::run(),
        _ => {
            println!("Invalid mode: {}", args[1]);
            println!("Usage: {} <mode>", args[0]);
            println!("Modes:");
            println!("  simple   - Run simple ping pong");
            println!("  multiple - Run multiple ping pong");
        }
    }

}
