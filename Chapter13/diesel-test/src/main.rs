extern crate diesel_test;
extern crate diesel;
extern crate uuid;
#[macro_use] extern crate serde_json;

use diesel::prelude::*;
use diesel_test::*;
use diesel_test::models::Person;
use diesel_test::schema::person::dsl::{person, name, data};
use uuid::Uuid;

fn new_person(new_name: &str) -> Uuid {
    let conn = establish_connection();
    let new_person = create_person(&conn, new_name);
    println!("Saved person: {}", new_person.name);
    new_person.id
}

fn add_data_to_person(uuid: Uuid, person_data: &serde_json::Value) {
    let conn = establish_connection();
    let steven_from_db = person.find(uuid);
    diesel::update(steven_from_db)
        .set(data.eq(person_data))
        .get_result::<Person>(&conn)
        .expect(&format!("Unable to change data on id {}", uuid));
}

fn find_all_with_name(needle: &str) -> Vec<Person> {
    let conn = establish_connection();
    person.filter(name.eq(needle))
        .load::<Person>(&conn)
        .expect(&format!("Error summoning {}", needle))
}

fn main() {
    let stevens_id = new_person("Steven");

    add_data_to_person(stevens_id, &json!({
        "tags": ["dangerous", "polite"]
    }));

    let all_stevens = find_all_with_name("Steven");
    println!("List of all found Stevens:");
    for steven in all_stevens {
        println!("id: {}", steven.id);
        match steven.data {
            Some(d) =>
                println!("data: {}", d),
            None =>
                println!("No data.")
        };
    }
}
