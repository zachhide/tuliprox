use gloo_storage::{LocalStorage, Storage};
use crate::error::{Error, ErrorInfo};
use log::{error};
use serde::{de::DeserializeOwned, Serialize};
use reqwasm::http::Request;

enum RequestMethod {
    GET,
    POST,
    PUT,
    // PATCH,
    DELETE
}

const API_ROOT: &str = "/api/v1";

const TOKEN_KEY: &str = "urak.token";
pub fn get_token() -> Option<String> {
    if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
        Some(token)
    } else {
        None
    }
}

pub fn set_token(token: Option<&str>) {
    if let Some(t) = token.clone() {
        LocalStorage::set(TOKEN_KEY, String::from(t)).expect("failed to set");
    } else {
        LocalStorage::delete(TOKEN_KEY);
    }
}

/// build all kinds of http request: post/get/delete etc.
async fn request<B, T>(method: RequestMethod, url: &str, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let mut request = match method {
        RequestMethod::GET => Request::get(&url),
        RequestMethod::POST => Request::post(&url).body(serde_json::to_string(&body).unwrap()),
        RequestMethod::PUT =>  Request::put(&url).body(serde_json::to_string(&body).unwrap()),
        // RequestMethod::PATCH =>  Request::patch(&url).body(serde_json::to_string(&body).unwrap()),
        RequestMethod::DELETE =>  Request::delete(&url),
    }.header("Content-Type", "application/json");
    if let Some(token) = get_token() {
        request = request.header("Authorization", format!("Bearer {token}").as_str());
    }
    match request.send().await {

        Ok(response) => {
            match response.status() {
                200 => {
                    let data: Result<T, _> = response.json::<T>().await;
                    if let Ok(data) = data {
                        // debug!("Response: {:?}", data);
                        Ok(data)
                    } else {
                        Err(Error::DeserializeError)
                    }
                },
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                422 => {
                    let data: Result<ErrorInfo, _> = response.json::<ErrorInfo>().await;
                    if let Ok(data) = data {
                        Err(Error::UnprocessableEntity(data))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                _ => Err(Error::RequestError),
            }
        }
        Err(e) => {
            error!("{e}");
            Err(Error::RequestError)
        }
    }
}

/// Delete request
pub async fn request_delete<T>(url: &str) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(RequestMethod::DELETE, url, ()).await
}

/// Get request
pub async fn request_get<T>(url: &str) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(RequestMethod::GET, url, ()).await
}

pub async fn request_get_api<T>(url: &str) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(RequestMethod::GET, format!("{API_ROOT}{url}").as_str(), ()).await
}

/// Post request with a body
pub async fn request_post<B, T>(url: &str, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(RequestMethod::POST, url, body).await
}

/// Put request with a body
pub async fn request_put<B, T>(url: &str, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(RequestMethod::PUT, url, body).await
}

/// Set limit for pagination
pub fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}
