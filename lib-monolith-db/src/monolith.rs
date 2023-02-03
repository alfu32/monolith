use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use serde::{Deserialize, Serialize};
use crate::index::Index;
use crate::record::Record;

    #[derive(Debug, Clone)]
    pub struct MonolithBackend {
        pub(crate) name: String,
        data_path: String,
        /// index_file: File,
        /// data_file: File,
        index_path: String,
        pub(crate) index_cache: Vec<Index>,
    }

    impl MonolithBackend {
        pub fn open(base_path: &str, name: &str) -> Self {
            let data_path = format!("{}/{}.db", base_path, name);
            let index_path = format!("{}/{}.idx", base_path, name);
            fs::create_dir_all(base_path).expect("couldn't create or access data dir");

            let _data_file = match File::open(data_path.clone()) {
                Ok(_file) => {
                    //println!("found database file {data_path}");
                    _file
                }
                Err(_) => {
                    //println!("creating database file {data_path}");
                    File::create(data_path.clone()).expect("Could not create Database File")
                }
            };
            let _index_file = match File::open(index_path.clone()) {
                Ok(_file) => {
                    //println!("found database index file {index_path}");
                    _file
                }
                Err(_) => {
                    //println!("creating database index file {index_path}");
                    File::create(index_path.clone()).expect("Could not create Database Index File")
                }
            };
            let mut s = Self {
                name: name.to_owned(),
                data_path: data_path.clone(),
                /// data_file:_data_file,
                index_path: index_path.clone(),
                /// index_file:_index_file,
                index_cache: vec!(),
            };
            s.read_all_index().expect("TODO: panic message @ Database::new ..init");
            //println!("initializing database object {s:#?}");
            s
        }
        pub fn close(&mut self) {
            self.write_all_index();
        }

        fn read_all_index(&mut self) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
            let index_file = File::open(self.clone().index_path).unwrap();
            let reader = BufReader::new(index_file);
            let records = Vec::new();
            if File::open(self.clone().index_path).unwrap().metadata().unwrap().len() > 0 {
                for line in reader.lines() {
                    let line = line?;
                    let record: Index = Index::from_csv(line);
                    self.index_cache.push(record);
                }
            }
            Ok(records)
        }
        fn write_all_index(&mut self) {
            let mut index_file = OpenOptions::new()
                .write(true)
                .append(false)
                .truncate(true)
                .create(true)
                .open(self.clone().index_path)
                .unwrap();
            for index in self.index_cache.clone() {
                let index_serialized = index.to_csv();
                writeln!(index_file, "{}", index_serialized).expect("TODO: couldnt write index file");
            }
        }

        pub fn get_from_index(&self, p0: fn(&Index) -> bool) -> Vec<Index> {
            self.index_cache.clone().into_iter().filter(p0)
                .collect::<Vec<Index>>()
        }
        pub fn read_record_at_index(&self, ix: Index) -> Result<Record, Box<dyn std::error::Error>> {
            let mut data_file = match File::open(self.clone().data_path.clone()) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Couldn't open file: {}", e);
                    return Err(e.into());
                },
            };
            let index = ix;
            let n = (index.end - index.start) as usize;
            let mut data = vec![0u8; n];
            data_file.seek(SeekFrom::Start(index.start as u64)).unwrap();
            data_file.read_exact(&mut data).unwrap();
            let record: Record = Record::from_index(index.clone(), data);
            Ok(record)
        }
        pub fn read_all_matching(&self, p0: fn(&Index) -> bool) -> Vec<Record> {
            self.index_cache.clone().into_iter()
                .filter(p0)
                .map(|ix| {
                    self.read_record_at_index(ix).unwrap()
                })
                .collect::<Vec<Record>>()
        }
        // fn always_true(crt:&Index,_prev:&Index,_prev_rec:&Record) -> bool { true }
        pub fn read_all(&self) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
            match File::open(self.clone().data_path.clone()) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Couldn't open file: {}", e);
                    return Err(e.into());
                },
            };
            let mut records = vec![];
            // let mut reader = BufReader::new(File::open(self.clone().data_path).unwrap());
            for ix in self.index_cache.clone() {
                match self.read_record_at_index(ix) {
                    Ok(r) => {
                        records.push(r);
                    }
                    Err(e) => {
                        return Err(e.into())
                    }
                }
            }
            Ok(records)
        }

        pub fn write(&mut self, r: Record) -> Result<(), Box<dyn std::error::Error>> {
            let record = r.clone();
            let start = File::open(self.clone().data_path).unwrap().metadata().unwrap().len();

            let serialized = record.clone().data.clone();
            let index = Index::of(record, start);
            self.index_cache.push(index);

            let mut data_file: File = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(self.clone().data_path)
                .unwrap();
            //println!("WRITING {} TO {}",serialized,self.clone().data_path);
            if let Err(e) = writeln!(data_file, "{}", serialized) {
                eprintln!("Couldn't write [{}] to file: {}", serialized, e);
            }
            Ok(())
        }
    }
#[cfg(test)]
mod tests {
    use crate::index::lib_monolith::Index;
    use crate::monolith::lib_monolith::MonolithBackend;
    use crate::record::lib_monolith::Record;
    use super::*;

