#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sql_types::Datetime;

pub mod schema;
pub mod models;
