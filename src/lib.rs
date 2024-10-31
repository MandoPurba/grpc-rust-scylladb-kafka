pub mod proto {
    pub mod notes {
        include!(concat!(env!("OUT_DIR"), "/notes.rs"));
    }
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("note_descriptor");
}

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;
