pub mod index;
pub mod monolith;
pub mod record;

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
}
