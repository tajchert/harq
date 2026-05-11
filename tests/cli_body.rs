use std::fs;
use std::process::Command;

fn write_har(name: &str, entries: &str) -> String {
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

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

fn minimal_entry(extra_request: &str, content: &str) -> String {
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

fn harq(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_harq"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn body_request_decodes_base64_post_data() {
    let entry = minimal_entry(
        r#""postData": { "mimeType": "text/plain;charset=UTF-8", "text": "dGVzdCBjb21tZW50", "encoding": "base64" },"#,
        r#"{ "size": 0, "mimeType": "text/plain", "text": "" }"#,
    );
    let file = write_har("request-base64", &entry);

    let output = harq(&["body", "1", "--request", &file]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "test comment\n");
}

#[test]
fn body_request_keeps_plain_post_data_as_text() {
    let entry = minimal_entry(
        r#""postData": { "mimeType": "text/plain;charset=UTF-8", "text": "test comment" },"#,
        r#"{ "size": 0, "mimeType": "text/plain", "text": "" }"#,
    );
    let file = write_har("request-plain", &entry);

    let output = harq(&["body", "1", "--request", &file]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "test comment\n");
}

#[test]
fn body_response_decodes_base64_then_gzip_content() {
    let entry = minimal_entry(
        "",
        r#"{ "size": 31, "mimeType": "application/json", "text": "H4sIAEtpAmoAA6tWys9WsiopKk2tBQCQX9SnCwAAAA==", "encoding": "base64" }"#,
    );
    let file = write_har("response-gzip", &entry);

    let output = harq(&["body", "1", &file]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "{\"ok\":true}\n");
}

#[test]
fn body_pretty_prints_decoded_json() {
    let entry = minimal_entry(
        r#""postData": { "mimeType": "application/json", "text": "eyJvayI6dHJ1ZX0=", "encoding": "base64" },"#,
        r#"{ "size": 0, "mimeType": "text/plain", "text": "" }"#,
    );
    let file = write_har("request-pretty", &entry);

    let output = harq(&["body", "1", "--request", "--pretty", &file]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "{\n  \"ok\": true\n}\n");
}
