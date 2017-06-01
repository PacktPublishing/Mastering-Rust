struct Character {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    wisdom: u8,
    intelligence: u8,
    charisma: u8,
    name: String,
}

impl Character {
    fn new_named(name: String) -> Character {
        Character {
            strength: 9,
            constitution: 9,
            dexterity: 9,
            wisdom: 9,
            intelligence: 9,
            charisma: 9,
            name: name,
        }
    }

    fn get_strength(&self) -> u8 {
        self.strength
    }
}

fn main() {
    let char = Character::new_named("Joe".to_string());
    println!("Character's strength is {}", char.get_strength());
}
