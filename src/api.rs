use crate::applicants::*;
use reqwest::header::HeaderValue;
use reqwest::{header, Client, Url};
use std::error::Error;
use std::io::prelude::*;

/// Returns the body of the response from a get request to onfido on the endpoint indicated.
/// Returns a Box<Error> if it fails to resolve the url, if the request fails to be sent or if its body can't be read
pub fn get_onfido(url: Url, api_token: &str) -> Result<ApplicantsList, Box<Error>> {
    let resp: ApplicantsList = Client::new()
        .get(url)
        .header(header::AUTHORIZATION, format!("Token token={}", api_token))
        .send()?
        .json()?;
    Ok(resp)
}

pub fn post_onfido(url: Url, api_token: &str, payload: String) -> Result<String, Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    headers.append(
        header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Token token={}", api_token))?,
    );
    headers.append(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    let mut resp = Client::new()
        .post(url)
        .headers(headers)
        .body(payload)
        .send()?;
    let mut buf = String::new();
    resp.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn post_data_onfido(
    url: Url,
    api_token: &str,
    payload: Vec<u8>,
) -> Result<String, Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    headers.append(
        header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Token token={}", api_token))?,
    );
    headers.append(
        header::CONTENT_TYPE,
        HeaderValue::from_static("multipart/form-data"),
    );
    let mut resp = Client::new()
        .post(url)
        .headers(headers)
        .body(payload)
        .send()?;
    let mut buf = String::new();
    resp.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn delete_onfido(url: Url, api_token: &str) -> Result<String, Box<dyn Error>> {
    let mut resp = Client::new()
        .delete(url)
        .header(header::AUTHORIZATION, format!("Token token={}", api_token))
        .send()?;
    let mut buf = String::new();
    resp.read_to_string(&mut buf)?;
    Ok(buf)
}
