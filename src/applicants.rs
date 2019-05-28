use crate::*;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json;
use std::borrow::ToOwned;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    flat_number: Option<String>,
    building_name: Option<String>,
    building_number: Option<String>,
    street: Option<String>,
    sub_street: Option<String>,
    state: Option<String>,
    town: Option<String>,
    postcode: Option<String>,
    country: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdNumber {
    r#type: Option<String>,
    value: Option<String>,
    state_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Applicant {
    first_name: Option<String>,
    last_name: Option<String>,
    id_numbers: Vec<IdNumber>,
    addresses: Vec<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dob: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    telephone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mothers_maiden_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nationality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    town_of_birth: Option<String>,
}

pub fn create_applicant(api_token: &str, app: Applicant) -> Result<Applicant> {
    let url = Url::parse("https://api.onfido.com/v2/applicants/")?;
    let payload: String = String::try_from(app)?;
    let resp = post_onfido(url, api_token, payload)?.json()?;
    Ok(resp)
}

impl Applicant {
    pub fn new(first_name: Option<&str>, last_name: Option<&str>) -> Self {
        Applicant {
            first_name: first_name.map(ToOwned::to_owned),
            last_name: last_name.map(ToOwned::to_owned),
            ..Default::default()
        }
    }

    pub fn get_name(&self) -> String {
        format!(
            "{} {}",
            match &self.first_name {
                Some(name) => &name,
                None => "null",
            },
            match &self.last_name {
                Some(name) => &name,
                None => "null",
            }
        )
    }

    pub fn get_id(&self) -> &str {
        match &self.id {
            Some(id) => id,
            None => "null",
        }
    }

    pub fn delete(&self, api_token: &str) -> Result<()> {
        let answer_delete = delete_onfido(
            Url::parse("https://api.onfido.com/v2/applicants/")
                .and_then(|url| url.join(self.get_id()))?,
            api_token,
        )?;
        println!("{}", answer_delete);
        Ok(())
    }
}

impl TryFrom<Applicant> for String {
    type Error = OnfidoError;
    fn try_from(app: Applicant) -> std::result::Result<Self, Self::Error> {
        Ok(serde_json::to_string(&app).map_err(OnfidoError::Serialiaze)?)
    }
}

impl Display for Applicant {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match serde_json::to_string_pretty(&self) {
            Ok(pretty) => write!(f, "{}", pretty),
            Err(_) => Err(fmt::Error),
        }
    }
}
