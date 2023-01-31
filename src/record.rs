    use csv::{Writer, Reader};
use sha256::{digest};
use serde::{Deserialize, Serialize};
use std::str;
use base64::{encode,decode};
use std::boxed::{Box};

    #[derive(Serialize, Deserialize, /*Copy, */Clone, Debug)]
    pub struct Record {
        id: u128,
        created: u64,
        updated: u64,
        deleted: u64,
        owner:String,
        tag:String,
        data: String,
        checksum: String,
    }
    const DEFAULT_OWNER:&[u8] = b"SYSTEM";
    const DEFAULT_TAG:&[u8] = b"GLOBAL";

    fn checksum(data:  &[u8]) -> String {
        let s = digest(data);
        let mut arr = [0u8; 32];
        let bytes = s.as_bytes();
        let len = bytes.len().min(32);
        arr[..len].copy_from_slice(&bytes[..len]);
        hex(&arr)
    }
    fn hex(data: &[u8]) -> String {
        let hex = data.iter().map(|b| format!("{:02x}", b)).collect::<String>();
        // println!("{}", hex);
        hex
    }
    fn atob(data:String) -> String {
        encode(data)
    }
    fn btoa(data:String) -> String {
        let decoded = decode(&data).unwrap();
        String::from_utf8(decoded.into()).unwrap()
    }
    fn timestamp() -> u64 {
        std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
    }
    fn timestamp128() -> u128 {
        std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
    }

    impl Record {
        pub fn from_json(serialized:String) -> Self{
            // Deserialize JSON string to struct
            let deserialized: Record = serde_json::from_str(&serialized).unwrap();
            println!("Deserialized: {:?}", deserialized);
            deserialized
        }
        pub fn from_csv(serialized:String) -> Self{
            // Deserialize CSV string to struct
            let mut rdr = Reader::from_reader(serialized.as_bytes());
            let deserialized: Vec<Record> = rdr.deserialize().collect::<Result<_, _>>().unwrap();
            println!("Deserialized: {:?}", deserialized);
            return deserialized[0].clone();
        }
        pub fn new(data: &[u8]) -> Box<Self> {
            let created = timestamp();

            let s: Self = Self {
                id:timestamp128(),
                created,
                updated: created,
                deleted: 0,
                owner:String::from_utf8(DEFAULT_OWNER.to_vec()).unwrap(),
                tag:str::from_utf8(DEFAULT_TAG).unwrap().into(),
                data: atob(str::from_utf8(data).unwrap().into()),
                checksum: checksum(data),
            };
            Box::new(s)
        }

        pub fn update(&mut self, data: &[u8]) -> &Self {
            self.updated = timestamp();

            self.checksum = checksum(data);
            self.data = atob(str::from_utf8(data).unwrap().into());
            self
        }

        pub fn delete(&mut self) -> &Self {
            self.deleted = timestamp();
            self
        }
        pub fn to_json(& self) -> String {
            // Serialize struct to JSON string
            let serialized = serde_json::to_string(self).unwrap();
            println!("Serialized: {}", serialized);
            serialized
        }
        pub fn to_csv(& self) -> String {
            // Serialize struct to CSV string
            let mut wtr = Writer::from_writer(vec![]);
            wtr.serialize(self).unwrap();
            let serialized = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
            println!("Serialized: {}", serialized);
            serialized
            
        }
        pub fn get_data(&self) -> String {
            btoa(self.data.clone())
        }
        pub fn id(&mut self,id: u128) -> Box<Self> {
            self.id=id;
            Box::new(self.to_owned())
        }
        pub fn tag(&mut self,tag:&[u8]) -> Box<Self> {
            self.tag=String::from_utf8(tag.to_vec()).unwrap();
            Box::new(self.to_owned())
        }
        pub fn own(&mut self,owner:&[u8]) -> Box<Self> {
            self.owner=String::from_utf8(owner.to_vec()).unwrap();
            Box::new(self.to_owned())
        }
    }
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn test_record_creation() {
        let data = String::from("new data");
        let encoded = atob(data.clone());
        let hash = checksum(data.as_bytes());
        let record = Record::new(data.as_bytes()).id(123);

        assert_eq!(record.id, 123);
        assert_eq!(record.data, encoded);
        assert_eq!(record.created, record.updated);
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
        assert_eq!(record.data, atob(data));
        assert!(record.updated > record.created);
        assert_eq!(record.deleted, 0);

        let hash = checksum(btoa(record.data).as_bytes());
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
        let deserialized: Record = Record::from_csv(serialized);
    
        assert_eq!(record.id, deserialized.id);
    }
    #[test]
    fn test_get_data() {
        let record = Record::new(b"some data").id(123);
    
    
        let data = record.get_data();
    
        assert_eq!(data, String::from("some data"));
    }
}
