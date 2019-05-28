type Result<T> = std::result::Result<T, OnfidoError>;

pub mod api;
pub mod applicants;
pub mod applicants_list;
pub mod documents;
pub mod documents_list;
pub mod env;
pub mod error;

use api::*;
use applicants::*;
use applicants_list::*;
use documents::*;
use documents_list::*;
use error::*;

pub fn run(api_token: &str) -> Result<()> {
    // let created_applicant = create_applicant(api_token, Applicant::new(Some("Mnd"), Some("Yami")))?;
    // println!("{}", created_applicant);
    // let app_list = ApplicantsList::get_applicant_list(api_token)?;
    // println!("{}", app_list);
    // app_list.clean_double_applicants(api_token)?;
    let updated_list = ApplicantsList::get_applicant_list(api_token)?;
    // println!("{}", updated_list);
    // let doc_result = upload_doc(
    //     api_token,
    //     "/Users/mennad/Pictures/photo diplo.jpg",
    //     &created_applicant,
    // )?;
    // println!("{}", doc_result);
    println!("{}", updated_list.applicants.len());
    for (count, app) in updated_list.applicants.into_iter().enumerate() {
        println!("Applicant {}: {}", count + 1, app);
        let doc_list = DocumentsList::get_docs_list(api_token, &app)?;
        println!("Documents: {}", doc_list);
    }
    Ok(())
}
