#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate toml;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Foobar(String, f32);

#[derive(Serialize, Deserialize, Debug)]
struct DataContainer {
    #[serde(rename="SHORT NUMBER")]
    short_number: u8,
    long_number: u64,
    signed_number: i32,

    string_number_pairs: HashMap<String, u32>,
    // list_of_foobars: Vec<Foobar>,
}

fn main() {
    let mut hm: HashMap<String, u32> = HashMap::new();
    hm.insert("derp".into(), 5);
    hm.insert("meep".into(), 12873);

    let foobar1 = Foobar("diip".into(), 41.12312398);
    let foobar2 = Foobar("moop".into(), 41.0);

    let dc = DataContainer {
        short_number: 43,
        long_number: 126387213,
        signed_number: -12367,

        string_number_pairs: hm,
       // list_of_foobars: vec![foobar1, foobar2],
    };

    let serialized_json = serde_json::to_string_pretty(&dc).unwrap();
    let serialized_toml = toml::to_string(&dc).unwrap();
    println!("Serialized JSON:\n{}", serialized_json);
    println!("Serialized TOML:\n{}", serialized_toml);

    let dc2: DataContainer = serde_json::from_str(&serialized_json).unwrap();
    let dc3: DataContainer = toml::from_str(&serialized_toml).unwrap();
}
