mod common;

use common::{harq, write_har};

fn entry(method: &str, path: &str, status: i32) -> String {
    format!(
        r#"{{
  "startedDateTime": "2026-05-12T00:00:00.000Z",
  "time": 1,
  "request": {{
    "method": "{method}",
    "url": "https://api.example.test{path}",
    "httpVersion": "HTTP/2",
    "cookies": [],
    "headers": [],
    "queryString": [],
    "headersSize": -1,
    "bodySize": -1
  }},
  "response": {{
    "status": {status},
    "statusText": "OK",
    "httpVersion": "HTTP/2",
    "headers": [],
    "content": {{ "size": 0, "mimeType": "text/plain", "text": "" }},
    "headersSize": -1,
    "bodySize": -1
  }},
  "cache": {{}},
  "timings": {{}}
}}"#
    )
}

#[test]
fn list_filters_by_comma_separated_methods() {
    let entries = [
        entry("GET", "/read", 200),
        entry("POST", "/create", 201),
        entry("DELETE", "/delete", 204),
    ]
    .join(",");
    let file = write_har("list-method", &entries);

    let output = harq(&[
        "list",
        "--output",
        "compact",
        "--method",
        "POST,DELETE",
        &file,
    ]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "2\tPOST\t201\t1ms\thttps://api.example.test/create\n3\tDELETE\t204\t1ms\thttps://api.example.test/delete\n"
    );
}
