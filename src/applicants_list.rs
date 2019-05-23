use crate::*;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicantsList {
    pub applicants: Vec<Applicant>,
}

impl Display for ApplicantsList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match serde_json::to_string_pretty(&self) {
            Ok(pretty) => write!(f, "{}", pretty),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl ApplicantsList {
    pub fn get_applicant_list(api_token: &str) -> Result<Self> {
        let url = Url::parse("https://api.onfido.com/v2/applicants/")?;
        let list = get_onfido(url, api_token)?.json()?;
        Ok(list)
    }

    pub fn clean_double_applicants(&self, api_token: &str) -> Result<()> {
        let mut duplicates: Vec<&Applicant> = Vec::new();
        let mut names_list: Vec<String> = Vec::new();
        for applicant in &self.applicants {
            let name = applicant.get_name();
            if names_list.contains(&name) {
                duplicates.push(&applicant);
            } else {
                names_list.push(name);
            }
        }
        for applicant in &duplicates {
            applicant.delete(api_token)?;
            println!(r#"Deleted "{}""#, applicant.get_name());
        }
        println!("Nb of duplicates deleted: {}", duplicates.len());
        Ok(())
    }
}
