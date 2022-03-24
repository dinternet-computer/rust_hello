use std::collections::BTreeMap;

type FileStorage = BTreeMap<String, Blob>;

pub enum BlobType {
    File,
    Directory,
}

struct Blob {
    blob_type: BlobType,
    content: Vec<u8>,
}