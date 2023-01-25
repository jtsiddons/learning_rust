#[derive(Debug)]
enum Elf {
    Dark(String),
    High(String),
    Orc(String),
    Wood(String),
}

#[derive(Debug)]
struct MagicBonus {
    alteration: u8,
    conjuration: u8,
    destruction: u8,
    illusion: u8,
}

fn set_magic_bonus(race: &Elf) -> MagicBonus {
    match race {
        Elf::Dark(character_name) => {
            println!("Selected: {:?}", race);
            println!("Character name: {}", character_name);
            MagicBonus {
                alteration: 5,
                conjuration: 0,
                destruction: 10,
                illusion: 5,
            }
        },
        Elf::High(character_name) => {
            println!("Selected: {:?}", race);
            println!("Character name: {}", character_name);
            MagicBonus {
                alteration: 5,
                conjuration: 5,
                destruction: 5,
                illusion: 5,
            }
        },
        Elf::Orc(character_name) => {
            println!("Selected: {:?}", race);
            println!("Character name: {}", character_name);
            MagicBonus {
                alteration: 0,
                conjuration: 0,
                destruction: 0,
                illusion: 0,
            }
        },
        Elf::Wood(character_name) => {
            println!("Selected: {:?}", race);
            println!("Character name: {}", character_name);
            MagicBonus {
                alteration: 5,
                conjuration: 0,
                destruction: 0,
                illusion: 5,
            }
        },
    }
}

fn main() {
    let character = Elf::Orc(String::from("Oswald"));

    let bonus = set_magic_bonus(&character);
    println!("{:?}", bonus);
}
