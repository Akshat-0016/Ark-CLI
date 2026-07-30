use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use crate::index::inverted_index::InvertedIndex;

/// Responsible for persisting an inverted index.
///
/// This module intentionally owns only serialization.
/// It does not build or mutate indexes.
pub struct IndexSerializer;

impl IndexSerializer {
    /// Save an index to disk.
    pub fn save<P: AsRef<Path>>(
        index: &InvertedIndex,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let writer = BufWriter::new(File::create(path)?);

        bincode::serialize_into(writer, index)?;

        Ok(())
    }

    /// Load an index from disk.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<InvertedIndex, Box<dyn std::error::Error>> {
        let reader = BufReader::new(File::open(path)?);

        let index = bincode::deserialize_from(reader)?;

        Ok(index)
    }

    /// Check whether a serialized index exists.
    pub fn exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists()
    }
}
