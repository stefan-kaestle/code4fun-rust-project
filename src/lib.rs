#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use] extern crate serde_derive;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn get_employees() -> MysqlConnection
{
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

    println!("Connection to {} successfully established", database_url);

    conn
}
