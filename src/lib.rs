use reqwest::Url;
use serde_json::to_string_pretty;
use std::error::Error;
use std::path::PathBuf;

pub mod api;
pub mod applicants;
pub mod documents;
pub mod env;

use api::*;
use applicants::*;
use documents::*;

fn create_applicant(api_token: &str, app: Applicant) -> Result<String, Box<dyn Error>> {
    let url = Url::parse("https://api.onfido.com/v2/applicants/")?;
    let payload: String = app.into();
    println!("{:?}", payload);
    let resp = post_onfido(url, api_token, payload)?;
    let post_answer = to_string_pretty(&resp)?;
    Ok(post_answer)
}

fn delete_applicant(api_token: &str, id: &str) -> Result<(), Box<dyn Error>> {
    let answer_delete = delete_onfido(
        Url::parse("https://api.onfido.com/v2/applicants/").and_then(|url| url.join(&id))?,
        api_token,
    )?;
    println!("{}", answer_delete);
    Ok(())
}

fn clean_double_applicants(api_token: &str, list: ApplicantsList) -> Result<usize, Box<dyn Error>> {
    let mut duplicates_ids: Vec<String> = Vec::new();
    let mut names_list: Vec<String> = Vec::new();
    for applicant in list.applicants.into_iter() {
        let name = applicant.get_name();
        if names_list.contains(&name) {
            duplicates_ids.push(applicant.get_id());
        } else {
            names_list.push(name);
        }
    }
    for id in &duplicates_ids {
        delete_applicant(api_token, &id)?;
    }
    Ok(duplicates_ids.len())
}

fn get_applicant_list(api_token: &str) -> Result<ApplicantsList, Box<dyn Error>> {
    let url = Url::parse("https://api.onfido.com/v2/applicants/")?;
    let list = get_onfido(url, api_token)?;
    Ok(list)
}

fn upload_doc(api_token: &str, path: &str, id: &str) -> Result<String, Box<dyn Error>> {
    let file = Document::read_file(PathBuf::from(path))?;
    let resp = post_data_onfido(
        Url::parse(format!("https://api.onfido.com/v2/applicants/{}/documents/", id).as_str())?,
        api_token,
        file,
    )?;
    Ok(resp)
}

pub fn run(api_token: &str) -> Result<(), Box<dyn Error>> {
    let creation_result = create_applicant(api_token, Applicant::new(Some("Mnd"), Some("Yami")))?;
    println!("{}", creation_result);
    let list = get_applicant_list(api_token)?;
    println!("{}", list);
    let nb_doubles = clean_double_applicants(api_token, list)?;
    println!("Nb of doubles deleted: {}", nb_doubles);
    let updated_list = get_applicant_list(api_token)?;
    println!("{}", updated_list);
    // let doc_resp = upload_doc(
    //     api_token,
    //     "Users/mennad/Pictures/meyami.jpg",
    //     "3727390d-e80e-4fb7-a65c-45cb25ffdd5b",
    // )?;
    // println!("{}", doc_resp);
    Ok(())
}
