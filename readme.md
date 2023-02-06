# Intent
(_written by me_)

I'm giving you the abriged version of a new database engine written in rust :

```rust
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
  pub fn open(base_path: &str, name: &str) -> Self
  pub fn close(&mut self)
  fn read_all_index(&mut self) -> Result<Vec<Record>, Box<dyn std::error::Error>>
  fn write_all_index(&mut self)
  pub fn get_from_index(&self, p0: fn(&Index) -> bool) -> Vec<Index>
  pub fn read_record_at_index(&self, ix: Index) -> Result<Record, Box<dyn std::error::Error>>
  pub fn read_all_matching(&self, p0: fn(&Index) -> bool) -> Vec<Record>
  pub fn read_all(&self) -> Result<Vec<Record>, Box<dyn std::error::Error>>
  pub fn write(&mut self, r: Record) -> Result<(), Box<dyn std::error::Error>>
}
```
The database engine has the name monolith ( making a reference to the Monolith from the Space Odyssey series of Arthur C. Clarke ).

The project has for aim to produce a small, portable minimal object storage engine, optimized for fast writes and fast reads.
For now it has only a CLI, but at a later stage we'll implement a TCP server and plugins that will trigger programmed actions on read, on create,on update and on delete. At a later stage we will integrate a sql99 script engine.

I need the following from you:
- a readme.md for the repository.
- a presentation part for the webpage
- the text for the api documentation.

# Troubleshooting
I had some trouble compiling becausse of cc error after adding tokio and deno_core.
A fix ( from https://github.com/rust-lang/rust/issues/25289 )

```bash
sudo apt install gcc-multilib
cargo clean
cargo build
```

If the problem appears again

```bash
cargo clean
cargo build
```
If you cannot `cargo clean`

```bash
sudo chown -R $USER:$USER target
```

# The readme
_generated with ChatGPT_


## Introducing Monolith: A Fast Object Storage Engine

Tired of slow and bulky storage engines? Look no further than Monolith! Our engine is written in Rust and optimized for fast writes and reads, making it the perfect solution for your object storage needs.

With its small and portable design, Monolith is easy to integrate into any project. And with planned support for a TCP server and plugins, you can trigger programmed actions on read, create, update, and delete. Plus, with a planned integration of a SQL99 script engine, you can take your storage to the next level.

Get started with Monolith today and experience lightning-fast writes and reads. Join us in exploring the limits of object storage and see what the monolith from the Space Odyssey series by Arthur C. Clarke has inspired.



## Monolith: A Fast Object Storage Engine
Welcome to Monolith, an object storage engine written in Rust and optimized for fast writes and reads. Monolith is inspired by the monolith from the Space Odyssey series by Arthur C. Clarke and aims to be a small, portable, and minimal engine.

## Features
* Small and portable
* Fast writes and reads
* Command-line interface (CLI)
* TCP server and plugins (planned)
* SQL99 script engine (planned)


## Getting Started
The MonolithBackend struct is the main component of the engine. To use the engine, call the open method with the base path and the name of the database. You can then interact with the database through methods such as write, read_all, and get_from_index.

## Contributing
We welcome contributions to Monolith. If you're interested in contributing, please take a look at our contributing guidelines.

## License
Monolith is licensed under the MIT License.

### API Documentation
## MonolithBackend
The MonolithBackend struct is the main component of the Monolith object storage engine. It contains the following fields:

- name: String: The name of the database.
- data_path: String: The path to the data file.
- index_path: String: The path to the index file.
- index_cache: Vec<Index>: A cache of the indices in the database.

### Methods
open(base_path: &str, name: &str) -> Self
Opens a Monolith database with the given base path and name.

close(&mut self)
Closes the Monolith database.

`read_all_index(&mut self) -> Result<Vec<Record>, Box<dyn std::error::Error>>`
Reads all indices in the Monolith database and returns a vector of records.

`write_all_index(&mut self)`
Writes all indices in the Monolith database.

`get_from_index(&self, p0: fn(&Index) -> bool) -> Vec<Index>`
Gets a vector of indices from the Monolith database that match the given function.

`read_record_at_index(&self, ix: Index) -> Result<Record, Box<dyn std::error::Error>>`
Reads a record at a given index in the Monolith database.

`read_all_matching(&self, p0: fn(&Index) -> bool) -> Vec<Record>`
Reads all records in the Monolith database that match the given function.

`read_all(&self) -> Result<Vec<Record>, Box<dyn std::error::Error>>`
Reads all records in the Monolith database.

`write(&mut self, r: Record) -> Result<(), Box<dyn std::error::Error>>`
Writes a record to the Monolith database.