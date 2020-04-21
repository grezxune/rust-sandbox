pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is from {}", "Tommy", "Wisconsin");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Tommy", "Wisconsin", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Tommy", activity = "Overwatch");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}