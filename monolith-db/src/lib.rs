use serde::{Deserialize, Serialize};
use std::str;
use serde_json::to_string;
use sha256::digest;

pub mod index;
pub mod monolith;
pub mod record;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Index {
    pub id: String,
    pub created: u64,
    pub updated: u64,
    pub deleted: u64,
    pub owner: String,
    pub tag: String,
    // pub(crate) checksum: String,
    pub start: u64,
    pub end: u64,
}

#[derive(Serialize, Deserialize, /*Copy, */Clone, Debug, PartialEq)]
pub struct Record {
    pub id: String,
    pub created: u64,
    pub updated: u64,
    pub deleted: u64,
    pub owner: String,
    pub tag: String,
    pub data: String,
    pub checksum: String,
}

const DEFAULT_OWNER: &[u8] = b"SYSTEM";
const DEFAULT_TAG: &[u8] = b"GLOBAL";

pub fn checksum(data: &[u8]) -> String {
    let s = digest(data);
    let mut arr = [0u8; 32];
    let bytes = s.as_bytes();
    let len = bytes.len().min(32);
    arr[..len].copy_from_slice(&bytes[..len]);
    hex(&arr)
}

pub fn hex(data: &[u8]) -> String {
    let hex = data.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    // println!("{}", hex);
    hex
}

///// fn atob(data:String) -> String {
/////     encode(data)
///// }
///// fn btoa(data:String) -> String {
/////     let decoded = decode(&data).unwrap();
/////     String::from_utf8(decoded.into()).unwrap()
///// }
pub fn timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn timestamp128() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
pub fn id128_from_number(ts: u128) ->  (u64, u64, u128,String) {
    let id=ts>>64;
    let idl=(ts<<64)>>64;
    (id as u64,idl as u64,ts,ts.to_string())
}
pub fn id128_new() -> (u64, u64, u128,String) {
    id128_from_number(timestamp128())
}

pub fn id128_parse(i128string: &str) -> (u64, u64, u128,String) {
    id128_from_number(i128string.parse::<u128>().unwrap())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io::{Read, Seek, SeekFrom, Write};

    #[test]
    fn write_to_eof() {
        fs::create_dir_all("test-data").expect("couldn't create or access test data dir");
        let mut f: File = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("./test.2.file")
            .unwrap();
        if let Err(e) = writeln!(f, "{}", "hello buddy\nhello world\nhello people\nhello files\nhello myself\n") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    #[test]
    fn read_n_bytes_from_pos_in_file() {
        fs::create_dir_all("test-data").expect("couldn't create or access test data dir");
        let file_path = "./test.2.file";
        let pos = 10;
        let n = 10;

        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Couldn't open file: {}", e);
                return;
            }
        };

        file.seek(SeekFrom::Start(pos as u64)).unwrap();

        let mut buffer = vec![0; n];
        file.read_exact(&mut buffer).unwrap();

        println!("Read {} bytes from position {}: {:?}", n, pos, String::from_utf8(buffer));
    }

    #[test]
    fn test_v8(){
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        let isolate = &mut v8::Isolate::new(Default::default());

        let scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(scope);
        let scope = &mut v8::ContextScope::new(scope, context);

        let code = v8::String::new(scope, "console.log('Hello' + ' World!')").unwrap();
        println!("javascript code: {}", code.to_rust_string_lossy(scope));

        let script = v8::Script::compile(scope, code, None).unwrap();
        let result = script.run(scope).unwrap();
        let result = result.to_string(scope).unwrap();
        println!("result: {}", result.to_rust_string_lossy(scope));
    }
}
