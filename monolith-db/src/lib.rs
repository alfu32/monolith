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
