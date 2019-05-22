use crate::*;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
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
    pub fn read_file(path: PathBuf) -> Result<Vec<u8>> {
        Ok(fs::read(path)?)
    }
}

pub fn upload_doc(api_token: &str, path: &str, applicant: &Applicant) -> Result<String> {
    let file = Document::read_file(PathBuf::from(path))?;
    // write a copy to test that the file was correctly read
    let resp = post_data_onfido(
        Url::parse(&format!(
            "https://api.onfido.com/v2/applicants/{}/documents/",
            applicant.get_id()
        ))?,
        api_token,
        file,
    )?;
    Ok(resp)
}
