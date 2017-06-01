extern crate serde;
extern crate serde_json;

use std::fmt;

use serde::ser::{Serialize, Serializer};
use serde::de;
use serde::de::{Deserialize, Deserializer, Visitor};

#[derive(Debug)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Serialize for Weekday {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let weekday_number = match *self {
            Weekday::Monday => 0,
            Weekday::Tuesday => 1,
            Weekday::Wednesday => 2,
            Weekday::Thursday => 3,
            Weekday::Friday => 4,
            Weekday::Saturday => 5,
            Weekday::Sunday => 6,
        };

        serializer.serialize_i32(weekday_number)
    }
}

struct WeekdayVisitor;
impl<'de> Visitor<'de> for WeekdayVisitor {
    type Value = Weekday;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
       formatter.write_str("Integer between 0 and 6")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Weekday, E>
        where E: de::Error
    {
        let weekday = match value {
            0 => Weekday::Monday,
            1 => Weekday::Tuesday,
            2 => Weekday::Wednesday,
            3 => Weekday::Thursday,
            4 => Weekday::Friday,
            5 => Weekday::Saturday,
            6 => Weekday::Sunday,
            _ => return Err(E::custom(format!("Number out of weekday range: {}", value))),
        };

        Ok(weekday)
    }
}

impl<'de> Deserialize<'de> for Weekday {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_i32(WeekdayVisitor)
    }
}


fn main() {
    let weekday = Weekday::Tuesday;

    let serialized_json = serde_json::to_string_pretty(&weekday).unwrap();
    println!("Serialized {:?} into {}", weekday, serialized_json);

    let deserialized: Weekday = serde_json::from_str(&serialized_json).unwrap();
    println!("Got back weekday {:?}", deserialized);
}
