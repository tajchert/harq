use crate::har::{Har, Entry};
use crate::filter::eval::FilterExpr;
use crate::output::json::create_filtered_har;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
#[command(after_long_help = FILTER_HELP)]
pub struct FilterCmd {
    /// Filter expression (e.g., 'status >= 400', 'method == "POST"')
    #[arg()]
    pub expr: String,

    /// HAR file to analyze (use - for stdin)
    #[arg(default_value = "-")]
    pub file: String,

    /// Output as valid HAR (default), otherwise output JSON array of entries
    #[arg(long)]
    pub entries_only: bool,
}

impl FilterCmd {
    pub fn run(&self, har: &Har) -> Result<()> {
        let filter = FilterExpr::parse(&self.expr)?;

        let matching_entries: Vec<(usize, &Entry)> = har.log.entries
            .iter()
            .enumerate()
            .filter(|(_, e)| filter.matches(e))
            .map(|(i, e)| (i + 1, e))
            .collect();

        if self.entries_only {
            let entries: Vec<&Entry> = matching_entries.iter().map(|(_, e)| *e).collect();
            println!("{}", serde_json::to_string_pretty(&entries)?);
        } else {
            // Output as valid HAR
            let filtered = create_filtered_har(har, &matching_entries);
            println!("{}", serde_json::to_string_pretty(&filtered)?);
        }

        Ok(())
    }
}

const FILTER_HELP: &str = r#"AVAILABLE FIELDS:
  Request:
    method              HTTP method (GET, POST, etc.)
    url                 Full request URL
    host, domain        Hostname from URL
    path                URL path (without query string)
    scheme, protocol    URL scheme (http, https)
    query               Query string

  Response:
    status              HTTP status code (e.g., 200, 404, 503)
    statusText          Status text (e.g., "OK", "Not Found")
    contentType         Response content type
    contentSize         Response content size in bytes
    bodySize            Response body size

  Timing:
    time                Total request time in milliseconds
    blocked, dns, connect, ssl, send, wait, receive

  GraphQL:
    isGraphQL           Boolean: is this a GraphQL request?
    operationName       GraphQL operation name
    operationType       GraphQL type (query/mutation/subscription)
    gql.query           Raw GraphQL query string

  Headers:
    request.header("Name")   Request header value
    response.header("Name")  Response header value

OPERATORS:
  ==, !=              Equality
  >, >=, <, <=        Comparison
  &&, ||              Logical AND/OR
  !                   Logical NOT

STRING METHODS:
  .contains("str")    Contains substring
  .startsWith("str")  Starts with prefix
  .endsWith("str")    Ends with suffix
  .matches(/regex/)   Matches regular expression

EXAMPLES:
  status == 200                           Successful requests
  status >= 400                           Error responses
  status != 200                           Non-200 responses
  method == "POST"                        POST requests only
  host == "api.example.com"               Specific host
  url.contains("/api/")                   URLs containing /api/
  time > 1000                             Slow requests (>1s)
  isGraphQL && status >= 400              Failed GraphQL requests
  operationName.contains("User")          GraphQL ops with "User"
  request.header("Authorization") != ""   Authenticated requests

NOTE: Use double quotes for expressions with != (shell escaping)"#;