    #[test]
    fn test_write_read() {
        let mut db = MonolithBackend::open("test-data", "test_write_read");
        let record = Record::new("some nice data".as_bytes()).id(123);
        db.write(*record.clone()).unwrap();
        let read_records = db.read_all().unwrap();
        let first = read_records[0].clone();
        assert_eq!(*record.tag, first.tag);
        assert_eq!(*record.owner, first.owner);
        assert_eq!(*record.data, first.data);
        assert_eq!(*record.checksum, first.checksum);
        db.write_all_index();
    }

    #[test]
    fn test_read_nonexistent_record() {
        let mut db = MonolithBackend::open("test-data", "test_read_nonexistent_record");
        let read_result = db.read_all();
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap().len(), 0);
        db.write_all_index();
    }
    #[test]
    fn test_read_where() {
        let mut db = MonolithBackend::open("test-data", "test_read_where");
        let record1 = Record {
            id: 1,
            created: 1,
            updated: 2,
            deleted: 3,
            owner: "aaaa".to_string(),
            tag: "vvvv".to_string(),
            data: "some nice data".to_string(),
            checksum: "6534356631393430343933323463313763313534623030636439393838386537".to_owned(),
        };
        let record2 = Record {
            id: 2,
            created: 4,
            updated: 5,
            deleted: 6,
            owner: "aaaa".to_string(),
            tag: "vvvv".to_string(),
            data: "some nice data".to_string(),
            checksum: "6534356631393430343933323463313763313534623030636439393838386537".to_owned(),
        };
        db.write(record1.clone()).unwrap();
        db.write(record2.clone()).unwrap();
        let result = db.read_all().unwrap();
        assert_eq!(result[0], record1);
        db.write_all_index();
    }
    #[test]
    fn test_read_where_no_matches() {
        let mut db = MonolithBackend::open("test-data", "test_read_where_no_matches");
        let record1 = Record {
            id: 1,
            created: 1,
            updated: 2,
            deleted: 3,
            owner: "aaaa".to_string(),
            tag: "vvvv".to_string(),
            data: "some nice data".to_string(),
            checksum: "6534356631393430343933323463313763313534623030636439393838386537".to_owned(),
        };
        let record2 = Record {
            id: 2,
            created: 4,
            updated: 5,
            deleted: 6,
            owner: "aaaa".to_string(),
            tag: "vvvv".to_string(),
            data: "some nice data".to_string(),
            checksum: "6534356631393430343933323463313763313534623030636439393838386537".to_owned(),
        };
        db.write(record1.clone()).unwrap();
        db.write(record2.clone()).unwrap();
        let result = db.read_all().unwrap();
        assert_eq!(result[0], record1);
        assert_eq!(result[1], record2);
        db.write_all_index();
    }
    #[test]
    fn test_is_index_fully_read(){
        let mut db = MonolithBackend::open("test-data", "test_is_index_fully_read");
        for i in 1..10u128 {
            let record = Record::new("some nice data".as_bytes()).id(i*10);
            db.write(*record).unwrap();
        }
        db.write_all_index();
        let mut db = MonolithBackend::open("test-data", "test_is_index_fully_read");
        for ix in db.index_cache.clone() {
            println!("Index {:#?}",ix)
        }
        db.write_all_index();
    }
    #[test]
    fn test_db_close(){
        let mut db = MonolithBackend::open("test-data", "test_db_close");
        for i in 1..10u128 {
            let record = Record::new("some nice data test_db_close".as_bytes()).id(i*10);
            db.write(*record).unwrap();
        }
        db.close();
        let mut db = MonolithBackend::open("test-data", "test_db_close");
        db.index_cache.iter().for_each(|x| println!("{:#?}", x.to_csv()));

        db.close();
    }
    #[test]
    fn test_filter_index(){
        let mut db = MonolithBackend::open("test-data", "test_filter_index");
        for i in 1..10u128 {
            let record = Record::new("some nice data test_filter_indexto test filter_index".as_bytes()).id(i*10);
            db.write(*record).unwrap();
        }
        db.close();
        let mut db = MonolithBackend::open("test-data", "test_filter_index");
        let result:Vec<Index> = db.get_from_index(|x:&Index| x.id==30);
        result.iter().for_each(|x| println!("{:#?}", x.to_csv()));

        db.close();
    }
    #[test]
    fn test_read_index_filtered(){
        let mut db = MonolithBackend::open("test-data", "test_read_index_filtered");
        for i in 1..10u128 {
            let record = Record::new("some nice data test_read_index_filtered test filter_index".as_bytes()).id(i*10);
            db.write(*record).unwrap();
        }
        db.close();
        let mut db = MonolithBackend::open("test-data", "test_read_filtered");
        let result:Vec<Index> = db.get_from_index(|x:&Index| x.id==30);
        result.iter().for_each(|x| println!("{:#?}", x.to_csv()));

        db.close();
    }
    #[test]
    fn test_read_records_filtered(){
        let mut db = MonolithBackend::open("test-data", "test_read_records_filtered");
        for i in 1..10u128 {
            let record = Record::new("some nice data test_read_records_filtered test filter_index".as_bytes()).id(i*10);
            db.write(*record).unwrap();
        }
        db.close();
        let mut db = MonolithBackend::open("test-data", "test_read_records_filtered");
        let result:Vec<Record> = db.read_all_matching(|x:&Index| x.id==30);
        result.iter().for_each(|x| println!("{:#?}", x.to_csv()));

        db.close();
    }
    #[test]
    fn test_use_db_name(){
        let db = MonolithBackend::open("test-data", "test_use_db_name");
        println!("Opened Monolith Backend {}",db.name);
        assert_eq!("test_use_db_name",db.name);
    }
}

