extern crate postgres;
extern crate uuid;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate chrono;

use postgres::{Connection, TlsMode};
use uuid::Uuid;
use chrono::DateTime;
use chrono::offset::utc::UTC;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    id: Uuid,
    name: String,
    previous_time_of_lunch: DateTime<UTC>,
    data: Option<serde_json::Value>,
}

fn main() {
    let conn = Connection::connect("postgres://postgres@localhost", TlsMode::None).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS person (
                    id              UUID PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    previous_time_of_lunch  TIMESTAMP WITH TIME ZONE,
                    data            JSONB
                  )", &[]).unwrap();
    let me = Person {
        id: Uuid::new_v4(),
        name: "Steven".to_string(),
        previous_time_of_lunch: UTC::now(),
        data: Some(json!({
            "tags": ["employee", "future_ceo"],
            "contacts": {
                "email": "steven@employer.com"
            }
        }))
    };
    conn.execute("INSERT INTO person (id, name, data, previous_time_of_lunch) VALUES ($1, $2, $3, $4)",
                 &[&me.id, &me.name, &me.data, &me.previous_time_of_lunch]).unwrap();
    for row in &conn.query("SELECT id, name, data, previous_time_of_lunch FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
            previous_time_of_lunch: row.get(3)
        };
        println!("Found person {:?}", person);
        println!("{}'s email: {}", person.name, person.data.unwrap()["contacts"]["email"]);
        println!("{}'s last lunch: {}", person.name, person.previous_time_of_lunch);

    }
}

