use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    worth: f32,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("foo.yml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", &contents);

    let yaml: serde_yaml::Value = serde_yaml::from_str(&contents).unwrap();
    let people: Vec<Person> = serde_yaml::from_value(yaml["people"].clone()).unwrap();

    for person in people {
        println!("Name: {}, Age: {}, Worth: {}", person.name, person.age, person.worth);
    }
    Ok(())
}
