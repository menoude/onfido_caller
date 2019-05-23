use crate::*;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentsList {
    pub documents: Vec<Document>,
}

impl Display for DocumentsList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match serde_json::to_string_pretty(&self) {
            Ok(pretty) => write!(f, "{}", pretty),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl DocumentsList {
    pub fn get_docs_list(api_token: &str, app: &Applicant) -> Result<Self> {
        let url = Url::parse(&format!(
            "https://api.onfido.com/v2/applicants/{}/documents/",
            app.get_id()
        ))?;
        let list: DocumentsList = api::get_onfido(url, api_token)?.json()?;
        Ok(list)
    }
}
