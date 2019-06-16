use std::fs::File;
use std::io::{BufReader, BufRead};
use std::thread;
use std::sync::Arc;

use regex::Regex;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use code4fun::models::*;

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

fn get_employees() {
    use code4fun::schema::employees::dsl::*;
    let conn = code4fun::get_employees();

    let results = employees.limit(5)
        .load::<Employees>(&conn)
        .expect("Could not load employees");

    for r in &results {
        println!("{} {}", r.first_name, r.last_name);
        println!("{:?}", r);
    }
}

fn main() {

    get_employees();
}
