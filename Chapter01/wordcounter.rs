use std::env;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug)]
struct WordStore(HashMap<String, u64>);

impl WordStore {
    fn new() -> WordStore {
        WordStore(HashMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(&self) {
        for (key, value) in self.0.iter() {
            println!("{}: {}", key, value);
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    println!("args 1 {}", arguments[1]);
    let filename = arguments[1].clone();

    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut word_store = WordStore::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue;
            } else {
                word_store.increment(word);
            }
        }
    }

    word_store.display();
}
