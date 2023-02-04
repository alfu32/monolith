use std::borrow::BorrowMut;
use monolith_db::monolith::MonolithBackend;
use monolith_db::record::Record;
use monolith_db::index::Index;

use std::env;
use std::string::ToString;
use clap::Parser;
use std::io::Write;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// the database
    dbname: std::path::PathBuf,
    /// The operation
    operation: String,
    /// The path to the file to read
    /// The path to the file to read
    data: String,
}

const OP_READ: &str = "read";
const OP_DELETE: &str = "delete";
const OP_CREATE: &str = "create";
const OP_WRITE: &str = "write";
const OP_READ_ALL: &str = "read-all";
const OP_WRITE_FROM_FILE: &str = "import";
const AVAILABLE_OPS: [&str; 6] = [
    OP_READ,
    OP_DELETE,
    OP_CREATE,
    OP_WRITE,
    OP_READ_ALL,
    OP_WRITE_FROM_FILE,
];

fn main() -> Result<(), i32> {
    let _args: Cli = Cli::parse();
    let mut db = MonolithBackend::open(env::current_dir().unwrap().to_str().unwrap(), _args.dbname.to_str().unwrap());
    match _args.operation.as_str() {
        OP_READ => {
            println!("operation {}", _args.operation);
            let result = db.read_all().unwrap();
            let id: u128 = _args.data.parse::<u128>().unwrap();

            result.iter().filter(|r| r.id == id).for_each(|x| println!("{:#?}", x.to_json()));
        }
        OP_DELETE => {
            println!("operation {}", _args.operation);

            let result = db.read_all().unwrap();
            let id: u128 = _args.data.parse::<u128>().unwrap();

            result.iter().filter(|r| r.id == id)
                .for_each(|x| {
                    let mut y =x.clone();
                    y.delete();
                    db.write(y.clone()).expect("Could not delete record ");//+x.to_json().as_str()+".");
                    println!("deleted {:#?}", y.to_json()) });
        }
        OP_CREATE => {
            // println!("operation {}",_args.operation);
            let rr = Record::new(_args.data.as_bytes()).tag("system".as_bytes());
            db.write(*rr.clone());
            println!("{:#?}", rr.id)
        }
        OP_WRITE => {
            let record=Record::from_csv(_args.data);
            db.write(record.clone()).expect("could not write record");
            println!("operation {} record {}", _args.operation,record.to_json());
        }
        OP_READ_ALL => {
            let result = db.read_all().unwrap();
            match _args.data.as_str() {
                "--json" => {
                    result.iter().for_each(|x| println!("{}", x.to_json()));
                },
                _ => {
                    println!("id;created;updated;deleted;owner;tag;data;checksum");
                    result.iter().for_each(|x| println!("{}", x.to_csv()));
                }
            }
        }
        OP_WRITE_FROM_FILE => {
            println!("operation {}", _args.operation);
        }
        _ => {
            println!("available operations {:#?}", AVAILABLE_OPS);
            return Err(-2);
        }
    }
    db.close();

    // println!("data {}",_args.data);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_args() {
        let args = Cli::parse();
    }
}
