pub fn run() {
    // Print to console
    println!("Hello from the print.rs");

    // Basic Formatting
    println!("{} is from {}", "Jorge", "Lafayette");

    // Postional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Jorge", "Lafayette", "code");

    // Named Arguments
    println!(
    "{name} likes to play {activity}", name = "Jorge", activity = "soccer");

    // Placedholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debugg traits
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}