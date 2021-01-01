pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");
    
    // Basic Formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Joe", "Texas");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    // Placeholder Traits
    println!("Binary: {:b}, Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait using a tuple
    println!("{:?}", (12, true, "hello"));

    println!("10 + 10 = {}", 10 + 10);
}