use urlencoding::encode;
use sha2::{Sha256, Digest};

pub const DEFAULT_SCHEMA_IRI: &str = "https://mail.described.at/";
pub const DEFAULT_DATA_IRI: &str = "https://example.org/data/";

pub const XSD_DATETIME: &str = "http://www.w3.org/2001/XMLSchema#dateTime";
pub const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";
pub const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
pub const RDFS_LABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";
pub const RDFS_DOMAIN: &str = "http://www.w3.org/2000/01/rdf-schema#domain";
pub const RDFS_RANGE: &str = "http://www.w3.org/2000/01/rdf-schema#range";

pub struct Vocab {
    pub schema_base: String,
    pub data_base: String,
}

impl Vocab {
    pub fn new(schema_base: String, data_base: String) -> Self {
        let mut schema_base = schema_base;
        if !schema_base.ends_with('/') {
            schema_base.push('/');
        }
        let mut data_base = data_base;
        if !data_base.ends_with('/') {
            data_base.push('/');
        }
        Self { schema_base, data_base }
    }

    pub fn message_iri(&self, msg_id: Option<&str>, raw_bytes: &[u8]) -> String {
        match msg_id {
            Some(id) if !id.is_empty() => {
                format!("{}msg/{}", self.data_base, encode(id))
            }
            _ => {
                let mut hasher = Sha256::new();
                hasher.update(raw_bytes);
                let result = hasher.finalize();
                format!("{}msg/sha256/{:x}", self.data_base, result)
            }
        }
    }

    pub fn address_iri(&self, email: &str) -> String {
        format!("{}addr/{}", self.data_base, encode(&email.to_lowercase()))
    }

    pub fn folder_iri(&self, name: &str) -> String {
        format!("{}folder/{}", self.data_base, encode(name))
    }

    pub fn list_iri(&self, list_id: &str) -> String {
        format!("{}list/{}", self.data_base, encode(list_id))
    }

    pub fn attachment_iri(&self, hash: &str) -> String {
        format!("{}attachment/sha256/{}", self.data_base, hash)
    }

    pub fn mid_iri(&self, mid: &str) -> String {
        format!("mid:{}", encode(mid))
    }

    pub fn term(&self, local_name: &str) -> String {
        format!("{}{}", self.schema_base, local_name)
    }
}
