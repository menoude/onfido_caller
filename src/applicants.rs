use serde::{Deserialize, Serialize};
use serde_json;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Applicant {
    id: Option<String>,
    created_at: Option<String>,
    delete_at: Option<String>,
    href: Option<String>,
    title: Option<String>,
    pub first_name: String,
    middle_name: Option<String>,
    pub last_name: String,
    gender: Option<String>,
    dob: Option<String>,
    telephone: Option<String>,
    mobile: Option<String>,
    country: Option<String>,
    mothers_maiden_name: Option<String>,
    previous_last_name: Option<String>,
    nationality: Option<String>,
    country_of_birth: Option<String>,
    town_of_birth: Option<String>,
    id_numbers: Vec<IdNumber>,
    addresses: Vec<Address>,
}

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

impl Applicant {
    pub fn new(first_name: &str, last_name: &str) -> Self {
        Applicant {
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            id: None,
            created_at: None,
            delete_at: None,
            href: None,
            title: None,
            middle_name: None,
            gender: None,
            dob: None,
            telephone: None,
            mobile: None,
            country: None,
            mothers_maiden_name: None,
            previous_last_name: None,
            nationality: None,
            country_of_birth: None,
            town_of_birth: None,
            id_numbers: vec![IdNumber {
                r#type: None,
                value: None,
                state_code: None,
            }],
            addresses: vec![Address {
                flat_number: None,
                building_name: None,
                building_number: None,
                street: None,
                sub_street: None,
                state: None,
                town: None,
                postcode: None,
                country: None,
                start_date: None,
                end_date: None,
            }],
        }
    }

    pub fn get_name(&mut self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn get_id(self) -> String {
        self.id.unwrap()
    }
}
