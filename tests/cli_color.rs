mod common;

use common::{harq, write_har};

fn entry(method: &str, path: &str, status: i32, time: f64) -> String {
    format!(
        r#"{{
  "startedDateTime": "2026-05-12T00:00:00.000Z",
  "time": {time},
  "request": {{
    "method": "{method}",
    "url": "https://api.example.test{path}",
    "httpVersion": "HTTP/2",
    "cookies": [],
    "headers": [
      {{ "name": "x-totp", "value": "123456" }}
    ],
    "queryString": [],
    "postData": {{ "mimeType": "application/json", "text": "{{\"ok\":true}}" }},
    "headersSize": -1,
    "bodySize": -1
  }},
  "response": {{
    "status": {status},
    "statusText": "OK",
    "httpVersion": "HTTP/2",
    "headers": [
      {{ "name": "content-type", "value": "application/json" }}
    ],
    "content": {{ "size": 11, "mimeType": "application/json", "text": "{{\"ok\":true}}" }},
    "headersSize": -1,
    "bodySize": 11
  }},
  "cache": {{}},
  "timings": {{ "wait": {time}, "receive": 1 }}
}}"#
    )
}

fn has_ansi(text: &str) -> bool {
    text.contains("\x1b[")
}

#[test]
fn table_outputs_use_color_when_forced() {
    let entries = [
        entry("GET", "/read", 200, 10.0),
        entry("POST", "/create", 500, 1500.0),
    ]
    .join(",");
    let file = write_har("color-table", &entries);

    let info = harq(&["--color", "always", "info", &file]);
    let timing = harq(&["--color", "always", "timing", &file]);
    let headers = harq(&["--color", "always", "headers", "1", "--name", "x-totp", &file]);

    assert!(info.status.success(), "stderr: {}", String::from_utf8_lossy(&info.stderr));
    assert!(timing.status.success(), "stderr: {}", String::from_utf8_lossy(&timing.stderr));
    assert!(headers.status.success(), "stderr: {}", String::from_utf8_lossy(&headers.stderr));
    assert!(has_ansi(&String::from_utf8_lossy(&info.stdout)));
    assert!(has_ansi(&String::from_utf8_lossy(&timing.stdout)));
    assert!(has_ansi(&String::from_utf8_lossy(&headers.stdout)));
}

#[test]
fn machine_outputs_stay_plain_when_color_is_forced() {
    let file = write_har("color-machine", &entry("POST", "/create", 201, 1.0));

    let json = harq(&["--color", "always", "info", "--output", "json", &file]);
    let compact = harq(&["--color", "always", "list", "--output", "compact", &file]);
    let body = harq(&["--color", "always", "body", "1", "--request", &file]);
    let curl = harq(&["--color", "always", "curl", "1", &file]);

    assert!(json.status.success(), "stderr: {}", String::from_utf8_lossy(&json.stderr));
    assert!(compact.status.success(), "stderr: {}", String::from_utf8_lossy(&compact.stderr));
    assert!(body.status.success(), "stderr: {}", String::from_utf8_lossy(&body.stderr));
    assert!(curl.status.success(), "stderr: {}", String::from_utf8_lossy(&curl.stderr));
    assert!(!has_ansi(&String::from_utf8_lossy(&json.stdout)));
    assert!(!has_ansi(&String::from_utf8_lossy(&compact.stdout)));
    assert!(!has_ansi(&String::from_utf8_lossy(&body.stdout)));
    assert!(!has_ansi(&String::from_utf8_lossy(&curl.stdout)));
}
