use crate::models::{APIResult, URL};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Serialize;

fn construct_headers(token: Option<&str>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("Tauri // Gister"));

    if let Some(token) = token {
        let token = format!("Bearer {token}");
        let header_value = HeaderValue::from_str(token.as_str())
            .expect("Could not generate header from token value");
        headers.insert(AUTHORIZATION, header_value);
    }

    headers
}

pub fn make_get_request(url: URL, token: Option<&str>) -> APIResult<String> {
    let url = url.value();
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).headers(construct_headers(token)).send()?;
    let response_body = response.text()?;

    Ok(response_body)
}

pub fn make_post_request<T: Serialize>(
    url: URL,
    token: Option<&str>,
    data: T,
) -> APIResult<String> {
    let url = url.value();
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .json(&data)
        .headers(construct_headers(token))
        .send()?;
    let response_body = response.text()?;

    Ok(response_body)
}
