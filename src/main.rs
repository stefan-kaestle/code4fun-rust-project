use std::fs::File;
use std::io::{BufReader, BufRead};
use std::thread;
use std::sync::Arc;

use regex::Regex;

fn read_file() -> std::io::Result<()> {
    let f = File::open("foo.txt")?;
    let f = BufReader::new(f);

    let mut i = 0;
    let re = Regex::new(r"Benutzer").expect("Malformed regular expression");

    for line in f.lines() {
        let line = line?;

        if re.is_match(&line) {
            i += 1;
        }
    }

    println!("Matches: {} lines", i);

    Ok(())
}

fn divide(x: f32, y: f32) -> Result<f32, String> {      // Return type with "->" syntax
    if y == 0. {
        Err("Don't feel like dividing by 0".to_string())
    } else {
        Ok(x/y)
    }
}

fn print_sth(x: &String) {
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
    let r = &s;
    print_sth(r);

    let handle = {
        let snd_r = Arc::new("Hello world".to_string());
        let thr_r = snd_r.clone();

        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(5));

            // Thanks to the sleep above, this printf will be executed AFTER the original Arc goes out of scope
            println!("From thread: {}", thr_r);
        })

        // snd_r is droppped (freed) here
    };

    handle.join().unwrap();


    println!("{}", divide(6., 3.).unwrap());

    match divide(6.0, 0.0) {
        Ok(v) => println!("{}", v),
        Err(t) => eprintln!("Failed to divide! Error: {}", t)
    }

    read_file().expect("Could not read file");
}
