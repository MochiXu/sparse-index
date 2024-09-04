use crate::core::common::types::{DimOffset, ElementOffsetType};
use crate::core::common::StorageVersion;
use crate::core::posting_list::PostingListIter;
use crate::core::sparse_vector::RemappedSparseVector;
use std::borrow::Cow;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

mod inverted_index_compressed_mmap;
mod inverted_index_compressed_immutable_ram;
mod inverted_index_immutable_ram;
mod inverted_index_mmap;
mod inverted_index_ram;
mod inverted_index_ram_builder;

pub use inverted_index_ram::InvertedIndexRam;
pub use inverted_index_ram_builder::InvertedIndexBuilder;
pub use inverted_index_mmap::InvertedIndexMmap;
pub use inverted_index_immutable_ram::InvertedIndexImmutableRam;
pub use inverted_index_compressed_immutable_ram::InvertedIndexCompressedImmutableRam;
pub use inverted_index_compressed_mmap::InvertedIndexCompressedMmap;


pub const OLD_INDEX_FILE_NAME: &str = "inverted_index.data";
pub const INDEX_FILE_NAME: &str = "inverted_index.dat";

pub trait InvertedIndex: Sized + Debug + 'static {
    type Iter<'a>: PostingListIter + Clone
    where
        Self: 'a;

    type Version: StorageVersion;

    /// Open existing core based on path
    fn open(path: &Path) -> std::io::Result<Self>;

    /// Save core
    fn save(&self, path: &Path) -> std::io::Result<()>;

    /// Get posting list for dimension id
    fn get(&self, id: &DimOffset) -> Option<Self::Iter<'_>>;

    /// Get number of posting lists
    fn len(&self) -> usize;

    /// Check if the core is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get number of posting lists for dimension id
    fn posting_list_len(&self, id: &DimOffset) -> Option<usize>;

    /// Files used by this core
    fn files(path: &Path) -> Vec<PathBuf>;

    fn remove(&mut self, id: ElementOffsetType, old_vector: RemappedSparseVector);

    /// Upsert a vector into the inverted core.
    fn upsert(
        &mut self,
        id: ElementOffsetType,
        vector: RemappedSparseVector,
        old_vector: Option<RemappedSparseVector>,
    );

    /// Create inverted core from ram core
    fn from_ram_index<P: AsRef<Path>>(
        ram_index: Cow<InvertedIndexRam>,
        path: P,
    ) -> std::io::Result<Self>;

    /// Number of indexed vectors
    fn vector_count(&self) -> usize;

    // Get max existed core
    fn max_index(&self) -> Option<DimOffset>;
}
