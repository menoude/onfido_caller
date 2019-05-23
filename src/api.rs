use crate::*;
use reqwest::header::HeaderValue;
use reqwest::{header, Client, Url};
use std::io::prelude::*;

/// Returns the body of the response from a get request to onfido on the endpoint indicated.
/// Returns a Box<Error> if it fails to resolve the url, if the request fails to be sent or if its body can't be read
pub fn get_onfido(url: Url, api_token: &str) -> Result<reqwest::Response> {
    let resp = Client::new()
        .get(url)
        .header(header::AUTHORIZATION, format!("Token token={}", api_token))
        .send()?;
    Ok(resp)
}

pub fn post_onfido(url: Url, api_token: &str, payload: String) -> Result<reqwest::Response> {
    let mut headers = header::HeaderMap::new();
    headers.append(
        header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Token token={}", api_token))?,
    );
    headers.append(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    let resp = Client::new()
        .post(url)
        .headers(headers)
        .body(payload)
        .send()?;
    Ok(resp)
}

pub fn post_data_onfido(url: Url, api_token: &str, payload: String) -> Result<String> {
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

pub fn delete_onfido(url: Url, api_token: &str) -> Result<String> {
    let mut resp = Client::new()
        .delete(url)
        .header(header::AUTHORIZATION, format!("Token token={}", api_token))
        .send()?;
    let mut buf = String::new();
    resp.read_to_string(&mut buf)?;
    Ok(buf)
}
