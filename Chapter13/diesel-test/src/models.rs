extern crate uuid;

use uuid::Uuid;
use serde_json;
use schema::person;

#[derive(Queryable)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Insertable)]
#[table_name="person"]
pub struct NewPerson<'a> {
    pub id: Uuid,
    pub name: &'a str,
}
