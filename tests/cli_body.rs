mod common;

use common::{harq, minimal_entry, write_har};

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
