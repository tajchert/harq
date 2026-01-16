use crate::har::Entry;
use anyhow::{Result, bail};
use regex::Regex;

/// A parsed filter expression
#[derive(Debug)]
pub enum FilterExpr {
    // Comparison operators
    Eq(Field, Value),
    Ne(Field, Value),
    Gt(Field, Value),
    Ge(Field, Value),
    Lt(Field, Value),
    Le(Field, Value),

    // String operations
    Contains(Field, String),
    StartsWith(Field, String),
    EndsWith(Field, String),
    Matches(Field, Regex),

    // Logical operators
    And(Box<FilterExpr>, Box<FilterExpr>),
    Or(Box<FilterExpr>, Box<FilterExpr>),
    Not(Box<FilterExpr>),

    // Boolean (for simple field checks)
    Bool(Field),
}

/// Field accessor
#[derive(Debug, Clone)]
pub enum Field {
    // Top-level entry fields
    Method,
    Url,
    Status,
    StatusText,
    Time,
    StartedDateTime,
    ServerIpAddress,

    // Request fields
    RequestHttpVersion,
    RequestHeadersSize,
    RequestBodySize,

    // Response fields
    ResponseHttpVersion,
    ResponseHeadersSize,
    ResponseBodySize,
    ContentType,
    ContentSize,

    // Timing fields
    TimingBlocked,
    TimingDns,
    TimingConnect,
    TimingSsl,
    TimingSend,
    TimingWait,
    TimingReceive,

    // Header access
    RequestHeader(String),
    ResponseHeader(String),
}

/// Value for comparison
#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
}

impl FilterExpr {
    /// Parse a filter expression string
    pub fn parse(expr: &str) -> Result<Self> {
        let expr = expr.trim();

        // Handle parentheses
        if expr.starts_with('(') && expr.ends_with(')') {
            // Check if these parens are balanced
            let inner = &expr[1..expr.len() - 1];
            if is_balanced(inner) {
                return Self::parse(inner);
            }
        }

        // Try to parse logical operators (lowest precedence)
        // Look for && and || at the top level (not inside parens)
        if let Some(pos) = find_top_level(expr, "||") {
            let left = Self::parse(&expr[..pos])?;
            let right = Self::parse(&expr[pos + 2..])?;
            return Ok(FilterExpr::Or(Box::new(left), Box::new(right)));
        }

        if let Some(pos) = find_top_level(expr, "&&") {
            let left = Self::parse(&expr[..pos])?;
            let right = Self::parse(&expr[pos + 2..])?;
            return Ok(FilterExpr::And(Box::new(left), Box::new(right)));
        }

        // Handle NOT
        if expr.starts_with('!') || expr.to_lowercase().starts_with("not ") {
            let inner = if expr.starts_with('!') {
                &expr[1..]
            } else {
                &expr[4..]
            };
            return Ok(FilterExpr::Not(Box::new(Self::parse(inner)?)));
        }

        // Parse comparison expressions
        Self::parse_comparison(expr)
    }

    fn parse_comparison(expr: &str) -> Result<Self> {
        // Check for method calls: field.method(arg)
        if let Some(idx) = expr.find('.') {
            let field_str = &expr[..idx];
            let rest = &expr[idx + 1..];

            if rest.starts_with("contains(") && rest.ends_with(')') {
                let arg = extract_string_arg(&rest[9..rest.len() - 1])?;
                let field = Field::parse(field_str)?;
                return Ok(FilterExpr::Contains(field, arg));
            }

            if rest.starts_with("startsWith(") && rest.ends_with(')') {
                let arg = extract_string_arg(&rest[11..rest.len() - 1])?;
                let field = Field::parse(field_str)?;
                return Ok(FilterExpr::StartsWith(field, arg));
            }

            if rest.starts_with("endsWith(") && rest.ends_with(')') {
                let arg = extract_string_arg(&rest[9..rest.len() - 1])?;
                let field = Field::parse(field_str)?;
                return Ok(FilterExpr::EndsWith(field, arg));
            }

            if rest.starts_with("matches(") && rest.ends_with(')') {
                let pattern = extract_regex_arg(&rest[8..rest.len() - 1])?;
                let field = Field::parse(field_str)?;
                return Ok(FilterExpr::Matches(field, pattern));
            }
        }

        // Binary comparison operators
        for (op, constructor) in [
            ("==", FilterExpr::Eq as fn(Field, Value) -> FilterExpr),
            ("!=", FilterExpr::Ne as fn(Field, Value) -> FilterExpr),
            (">=", FilterExpr::Ge as fn(Field, Value) -> FilterExpr),
            ("<=", FilterExpr::Le as fn(Field, Value) -> FilterExpr),
            (">", FilterExpr::Gt as fn(Field, Value) -> FilterExpr),
            ("<", FilterExpr::Lt as fn(Field, Value) -> FilterExpr),
        ] {
            if let Some(pos) = expr.find(op) {
                let field_str = expr[..pos].trim();
                let value_str = expr[pos + op.len()..].trim();

                let field = Field::parse(field_str)?;
                let value = Value::parse(value_str)?;

                return Ok(constructor(field, value));
            }
        }

        bail!("Unable to parse expression: {}", expr);
    }

