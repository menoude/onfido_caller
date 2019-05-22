use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Document {
    id: Option<String>,
    created_at: Option<String>,
    file_name: Option<String>,
    file_type: Option<String>,
    file_size: Option<String>,
    r#type: Option<String>,
    side: Option<String>,
    issuing_country: Option<String>,
    href: Option<String>,
    download_href: Option<String>,
}

impl Document {
    pub fn read_file(path: PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(fs::read(path)?)
    }
}
