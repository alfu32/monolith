use sha256::digest;
use serde::{Deserialize, Serialize};
use std::str;
use std::boxed::Box;
use std::io::Write;

use crate::{id128_new, Index};
use crate::{DEFAULT_OWNER, DEFAULT_TAG, id128_parse, Record};


impl Record {
    pub fn from_index(index: Index, data: Vec<u8>) -> Self {
        Self {
            id: index.id,
            created: index.created,
            updated: index.updated,
            deleted: index.deleted,
            owner: index.owner,
            tag: index.tag,
            checksum: crate::checksum(&data[..]),
            data: String::from_utf8(data).unwrap(),
        }
    }
    pub fn from_json(serialized: String) -> Self {
        // Deserialize JSON string to struct
        let deserialized: Record = serde_json::from_str(&serialized).unwrap();
        //println!("Deserialized: {:?}", deserialized);
        deserialized
    }
    pub fn from_csv(serialized: String) -> Self {
        let parts = serialized.split(';').collect::<Vec<&str>>();
        let (_,_,_,id) = crate::id128_parse(parts[0]);
        let created = parts[1].parse::<u64>().unwrap();
        let updated = parts[2].parse::<u64>().unwrap();
        let deleted = parts[3].parse::<u64>().unwrap();
        let owner = parts[4].to_string();
        let tag = parts[5].to_string();
        let data = parts[6].to_string();
        let _checksum = if parts.len() >= 8 {
            parts[7].to_string()
        } else {
            crate::checksum(data.as_bytes().clone())
        };
        Self {
            id,
            created,
            updated,
            deleted,
            owner,
            tag,
            data:data.clone(),
            checksum:_checksum,
        }
    }
    pub fn new(data: &[u8]) -> Box<Self> {
        let created = crate::timestamp();
        let (_,_,_,id)=id128_new();
        let s: Self = Self {
            id,
            created,
            updated: 0,
            deleted: 0,
            owner: String::from_utf8(DEFAULT_OWNER.to_vec()).unwrap(),
            tag: str::from_utf8(DEFAULT_TAG).unwrap().into(),
            data: str::from_utf8(data).unwrap().into(),//atob(str::from_utf8(data).unwrap().into()),
            checksum: crate::checksum(data),
        };
        Box::new(s)
    }

    pub fn update(&mut self, data: &[u8]) -> &Self {
        self.updated = crate::timestamp();

        self.checksum = crate::checksum(data);
        self.data = str::from_utf8(data).unwrap().into();//atob(str::from_utf8(data).unwrap().into());
        self
    }

    pub fn delete(&mut self) -> &Self {
        self.deleted = crate::timestamp();
        self
    }
    pub fn to_json(&self) -> String {
        // Serialize struct to JSON string
        let serialized = serde_json::to_string(self).unwrap();
        //println!("Serialized: {}", serialized);
        serialized
    }
    pub fn to_csv(&self) -> String {
        let mut w = Vec::new();
        write!(w, "{};{};{};{};{};{};{};{}",
               self.id,
               self.created, self.updated, self.deleted,
               self.owner, self.tag,
               self.checksum, self.data
        ).expect("TODO: could not serialize Index");
        String::from_utf8(w).unwrap()
    }
    pub fn get_data(&self) -> String {
        self.data.clone()//btoa(self.data.clone())
    }
    pub fn id(&mut self, id128: u128) -> Box<Self> {
        let (_,_,_,id) = id128_parse(id128.to_string().as_str());
        self.id = id;
        Box::new(self.to_owned())
    }
    pub fn created(&mut self, d: u64) -> Box<Self> {
        self.created = d;
        Box::new(self.to_owned())
    }
    pub fn updated(&mut self, d: u64) -> Box<Self> {
        self.updated = d;
        Box::new(self.to_owned())
    }
    pub fn deleted(&mut self, d: u64) -> Box<Self> {
        self.deleted = d;
        Box::new(self.to_owned())
    }
    pub fn tag(&mut self, tag: &[u8]) -> Box<Self> {
        self.tag = String::from_utf8(tag.to_vec()).unwrap();
        Box::new(self.to_owned())
    }
    pub fn own(&mut self, owner: &[u8]) -> Box<Self> {
        self.owner = String::from_utf8(owner.to_vec()).unwrap();
        Box::new(self.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::thread;
    use crate::{checksum, Record};

    #[test]
    fn test_record_creation() {
        let data = String::from("new data");
        let encoded = data.clone();// atob(data.clone());
        let hash = checksum(data.as_bytes());
        let record = Record::new(data.as_bytes()).id(123);

        assert_eq!(record.id, 123);
        assert_eq!(record.data, encoded);
        assert!(record.created > 0);
        assert_eq!(record.updated, 0);
        assert_eq!(record.deleted, 0);

        assert_eq!(record.checksum, hash);
    }

    #[test]
    fn test_record_update() {
        let mut record = Record::new(b"some data").id(123);
        let data = String::from("new data");
        thread::sleep(Duration::new(2, 0));

        record.update(data.as_bytes());

        assert_eq!(record.id, 123);
        assert_eq!(record.data, data);//atob(data));
        assert!(record.updated > record.created);
        assert_eq!(record.deleted, 0);

        let hash = checksum(record.data.as_bytes());//checksum(btoa(record.data).as_bytes());
        assert_eq!(record.checksum, hash);
    }

    #[test]
    fn test_record_delete() {
        let mut record = Record::new(b"some data").id(123);
        thread::sleep(Duration::new(2, 0));

        record.delete();

        assert_eq!(record.id, 123);
        assert!(record.deleted >= record.created);
    }

    #[test]
    fn test_record_serialization_json() {
        let record = Record::new(b"some data").id(123);

        let serialized = record.to_json();
        let deserialized: Record = Record::from_json(serialized);

        assert_eq!(record.id, deserialized.id);
    }

    #[test]
    fn test_record_serialization_csv() {
        let record = Record::new(b"some data").id(123);


        let serialized = record.to_csv();
        println!("serialized:{}", serialized);
        let deserialized: Record = Record::from_csv(serialized);

        assert_eq!(record.id, deserialized.id);
    }

    #[test]
    fn test_record_deserialization_csv() {
        let record = Record::new(b"some data").id(123);


        let serialized = "123;1675335526;0;0;SYSTEM;GLOBAL;3133303739393065366261356361313435656233356539393138326139626563;some data".to_string();
        println!("serialized:{}", serialized);
        let deserialized: Record = Record::from_csv(serialized);
        println!("deserialized:{:#?}", deserialized);
        assert_eq!(record.id, deserialized.id);
    }

    #[test]
    fn test_get_data() {
        let record = Record::new(b"some data").id(123);


        let data = record.get_data();
        println!("data:{}", data);

        assert_eq!(data, String::from("some data"));
    }

    #[test]
    fn test_setters() {
        let record = Record::new(b"some data")
            .id(123)
            .created(11)
            .updated(12)
            .deleted(13)
            .own("GERONIMO".as_bytes())
            .tag("general,users,trace".as_bytes());


        let data = record.get_data();
        println!("data:{}", data);

        assert_eq!(data, String::from("some data"));
    }
}
