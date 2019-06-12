fn divide(x: f32, y: f32) -> f32 {      // Return type with "->" syntax
    x/y                                 // Note, no "return" keyword needed
}

fn print_sth(x: String) {
    println!("{}", x);
}

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
    // The compiler will detect if something goes wrong, e.g.:
    let x: i32 = 42;

    // Note that we didn't specify any types yet, but I told you before that Rust is
    // a typed language.
    // What happens here is type inference. The compiler automatically detects what the
    // variable type should be (much like "auto" in recent C++ versions)

    // The following won't work, because we are comparing values of different types.
    // Let's force a type conversion
    if 3 as f64 == 3.0 { // << Note that there are no brackets here. Rust is kind of minimalistic
        println!("Hello, world {}!", x);
    }

    let s: String = "Hello".to_string();
    print_sth(s);


    println!("{}", divide(6., 3.));
}