    /// Evaluate filter against an entry
    pub fn matches(&self, entry: &Entry) -> bool {
        match self {
            FilterExpr::Eq(field, value) => {
                field.get_value(entry).map_or(false, |v| v.eq_value(value))
            }
            FilterExpr::Ne(field, value) => {
                field.get_value(entry).map_or(true, |v| !v.eq_value(value))
            }
            FilterExpr::Gt(field, value) => {
                field.get_value(entry).map_or(false, |v| v.gt_value(value))
            }
            FilterExpr::Ge(field, value) => {
                field.get_value(entry).map_or(false, |v| v.ge_value(value))
            }
            FilterExpr::Lt(field, value) => {
                field.get_value(entry).map_or(false, |v| v.lt_value(value))
            }
            FilterExpr::Le(field, value) => {
                field.get_value(entry).map_or(false, |v| v.le_value(value))
            }
            FilterExpr::Contains(field, s) => {
                field.get_string(entry).map_or(false, |v| v.contains(s))
            }
            FilterExpr::StartsWith(field, s) => {
                field.get_string(entry).map_or(false, |v| v.starts_with(s))
            }
            FilterExpr::EndsWith(field, s) => {
                field.get_string(entry).map_or(false, |v| v.ends_with(s))
            }
            FilterExpr::Matches(field, re) => {
                field.get_string(entry).map_or(false, |v| re.is_match(&v))
            }
            FilterExpr::And(left, right) => {
                left.matches(entry) && right.matches(entry)
            }
            FilterExpr::Or(left, right) => {
                left.matches(entry) || right.matches(entry)
            }
            FilterExpr::Not(inner) => !inner.matches(entry),
            FilterExpr::Bool(field) => {
                field.get_value(entry).map_or(false, |v| v.is_truthy())
            }
        }
    }
}

impl Field {
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();

        // Check for header access: request.header("Name") or response.header("Name")
        if s.starts_with("request.header(") && s.ends_with(')') {
            let name = extract_string_arg(&s[15..s.len() - 1])?;
            return Ok(Field::RequestHeader(name));
        }

        if s.starts_with("response.header(") && s.ends_with(')') {
            let name = extract_string_arg(&s[16..s.len() - 1])?;
            return Ok(Field::ResponseHeader(name));
        }

        Ok(match s.to_lowercase().as_str() {
            "method" => Field::Method,
            "url" => Field::Url,
            "status" => Field::Status,
            "statustext" | "status_text" => Field::StatusText,
            "time" => Field::Time,
            "starteddatetime" | "started_date_time" => Field::StartedDateTime,
            "serveripaddress" | "server_ip_address" | "serverip" => Field::ServerIpAddress,

            "request.httpversion" | "request.http_version" => Field::RequestHttpVersion,
            "request.headerssize" | "request.headers_size" => Field::RequestHeadersSize,
            "request.bodysize" | "request.body_size" => Field::RequestBodySize,

            "response.httpversion" | "response.http_version" => Field::ResponseHttpVersion,
            "response.headerssize" | "response.headers_size" => Field::ResponseHeadersSize,
            "response.bodysize" | "response.body_size" | "bodysize" | "body_size" => Field::ResponseBodySize,
            "contenttype" | "content_type" | "response.contenttype" => Field::ContentType,
            "contentsize" | "content_size" | "response.content.size" => Field::ContentSize,

            "timings.blocked" | "blocked" => Field::TimingBlocked,
            "timings.dns" | "dns" => Field::TimingDns,
            "timings.connect" | "connect" => Field::TimingConnect,
            "timings.ssl" | "ssl" => Field::TimingSsl,
            "timings.send" | "send" => Field::TimingSend,
            "timings.wait" | "wait" => Field::TimingWait,
            "timings.receive" | "receive" => Field::TimingReceive,

            _ => bail!("Unknown field: {}", s),
        })
    }

    pub fn get_value(&self, entry: &Entry) -> Option<Value> {
        match self {
            Field::Method => Some(Value::String(entry.request.method.clone())),
            Field::Url => Some(Value::String(entry.request.url.clone())),
            Field::Status => Some(Value::Number(entry.response.status as f64)),
            Field::StatusText => Some(Value::String(entry.response.status_text.clone())),
            Field::Time => Some(Value::Number(entry.time)),
            Field::StartedDateTime => Some(Value::String(entry.started_date_time.clone())),
            Field::ServerIpAddress => entry.server_ip_address.as_ref().map(|s| Value::String(s.clone())),

            Field::RequestHttpVersion => Some(Value::String(entry.request.http_version.clone())),
            Field::RequestHeadersSize => Some(Value::Number(entry.request.headers_size as f64)),
            Field::RequestBodySize => Some(Value::Number(entry.request.body_size as f64)),

            Field::ResponseHttpVersion => Some(Value::String(entry.response.http_version.clone())),
            Field::ResponseHeadersSize => Some(Value::Number(entry.response.headers_size as f64)),
            Field::ResponseBodySize => Some(Value::Number(entry.response.body_size as f64)),
            Field::ContentType => entry.content_type().map(|s| Value::String(s.to_string())),
            Field::ContentSize => Some(Value::Number(entry.response.content.size as f64)),

            Field::TimingBlocked => entry.timings.blocked.map(Value::Number),
            Field::TimingDns => entry.timings.dns.map(Value::Number),
            Field::TimingConnect => entry.timings.connect.map(Value::Number),
            Field::TimingSsl => entry.timings.ssl.map(Value::Number),
            Field::TimingSend => entry.timings.send.map(Value::Number),
            Field::TimingWait => entry.timings.wait.map(Value::Number),
            Field::TimingReceive => entry.timings.receive.map(Value::Number),

            Field::RequestHeader(name) => entry.request_header(name).map(|s| Value::String(s.to_string())),
            Field::ResponseHeader(name) => entry.response_header(name).map(|s| Value::String(s.to_string())),
        }
    }

    pub fn get_string(&self, entry: &Entry) -> Option<String> {
        self.get_value(entry).map(|v| v.to_string())
    }
}

