#![allow(dead_code)]

use std::fs;
use std::process::{Command, Output};

pub fn write_har(name: &str, entries: &str) -> String {
    let path = std::env::temp_dir().join(format!(
        "harq-{name}-{}-{}.har",
        std::process::id(),
        unique_suffix()
    ));
    let har = format!(
        r#"{{
  "log": {{
    "version": "1.2",
    "creator": {{ "name": "test", "version": "1.0" }},
    "entries": [{entries}]
  }}
}}"#
    );
    fs::write(&path, har).unwrap();
    path.to_string_lossy().into_owned()
}

pub fn minimal_entry(extra_request: &str, content: &str) -> String {
    format!(
        r#"{{
  "startedDateTime": "2026-05-12T00:00:00.000Z",
  "time": 1,
  "request": {{
    "method": "POST",
    "url": "https://api.example.test/v1/comment",
    "httpVersion": "HTTP/2",
    "cookies": [],
    "headers": [{{ "name": "content-type", "value": "application/json" }}],
    "queryString": [],
    {extra_request}
    "headersSize": -1,
    "bodySize": -1
  }},
  "response": {{
    "status": 200,
    "statusText": "OK",
    "httpVersion": "HTTP/2",
    "headers": [{{ "name": "Content-Type", "value": "application/json" }}, {{ "name": "Content-Encoding", "value": "gzip" }}],
    "content": {content},
    "headersSize": -1,
    "bodySize": -1
  }},
  "cache": {{}},
  "timings": {{}}
}}"#
    )
}

pub fn harq(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_harq"))
        .args(args)
        .output()
        .unwrap()
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
