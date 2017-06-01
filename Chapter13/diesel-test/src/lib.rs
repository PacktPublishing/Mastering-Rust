#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_json;
extern crate dotenv;
extern crate uuid;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

use self::models::{Person, NewPerson};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_person<'a>(conn: &PgConnection, name: &'a str) -> Person {
    use schema::person;

    let new_person = NewPerson {
        id: Uuid::new_v4(),
        name: name
    };

    diesel::insert(&new_person).into(person::table)
        .get_result(conn)
        .expect("Error saving person")
}
