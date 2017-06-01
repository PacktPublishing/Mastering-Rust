struct Character {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    wisdom: u8,
    intelligence: u8,
    charisma: u8,
    name: String,
}

fn main() {
    let char = Character {
        strength: 9,
        dexterity: 9,
        constitution: 9,
        wisdom: 9,
        intelligence: 9,
        charisma: 9,
        name: "Generic AD&D Hero".to_string(),
    };

    println!("Character's name is {}, and his/her strength is {}",
             char.name,
             char.strength);
}
