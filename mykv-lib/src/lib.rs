// TODO - need to decide on type system
// TODO - need to decide on file format for SST (https://github.com/facebook/rocksdb/wiki/Rocksdb-BlockBasedTable-Format)
// TODO - compaction
struct SSTable {
    level: u8,
}

/// Memtable implementation based off of rocks db
/// https://github.com/facebook/rocksdb/wiki/MemTable
struct Memtable {
    /// amount of data the Memtable can hold before requiring a flush
    capacity: u8,
}

impl Memtable {
    /// Write data to an SSTable structure, then drop
    pub fn flush() {
        unimplemented!()
    }
}

struct WAL {
    /// Total size in bytes a WAL may contain before writing to a new WAL
    max_total_wal_size: u8,
}

struct Manifest {}

struct MyKV {}

impl MyKV {
    /// Retrieve a value from the DB by key
    pub fn get() {
        unimplemented!()
    }

    /// Insert a new value into the DB with key and value
    /// Inserting a value with an existing key will behave as an update when the value is read
    pub fn put() {
        unimplemented!()
    }

    /// Remove a value from the DB by marking with a tombstone
    pub fn del() {
        unimplemented!()
    }

    /// Retrieve a range of contiguous keys from the DB
    pub fn range() {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
