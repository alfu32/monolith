use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Index {
    id: i128,
    created: i64,
    updated: i64,
    deleted: i64,
    owner: String,
    tag: String,
    start: u64,
    end: u64,
}
impl Index {
    pub fn from_csv(serialized:String) -> Self{
        // Deserialize CSV string to struct
        let mut rdr = Reader::from_reader(serialized.as_bytes());
        let deserialized: Vec<Record> = rdr.deserialize().collect::<Result<_, _>>().unwrap();
        return deserialized[0].clone();
    }
    pub fn to_csv(& self) -> String {
        // Serialize struct to CSV string
        let mut wtr = Writer::from_writer(vec![]);
        wtr.serialize(self).unwrap();
        let serialized = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
        serialized
        
    }
}

pub struct Database {
    name: String,
    data_path: String,
    index_path: String,
    index_cache:Vec<Index>,
}

impl Database {
    pub fn new(name: &str) -> Self {
        let data_path = format!("{}.db", name);
        let index_path = format!("{}.idx", name);
        Self {
            name: name.to_owned(),
            data_path,
            index_path,
            index_cache:vec!(),
        }
    }

    pub fn read_all(&self) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
        let file = File::open(&self.data_path)?;
        let reader = BufReader::new(file);
        let mut records = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let record: Record = serde_json::from_str(&line)?;
            records.push(record);
        }
        Ok(records)
    }

    pub fn write(&mut self, record: &Record) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(&self.data_path)?;
        let index_file = File::create(&self.index_path)?;
        let mut writer = BufWriter::new(file);
        let mut index_writer = BufWriter::new(index_file);
        let serialized = record.to_csv();
        let start=file.metadata().unwrap().len();
        let end = start + serialized.len();
        let index = Index {
            id: record.id,
            created: record.created,
            updated: record.updated,
            deleted: record.deleted,
            owner: record.owner,
            tag: record.tag,
            start,
            end,
        };
        self.;
        let index_serialized = index.to_csv();
        writer.write_all(serialized.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.write_all(index_serialized.as_bytes())?;
        index_writer.write_all(b"\n")?;
        self.update_index(record)?;
        Ok(())
    }
}
