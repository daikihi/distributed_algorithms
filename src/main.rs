mod lamport_clock;
mod ping_pong;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args.clone());

    if args.len() < 2 {
        println!("Usage: {} <mode>", args[0]);
        println!("Modes:");
        println!("  simple   - Run simple ping pong");
        println!("  multiple - Run multiple ping pong");
        println!("  lc - Run lamport clock");
        return;
    }

    match args[1].as_str() {
        "simple" => ping_pong::simple_ping_pong::run(),
        "multiple" => ping_pong::multiple_ping_pong::run(),
        "lc" => lamport_clock::simple_lamport_clock::run(),
        _ => {
            println!("Invalid mode: {}", args[1]);
            println!("Usage: {} <mode>", args[0]);
            println!("Modes:");
            println!("  simple   - Run simple ping pong");
            println!("  multiple - Run multiple ping pong");
            println!("  lc - Run lamport clock");
        }
    }
}
