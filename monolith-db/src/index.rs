use serde::{Deserialize, Serialize};
use std::io::Write;
use crate::record::Record;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Index {
    pub id: u128,
    pub created: u64,
    pub updated: u64,
    pub deleted: u64,
    pub owner: String,
    pub tag: String,
    // pub(crate) checksum: String,
    pub start: u64,
    pub end: u64,
}

impl Index {
    pub fn of(record: Record, at_start: u64) -> Index {
        let serialized = record.clone().data.clone();
        let end = at_start + (serialized.len() as u64);
        let index = Index {
            id: record.id,
            created: record.created,
            updated: record.updated,
            deleted: record.deleted,
            owner: record.clone().owner,
            tag: record.clone().tag,
            start: at_start,
            end,
        };
        index
    }
    pub fn from_json(serialized: String) -> Self {
        // Deserialize JSON string to struct
        let deserialized: Index = serde_json::from_str(&serialized).unwrap();
        deserialized
    }
    pub fn from_csv(serialized: String) -> Self {
        let parts = serialized.split(';').collect::<Vec<&str>>();
        let id = parts[0].parse::<u128>().unwrap();
        let created = parts[1].parse::<u64>().unwrap();
        let updated = parts[2].parse::<u64>().unwrap();
        let deleted = parts[3].parse::<u64>().unwrap();
        let owner = parts[4].to_string();
        let tag = parts[5].to_string();
        let start = parts[6].parse::<u64>().unwrap();
        let end = parts[7].parse::<u64>().unwrap();
        Self {
            id,
            created,
            updated,
            deleted,
            owner,
            tag,
            start,
            end,
        }
    }
    pub fn to_csv(&self) -> String {
        let mut w = Vec::new();
        write!(w, "{};{};{};{};{};{};{};{}",
               self.id,
               self.created, self.updated, self.deleted,
               self.owner, self.tag,
               self.start, self.end
        ).expect("TODO: could not serialize Index");
        String::from_utf8(w).unwrap()
    }
    pub fn to_json(&self) -> String {
        // Serialize struct to JSON string
        let serialized = serde_json::to_string(self).unwrap();
        //println!("Serialized: {}", serialized);
        serialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_json() {
        let serialized = r#"{ "id": 123, "created": 456, "updated": 789, "deleted": 159, "owner": "John", "tag": "test", "start": 100, "end": 200 }"#;
        let expected = Index {
            id: 123,
            created: 456,
            updated: 789,
            deleted: 159,
            owner: "John".to_owned(),
            tag: "test".to_owned(),
            start: 100,
            end: 200,
        };
        let result = Index::from_json(serialized.to_owned());
        println!("Deserialized from_json: {:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_csv() {
        let serialized = "123;456;789;159;John;test;100;200";
        let expected = Index {
            id: 123,
            created: 456,
            updated: 789,
            deleted: 159,
            owner: "John".to_owned(),
            tag: "test".to_owned(),
            start: 100,
            end: 200,
        };
        let result = Index::from_csv(serialized.to_owned());
        println!("Deserialized from_csv: {:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_csv() {
        let index = Index {
            id: 123,
            created: 456,
            updated: 789,
            deleted: 159,
            owner: "John".to_owned(),
            tag: "test".to_owned(),
            start: 100,
            end: 200,
        };
        let expected = "123;456;789;159;John;test;100;200";
        let result = index.to_csv();
        println!("Serialized to_csv: {:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_json() {
        let index = Index {
            id: 123,
            created: 456,
            updated: 789,
            deleted: 159,
            owner: "John".to_owned(),
            tag: "test".to_owned(),
            start: 100,
            end: 200,
        };
        let expected = r#"{"id":123,"created":456,"updated":789,"deleted":159,"owner":"John","tag":"test","start":100,"end":200}"#;
        let result = index.to_json();
        println!("Serialized to_json: {:?}", result);
        assert_eq!(result, expected);
    }
}
