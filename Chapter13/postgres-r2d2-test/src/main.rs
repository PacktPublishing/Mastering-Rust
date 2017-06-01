extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate uuid;
#[macro_use]
extern crate serde_json;
extern crate chrono;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use chrono::offset::utc::UTC;
use std::{thread, time};
use std::thread::sleep;

#[derive(Debug)]
struct Person {
    name: String,
    previous_time_of_lunch: String,
    data: String
}

type DbConnection = r2d2::PooledConnection<PostgresConnectionManager>;

fn create_table(conn: DbConnection) {
    conn.execute("CREATE TABLE IF NOT EXISTS person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    previous_time_of_lunch  VARCHAR NOT NULL,
                    data            VARCHAR NOT NULL
                  )", &[]).unwrap();
}

fn insert_person(conn: DbConnection) {
    let me = Person {
        name: "Steven".to_string(),
        previous_time_of_lunch: UTC::now().to_string(),
        data: json!({
            "tags": ["employee", "future_ceo"],
            "contacts": {
                "email": "steven@employer.com"
            }
        }).to_string()
    };
    conn.execute("INSERT INTO person (name, data, previous_time_of_lunch) VALUES ($1, $2, $3)",
                 &[&me.name, &me.data, &me.previous_time_of_lunch]).expect("Table creation");
    
}

fn main() {
    let config = r2d2::Config::builder()
        .pool_size(5)
        .build();
    let manager = PostgresConnectionManager::new("postgres://postgres@localhost", TlsMode::None).unwrap();
    let pool = r2d2::Pool::new(config, manager).unwrap();

    {
        let pool = pool.clone();
        thread::spawn(move || {
            let conn = pool.get().unwrap();
            create_table(conn);
            println!("Table creation thread finished.");
        });
    }

    {
        let pool = pool.clone();
        thread::spawn(move || {
            sleep(time::Duration::from_millis(500));
            let conn = pool.get().unwrap();
            insert_person(conn);
            println!("Person insertion thread finished.");
        });
    }

    sleep(time::Duration::from_millis(1000));
    {
        for _ in 0..1024 {
            let pool = pool.clone();
            thread::spawn(move || {
                let conn = pool.get().unwrap();
                for row in &conn.query("SELECT id, name, data, previous_time_of_lunch FROM person", &[]).unwrap() {
                    let person = Person {
                        name: row.get(1),
                        data: row.get(2),
                        previous_time_of_lunch: row.get(3)
                    };
                    println!("Found person {:?}", person);
                }
            });
        }
    }
}


