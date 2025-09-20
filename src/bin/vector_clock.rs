use distributed_algorithms::logical_clock::vector_clock::simple_vector_clock;

fn main() {
    println!("Running Vector Clock...");
    let result = simple_vector_clock::run();
    println!("Result: {:?}", result);
}
