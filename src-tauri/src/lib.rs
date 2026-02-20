pub mod history;

use std::collections::HashMap;

use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri::{State, async_runtime::Mutex, http::{Method}};
use tauri_plugin_http::reqwest::{self};

use crate::history::{create_http_history, get_all_http_history};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

struct AppState {
    conn: Mutex<Connection>
}

#[tauri::command]
async fn execute_http_request(request: HttpRequest, app_state: State<'_, AppState>) -> Result<HttpResponse, String> {
    let conn = app_state.conn.lock().await;
    match create_http_history(&conn, &request) {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
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
            None => Err("Something went wrong!".to_string()),
        },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let conn = Connection::open("test.db").unwrap();
    println!("{:?}", conn.path());
    conn.execute("CREATE TABLE IF NOT EXISTS http_request_history (id TEXT PRIMARY KEY, timestamp TEXT NOT NULL, method TEXT NOT NULL, url TEXT NOT NULL, headers TEXT, body TEXT)", []).unwrap();
    println!("{:?}", get_all_http_history(&conn).unwrap());
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState { conn: Mutex::new(conn) })
        .invoke_handler(tauri::generate_handler![greet, execute_http_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
