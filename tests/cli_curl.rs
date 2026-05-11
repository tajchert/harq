mod common;

use common::{harq, write_har};

#[test]
fn curl_emits_replayable_command_with_headers_and_decoded_body() {
    let entry = r#"{
  "startedDateTime": "2026-05-12T00:00:00.000Z",
  "time": 1,
  "request": {
    "method": "POST",
    "url": "https://api.example.test/v1/comment",
    "httpVersion": "HTTP/2",
    "cookies": [],
    "headers": [
      { "name": "content-type", "value": "application/json" },
      { "name": "x-totp", "value": "123456" }
    ],
    "queryString": [],
    "postData": { "mimeType": "application/json", "text": "eyJvayI6dHJ1ZX0=", "encoding": "base64" },
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
    let file = write_har("curl", &entry);

    let output = harq(&["curl", "1", &file]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "curl -X POST 'https://api.example.test/v1/comment' \\\n  -H 'content-type: application/json' \\\n  -H 'x-totp: 123456' \\\n  --data-raw '{\"ok\":true}'\n"
    );
}