impl Value {
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();

        // String literal
        if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
            return Ok(Value::String(s[1..s.len() - 1].to_string()));
        }

        // Boolean
        if s == "true" {
            return Ok(Value::Bool(true));
        }
        if s == "false" {
            return Ok(Value::Bool(false));
        }

        // Number
        if let Ok(n) = s.parse::<f64>() {
            return Ok(Value::Number(n));
        }

        // Treat as string without quotes
        Ok(Value::String(s.to_string()))
    }

    fn eq_value(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::Number(b)) => a.parse::<f64>().map_or(false, |n| (n - b).abs() < f64::EPSILON),
            (Value::Number(a), Value::String(b)) => b.parse::<f64>().map_or(false, |n| (n - a).abs() < f64::EPSILON),
            _ => false,
        }
    }

    fn gt_value(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a > b,
            (Value::String(a), Value::Number(b)) => a.parse::<f64>().map_or(false, |n| n > *b),
            (Value::Number(a), Value::String(b)) => b.parse::<f64>().map_or(false, |n| *a > n),
            (Value::String(a), Value::String(b)) => a > b,
            _ => false,
        }
    }

    fn ge_value(&self, other: &Value) -> bool {
        self.eq_value(other) || self.gt_value(other)
    }

    fn lt_value(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a < b,
            (Value::String(a), Value::Number(b)) => a.parse::<f64>().map_or(false, |n| n < *b),
            (Value::Number(a), Value::String(b)) => b.parse::<f64>().map_or(false, |n| *a < n),
            (Value::String(a), Value::String(b)) => a < b,
            _ => false,
        }
    }

    fn le_value(&self, other: &Value) -> bool {
        self.eq_value(other) || self.lt_value(other)
    }

    fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
        }
    }
}

// Helper functions

fn extract_string_arg(s: &str) -> Result<String> {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        Ok(s[1..s.len() - 1].to_string())
    } else {
        Ok(s.to_string())
    }
}

fn extract_regex_arg(s: &str) -> Result<Regex> {
    let s = s.trim();

    if s.starts_with('/') && s.ends_with("/i") {
        return Ok(Regex::new(&format!("(?i){}", &s[1..s.len() - 2]))?);
    }

    let pattern = if s.starts_with('/') && s.ends_with('/') {
        s[1..s.len() - 1].to_string()
    } else {
        extract_string_arg(s)?
    };

    Ok(Regex::new(&pattern)?)
}

fn is_balanced(s: &str) -> bool {
    let mut depth = 0;
    for c in s.chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
}

fn find_top_level(s: &str, pattern: &str) -> Option<usize> {
    let mut depth = 0;
    let mut in_string = false;
    let mut string_char = '"';
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len() {
        let c = chars[i];

        if in_string {
            if c == string_char && (i == 0 || chars[i - 1] != '\\') {
                in_string = false;
            }
            continue;
        }

        match c {
            '"' | '\'' => {
                in_string = true;
                string_char = c;
            }
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {
                if depth == 0 && s[i..].starts_with(pattern) {
                    return Some(i);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_eq() {
        let expr = FilterExpr::parse("status == 200").unwrap();
        assert!(matches!(expr, FilterExpr::Eq(Field::Status, Value::Number(200.0))));
    }

    #[test]
    fn test_parse_string_eq() {
        let expr = FilterExpr::parse(r#"method == "POST""#).unwrap();
        assert!(matches!(expr, FilterExpr::Eq(Field::Method, Value::String(_))));
    }

    #[test]
    fn test_parse_and() {
        let expr = FilterExpr::parse(r#"status == 200 && method == "GET""#).unwrap();
        assert!(matches!(expr, FilterExpr::And(_, _)));
    }
}
