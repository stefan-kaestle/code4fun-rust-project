fn main() {

    // Let's add a variable
    let mut x = 5;

    // Re-assigning a value does not work, we need to add the "mut" keyword
    // Check out the compiler error - it's really helpful
    x = 6;

    // You will see later why the distinction between read/write accesses to variables
    // is so important to Rust and how it is used for thread safety.
    // It allows some extra checks the compiler does that you wouldn't be getting in
    // languages like C++ or Java

    // Now here comes something interesting:
    // Rust allows allocating another variable with the same name, that can even have a
    // different type.
    let x = 42.0;

    // .. and print it
    println!("Hello, world {}!", x);
}
