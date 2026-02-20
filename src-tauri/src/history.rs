use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{HttpRequest};

#[derive(Deserialize, Serialize, Debug)]
pub struct HttpRequestHistoryEntry {
    id: Uuid,
    timestamp: DateTime<Utc>,
    http_request: HttpRequest,
}

pub fn get_all_http_history(conn: &Connection) -> Result<Vec<HttpRequestHistoryEntry>> {
    let mut stmt = conn.prepare("SELECT * FROM http_request_history")?;
    let result = stmt.query_map(params![], |row| {
        let index: String = row.get(0)?;
        let datetime: String = row.get(1)?;
        let method: String = row.get(2)?;
        let headers: String = row.get(4)?;
        Ok(HttpRequestHistoryEntry {
            id: Uuid::parse_str(&index).unwrap(),
            timestamp: DateTime::parse_from_rfc3339(&datetime)
                .unwrap()
                .with_timezone(&Utc),
            http_request: HttpRequest {
                method: serde_json::from_str(&method).unwrap(),
                url: row.get(3)?,
                headers: serde_json::from_str(&headers).unwrap(),
                body: row.get(5)?,
            },
        })
    })?;
    Ok(result.filter(|i| i.is_ok()).map(|i| i.unwrap()).collect())
}


pub fn create_http_history(conn: &Connection, http_request: &HttpRequest) -> Result<()> {
    let id = Uuid::new_v4();
    let timestamp = Utc::now();
    let _ = conn.execute("INSERT INTO http_request_history (id, timestamp, method, url, headers, body) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", params![
        id.to_string(),
        timestamp.to_rfc3339(),
        serde_json::to_string(&http_request.method)?,
        http_request.url,
        serde_json::to_string(&http_request.headers)?,
        http_request.body
    ]);
    Ok(())
}