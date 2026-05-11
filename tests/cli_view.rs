mod common;

use common::{harq, write_har};

#[test]
fn view_headers_only_omits_body_timing_and_metadata() {
    let entry = r#"{
  "startedDateTime": "2026-05-12T00:00:00.000Z",
  "time": 42,
  "serverIPAddress": "203.0.113.10",
  "request": {
    "method": "POST",
    "url": "https://api.example.test/v1/comment",
    "httpVersion": "HTTP/2",
    "cookies": [],
    "headers": [
      { "name": "x-totp", "value": "123456" }
    ],
    "queryString": [],
    "postData": { "mimeType": "text/plain", "text": "secret request body" },
    "headersSize": -1,
    "bodySize": -1
  },
  "response": {
    "status": 200,
    "statusText": "OK",
    "httpVersion": "HTTP/2",
    "headers": [
      { "name": "content-type", "value": "text/plain" }
    ],
    "content": { "size": 20, "mimeType": "text/plain", "text": "secret response body" },
    "headersSize": -1,
    "bodySize": 20
  },
  "cache": {},
  "timings": { "wait": 40, "receive": 2 }
}"#;
    let file = write_har("view-headers-only", entry);

    let output = harq(&["view", "1", "--headers-only", &file]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("REQUEST"));
    assert!(stdout.contains("x-totp: 123456"));
    assert!(stdout.contains("RESPONSE"));
    assert!(stdout.contains("content-type: text/plain"));
    assert!(!stdout.contains("Body"));
    assert!(!stdout.contains("secret"));
    assert!(!stdout.contains("TIMING"));
    assert!(!stdout.contains("Started"));
    assert!(!stdout.contains("Server IP"));
}
