use crate::*;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::string::ToString;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Document {
    r#type: Option<String>,
    #[serde(default)]
    file: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuing_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    download_href: Option<String>,
}

impl Document {
    pub fn new(path: PathBuf) -> Result<Self> {
        let doc_type = path
            .extension()
            .ok_or(OnfidoError::FileType)?
            .to_str()
            .map(ToString::to_string);
        let file = fs::read(path)?;
        let doc = Document {
            r#type: doc_type,
            file,
            ..Default::default()
        };
        Ok(doc)
    }

    // fn is_valid(&self) -> bool {
    //     match self.file_type
    // }

    pub fn to_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

pub fn upload_doc(api_token: &str, path: &str, applicant: &Applicant) -> Result<String> {
    let file = Document::new(PathBuf::from(path))?.to_string()?;
    println!("{}", file);
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
