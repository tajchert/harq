use serde::{Deserialize, Serialize};

/// Root HAR structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Har {
    pub log: Log,
}

/// Log object - the main container
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub version: String,
    pub creator: Creator,
    #[serde(default)]
    pub browser: Option<Creator>,
    #[serde(default)]
    pub pages: Option<Vec<Page>>,
    pub entries: Vec<Entry>,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Creator/Browser info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Page info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub started_date_time: String,
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub page_timings: Option<PageTimings>,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Page timing info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageTimings {
    #[serde(default)]
    pub on_content_load: Option<f64>,
    #[serde(default)]
    pub on_load: Option<f64>,
    #[serde(default)]
    pub comment: Option<String>,
}

/// HTTP request/response entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    #[serde(default)]
    pub pageref: Option<String>,
    pub started_date_time: String,
    pub time: f64,
    pub request: Request,
    pub response: Response,
    pub cache: Cache,
    pub timings: Timings,
    #[serde(default)]
    pub server_ip_address: Option<String>,
    #[serde(default)]
    pub connection: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    // Custom fields (prefixed with _)
    #[serde(flatten)]
    pub custom: std::collections::HashMap<String, serde_json::Value>,
}

/// HTTP Request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub method: String,
    pub url: String,
    pub http_version: String,
    pub cookies: Vec<Cookie>,
    pub headers: Vec<Header>,
    pub query_string: Vec<QueryParam>,
    #[serde(default)]
    pub post_data: Option<PostData>,
    pub headers_size: i64,
    pub body_size: i64,
    #[serde(default)]
    pub comment: Option<String>,
}

/// HTTP Response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub status: i32,
    pub status_text: String,
    pub http_version: String,
    #[serde(default)]
    pub cookies: Vec<Cookie>,
    #[serde(default)]
    pub headers: Vec<Header>,
    pub content: Content,
    #[serde(default, alias = "redirectURL")]
    pub redirect_url: Option<String>,
    #[serde(default)]
    pub headers_size: i64,
    #[serde(default)]
    pub body_size: i64,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Cookie
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub expires: Option<String>,
    #[serde(default)]
    pub http_only: Option<bool>,
    #[serde(default)]
    pub secure: Option<bool>,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Query parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParam {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub comment: Option<String>,
}

/// POST data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostData {
    pub mime_type: String,
    #[serde(default)]
    pub params: Option<Vec<PostParam>>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
}

/// POST parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostParam {
    pub name: String,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub file_name: Option<String>,
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Response content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub size: i64,
    #[serde(default)]
    pub compression: Option<i64>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Cache info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cache {
    #[serde(default)]
    pub before_request: Option<CacheEntry>,
    #[serde(default)]
    pub after_request: Option<CacheEntry>,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheEntry {
    #[serde(default)]
    pub expires: Option<String>,
    #[serde(default)]
    pub last_access: Option<String>,
    #[serde(default)]
    pub e_tag: Option<String>,
    #[serde(default)]
    pub hit_count: Option<i64>,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Timing breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timings {
    #[serde(default)]
    pub blocked: Option<f64>,
    #[serde(default)]
    pub dns: Option<f64>,
    #[serde(default)]
    pub connect: Option<f64>,
    #[serde(default)]
    pub send: Option<f64>,
    #[serde(default)]
    pub wait: Option<f64>,
    #[serde(default)]
    pub receive: Option<f64>,
    #[serde(default)]
    pub ssl: Option<f64>,
    #[serde(default)]
    pub comment: Option<String>,
}

// Helper implementations

impl Entry {
    /// Get a header value from request
    pub fn request_header(&self, name: &str) -> Option<&str> {
        self.request
            .headers
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(name))
            .map(|h| h.value.as_str())
    }

    /// Get a header value from response
    pub fn response_header(&self, name: &str) -> Option<&str> {
        self.response
            .headers
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(name))
            .map(|h| h.value.as_str())
    }

    /// Get response content type
    pub fn content_type(&self) -> Option<&str> {
        self.response
            .content
            .mime_type
            .as_deref()
            .or_else(|| self.response_header("content-type"))
    }
}

impl Content {
    /// Decode content if base64 encoded
    pub fn decoded_text(&self) -> Option<Vec<u8>> {
        let text = self.text.as_ref()?;

        if self.encoding.as_deref() == Some("base64") {
            use base64::{Engine as _, engine::general_purpose::STANDARD};
            STANDARD.decode(text).ok()
        } else {
            Some(text.as_bytes().to_vec())
        }
    }

    /// Get text content as string (decoding base64 if needed)
    pub fn text_content(&self) -> Option<String> {
        let bytes = self.decoded_text()?;
        String::from_utf8(bytes).ok()
    }
}
