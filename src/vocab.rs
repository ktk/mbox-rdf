use urlencoding::encode;
use sha2::{Sha256, Digest};

pub const DEFAULT_SCHEMA_IRI: &str = "https://mail.described.at/";
pub const DEFAULT_DATA_IRI: &str = "https://example.org/data/";
pub const SCHEMA_ORG: &str = "http://schema.org/";

pub const XSD_DATETIME: &str = "http://www.w3.org/2001/XMLSchema#dateTime";
pub const XSD_INTEGER: &str = "http://www.w3.org/2001/XMLSchema#integer";
pub const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

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

    pub fn account_iri(&self, email: &str) -> String {
        let lower = email.trim().to_lowercase();
        // Percent-encode characters that are illegal in IRIs
        let sanitized = lower
            .replace('"', "%22")
            .replace(' ', "%20")
            .replace('<', "%3C")
            .replace('>', "%3E");
        format!("mailto:{}", sanitized)
    }

    pub fn thread_iri(&self, root_msg_id: &str) -> String {
        format!("{}thread/{}", self.data_base, encode(root_msg_id))
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
        // RFC 3986 §3.3: '@' is a valid pchar in a URI path, so keep it literal.
        // Thunderbird's mid: handler doesn't decode percent-encoding before
        // searching for Message-ID, so %40 would break lookups.
        let encoded = encode(mid).replace("%40", "@");
        format!("mid:{}", encoded)
    }

    pub fn term(&self, local_name: &str) -> String {
        format!("{}{}", self.schema_base, local_name)
    }

    pub fn schema_term(&self, local_name: &str) -> String {
        format!("{}{}", SCHEMA_ORG, local_name)
    }
}
