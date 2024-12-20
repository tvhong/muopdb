use index::hnsw::reader::HnswReader;
use index::index::BoxedIndex;

pub struct IndexProvider {
    data_directory: String,
}

impl IndexProvider {
    pub fn new(data_directory: String) -> Self {
        Self { data_directory }
    }

    pub fn read_index(&self, name: &str) -> Option<BoxedIndex> {
        let index_path = format!("{}/{}", self.data_directory, name);
        let reader = HnswReader::new(index_path);
        let index = reader.read();
        Some(Box::new(index))
    }
}
