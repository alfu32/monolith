mod record;
mod monolith;

use serde::{Deserialize, Serialize};
use csv::{Reader, Writer};

use crate::record::Record;



#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    let user = User {
        name: String::from("John Doe"),
        age: 30,
        email: String::from("johndoe@example.com"),
    };

    // Serialize struct to JSON string
    let serialized = serde_json::to_string(&user).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize JSON string to struct
    let deserialized: User = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);

    // Serialize struct to CSV string
    let mut wtr = Writer::from_writer(vec![]);
    wtr.serialize(user).unwrap();
    let serialized = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize CSV string to struct
    let mut rdr = Reader::from_reader(serialized.as_bytes());
    let deserialized: Vec<User> = rdr.deserialize().collect::<Result<_, _>>().unwrap();
    println!("Deserialized: {:?}", deserialized);

    let record = Record::new(b"Peppa Pig").tag(b"series").own(b"children");

    let serialized = record.to_json();
    let deserialized: Record = Record::from_json(serialized);

    let record = Record::new(b"Paw Patrol").tag(b"series").own(b"children");


    let serialized = record.to_csv();
    let deserialized: Record = Record::from_csv(serialized);
}