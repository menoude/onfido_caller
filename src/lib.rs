use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub mod api;
pub mod applicants;
pub mod applicants_list;
pub mod documents;
pub mod documents_list;
pub mod env;

use api::*;
use applicants::*;
use applicants_list::*;
use documents::*;
use documents_list::*;

pub fn run(api_token: &str) -> Result<()> {
    let created_applicant = create_applicant(api_token, Applicant::new(Some("Mnd"), Some("Yami")))?;
    println!("{}", created_applicant);
    let app_list = ApplicantsList::get_applicant_list(api_token)?;
    println!("{}", app_list);
    // app_list.clean_double_applicants(api_token)?;
    // let updated_list = ApplicantsList::get_applicant_list(api_token)?;
    // println!("{}", updated_list);
    let doc_result = upload_doc(
        api_token,
        "/Users/mennad/Pictures/meyami.jpg",
        &created_applicant,
    )?;
    println!("{}", doc_result);
    let doc_list = DocumentsList::get_docs_list(api_token, &created_applicant)?;
    println!("{}", doc_list);
    Ok(())
}
