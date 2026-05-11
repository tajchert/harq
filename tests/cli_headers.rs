mod common;

use common::{harq, write_har};

#[test]
fn headers_name_matches_exact_header_name_case_insensitively() {
    let entry = r#"{
  "startedDateTime": "2026-05-12T00:00:00.000Z",
  "time": 1,
  "request": {
    "method": "POST",
    "url": "https://api.example.test/v1/comment",
    "httpVersion": "HTTP/2",
    "cookies": [],
    "headers": [
      { "name": "x-totp", "value": "123456" },
      { "name": "x-totp-extra", "value": "hidden" }
    ],
    "queryString": [],
    "headersSize": -1,
    "bodySize": -1
  },
  "response": {
    "status": 200,
    "statusText": "OK",
    "httpVersion": "HTTP/2",
    "headers": [],
    "content": { "size": 0, "mimeType": "text/plain", "text": "" },
    "headersSize": -1,
    "bodySize": -1
  },
  "cache": {},
  "timings": {}
}"#;
    let file = write_har("headers-name", entry);

    let output = harq(&["headers", "1", "--request", "--name", "X-TOTP", &file]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("x-totp: 123456"));
    assert!(!stdout.contains("x-totp-extra"));
}
