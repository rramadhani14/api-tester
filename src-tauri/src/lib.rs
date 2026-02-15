use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tauri::http::{Method, StatusCode};
use tauri_plugin_http::reqwest::{self};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    POST,
    GET,
    PATCH,
    PUT,
}

impl From<HttpMethod> for Method {
    fn from(m: HttpMethod) -> Self {
        match m {
            HttpMethod::POST => Method::POST,
            HttpMethod::GET => Method::GET,
            HttpMethod::PATCH => Method::PATCH,
            HttpMethod::PUT => Method::PUT,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub body: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

#[tauri::command]
async fn execute_http_request(request: HttpRequest) -> Result<HttpResponse, String> {
    let client = reqwest::Client::new();
    let mut req = client.request(request.method.into(), request.url);
    req = match request.body {
        Some(body) => req.body(body),
        _ => req,
    };
    let response = req.send().await;
    match response {
        Ok(response) => Ok(HttpResponse {
            status_code: response.status().as_u16(),
            headers: response
                .headers()
                .iter()
                .map(|(key, values)| {
                    (
                        key.as_str().to_string(),
                        values.to_str().unwrap_or("").to_string(),
                    )
                })
                .collect(),
            body: response.text().await.unwrap_or("".to_string()),
        }),
        Err(err) => match err.status() {
            Some(code) => Err(code.to_string()),
            None => Err("Something went wrong!".to_string())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, execute_http_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
