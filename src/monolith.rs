use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use serde::{Deserialize, Serialize};
use std::path::Path;
use csv::{Reader, Writer};
use crate::record::Record;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Index {
    id: u128,
    created: u64,
    updated: u64,
    deleted: u64,
    owner: String,
    tag: String,
    start: u128,
    end: u128,
}
impl Index {
    pub fn from_csv(serialized:String) -> Self{
        // Deserialize CSV string to struct
        let mut rdr = Reader::from_reader(serialized.as_bytes());
        let deserialized: Vec<Index> = rdr.deserialize().collect::<Result<_, _>>().unwrap();
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

#[derive(Debug, Clone)]
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
        let mut s = Self {
            name: name.to_owned(),
            data_path:data_path.clone(),
            index_path:index_path.clone(),
            index_cache:vec!(),
        };
        s
    }

    pub fn read_all_index( &mut self) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
        let mut index_file = File::create(self.clone().index_path).unwrap();
        let reader = BufReader::new(index_file);
        let mut records = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let record: Index = Index::from_csv(line);
            self.index_cache.push(record);
        }
        Ok(records)
    }
    pub fn read_all(&mut self) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
        let data_file = File::create(self.clone().data_path).unwrap();
        let reader = BufReader::new(data_file);
        let mut records = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let record: Record = Record::from_csv(line);
            records.push(record);
        }
        Ok(records)
    }

    pub fn write(&mut self, record: &mut Record) -> Result<(), Box<dyn std::error::Error>> {
        let mut data_file = File::create(self.clone().data_path).unwrap();
        let mut writer = BufWriter::new(data_file);
        let serialized = record.to_csv();
        let start=File::open(self.clone().data_path).unwrap().metadata().unwrap().len();
        let end = (start as u128) + (serialized.len() as u128);
        let index = Index {
            id: record.id,
            created: record.created,
            updated: record.updated,
            deleted: record.deleted,
            owner: record.clone().owner,
            tag: record.clone().tag,
            start:start.into(),
            end,
        };
        self.index_cache.push(index);
        writer.write_all(serialized.as_bytes()).expect("TODO: panic message writing record");
        writer.write_all(b"\n").expect("TODO: panic message writing record");
        //writer.flush().expect("TODO: panic message flushing data writer");
        //data_file.flush().expect("TODO: panic message flushing data file");
        Ok(())
    }
    pub fn store_index(&mut self){
        let mut index_file = File::create(self.clone().index_path).unwrap();
        let mut index_writer = BufWriter::new(index_file);
        for index in self.index_cache.clone() {
            let index_serialized = index.to_csv();
            index_writer.write_all(index_serialized.as_bytes()).expect("TODO: panic message writing index");
            index_writer.write_all(b"\n").expect("TODO: panic message writing index");
        }
        //index_writer.flush().expect("TODO: panic message flushing index_file");
        //index_file.flush().expect("TODO: panic message flushing index file");
    }
}
