# Introducing Monolith: A Fast Object Storage Engine
(_ description authored by our tiny friend ChatGPT _)

Tired of slow and bulky storage engines? Look no further than Monolith! Our engine is written in Rust and optimized for fast writes and reads, making it the perfect solution for your object storage needs.

With its small and portable design, Monolith is easy to integrate into any project. And with planned support for a TCP server and plugins, you can trigger programmed actions on read, create, update, and delete. Plus, with a planned integration of a SQL99 script engine, you can take your storage to the next level.

Get started with Monolith today and experience lightning-fast writes and reads. Join us in exploring the limits of object storage and see what the monolith from the Space Odyssey series by Arthur C. Clarke has inspired.
(_a bit childish isn't it ?_)



# Monolith: A Fast Object Storage Engine
Welcome to Monolith, an object storage engine written in Rust and optimized for fast writes and reads. Monolith is inspired by the monolith from the Space Odyssey series by Arthur C. Clarke and aims to be a small, portable, and minimal engine.

# Features
* Small and portable
* Fast writes and reads
* Command-line interface (CLI)
* TCP server and plugins (planned)
* SQL99 script engine (planned)


# Getting Started
The MonolithBackend struct is the main component of the engine. To use the engine, call the open method with the base path and the name of the database. You can then interact with the database through methods such as write, read_all, and get_from_index.

# Contributing
We welcome contributions to Monolith. If you're interested in contributing, please take a look at our contributing guidelines.

# License
Monolith is licensed under the MIT License.

## API Documentation
# MonolithBackend
The MonolithBackend struct is the main component of the Monolith object storage engine. It contains the following fields:

- name: String: The name of the database.
- data_path: String: The path to the data file.
- index_path: String: The path to the index file.
- index_cache: Vec<Index>: A cache of the indices in the database.

## Methods
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