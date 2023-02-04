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
The database engine has the name monolith ( making a reference to the Monolith from the Space Odissey series of Arthur C. Clarke ).

The project has for aim to produce a small, portable minimal object storage engine, optimized for fast writes and fast reads.
For now it has only a CLI, but at a later stage we'll implement a TCP server and plugins that will trigger programmed actions on read, on create,on update and on delete. At a later stage we will intergrate a sql99 script engine.

I need the following from you:
- a readme.md for the repository.
- a presentation part for the webpage
