# harq

A CLI tool for exploring and filtering HAR files.

![Rust](https://img.shields.io/badge/rust-stable-orange.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Features

- **HAR file inspection** - View metadata, summary statistics, and entry details
- **Powerful filtering** - Expression-based filtering with comparison, logical, and string operators
- **Multiple output formats** - Table (colored), JSON, and compact (tab-separated for scripting)
- **GraphQL support** - Detect GraphQL requests and filter by operation name, type, or query
- **Timing analysis** - Performance insights with detailed timing breakdowns and statistics
- **Header inspection** - Search and filter HTTP headers
- **Flexible input** - Read from file path or stdin

## Installation

### Homebrew (macOS/Linux)

```bash
brew install tajchert/tap/harq
```

### From source

Build from source using Cargo:

```bash
cargo build --release
```

The binary will be available at `target/release/harq`.

## Quick Start

```bash
# View HAR file summary
harq info recording.har

# List all entries
harq ls recording.har

# Filter for failed requests
harq filter 'status >= 400' recording.har

# Find slow API calls
harq filter 'time > 1000 && url.contains("/api/")' recording.har

# Search for a pattern in URLs
harq search "login" recording.har

# View detailed timing breakdown
harq timing --stats recording.har

# Read from stdin
cat recording.har | harq ls -
```

## Commands

### info

Show HAR file metadata and summary statistics.

```bash
harq info recording.har
harq info --output json recording.har
```

Displays: version, creator, browser info, pages, entry count, method breakdown, status code breakdown, and timing summary.

### list / ls

List entries in the HAR file.

```bash
harq ls recording.har                    # Basic listing
harq ls -l recording.har                 # Long format with more columns
harq ls --head 10 recording.har          # First 10 entries
harq ls --tail 5 recording.har           # Last 5 entries
harq ls --output json recording.har      # JSON output
harq ls --output compact recording.har   # Tab-separated for scripting
```

### count

Count entries in the HAR file.

```bash
harq count recording.har
```

### view

View detailed information about a specific entry (1-based index).

```bash
harq view 1 recording.har                # View first entry
harq view 5 --full recording.har         # Full body content
harq view 3 --headers-only recording.har # Headers only
harq view 2 --output json recording.har  # JSON output
```

### search

Search entries by text or regex pattern.

```bash
harq search "api" recording.har                  # Search in URLs (default)
harq search -i "API" recording.har               # Case-insensitive
harq search -r "user/\d+" recording.har          # Regex pattern
harq search --headers "Authorization" recording.har  # Search in headers
harq search --body "error" recording.har         # Search in response bodies
harq search -v "static" recording.har            # Invert match
harq search -c "api" recording.har               # Count matches only
```

### filter

Filter entries using powerful expressions.

```bash
harq filter 'status == 200' recording.har
harq filter 'method == "POST" && time > 500' recording.har
harq filter 'isGraphQL && operationName.contains("User")' recording.har
harq filter 'status >= 400' --entries-only recording.har  # JSON array of entries
```

See [Filter Expression Syntax](#filter-expression-syntax) for full documentation.

### body

Extract request or response body content.

```bash
harq body 1 recording.har                # Response body of entry 1
harq body 3 --request recording.har      # Request body
harq body 2 --pretty recording.har       # Pretty-print JSON
harq body 5 --raw recording.har          # Raw bytes for binary content
```

### timing

Show timing breakdown for entries.

```bash
harq timing recording.har                     # Timing table
harq timing --stats recording.har             # Statistics summary
harq timing --sort wait recording.har         # Sort by wait time
harq timing --sort time --reverse recording.har  # Slowest first
harq timing --limit 10 recording.har          # Top 10 entries
```

### headers

Show headers for entries.

```bash
harq headers 1 recording.har             # Headers for entry 1
harq headers all recording.har           # Headers for all entries
harq headers 1 --request recording.har   # Request headers only
harq headers 1 --response recording.har  # Response headers only
harq headers all -f "content" recording.har  # Filter by header name
```

## Filter Expression Syntax

The `filter` command accepts powerful expressions for querying HAR entries.

### Available Fields

**Request fields:**
| Field | Description |
|-------|-------------|
| `method` | HTTP method (GET, POST, etc.) |
| `url` | Full request URL |
| `host`, `domain` | Hostname from URL |
| `path` | URL path (without query string) |
| `scheme`, `protocol` | URL scheme (http, https) |
| `query` | Query string |
| `request.httpVersion` | HTTP version |
| `request.headersSize` | Request headers size in bytes |
| `request.bodySize` | Request body size in bytes |
| `request.header("Name")` | Request header value (case-insensitive) |

**Response fields:**
| Field | Description |
|-------|-------------|
| `status` | HTTP status code |
| `statusText` | Status text (e.g., "OK", "Not Found") |
| `contentType` | Response content type |
| `contentSize` | Response content size in bytes |
| `bodySize` | Response body size in bytes |
| `response.httpVersion` | HTTP version |
| `response.headersSize` | Response headers size in bytes |
| `response.bodySize` | Response body size in bytes |
| `response.header("Name")` | Response header value (case-insensitive) |

**Timing fields (in milliseconds):**
| Field | Description |
|-------|-------------|
| `time` | Total request time |
| `blocked` | Time blocked waiting for connection |
| `dns` | DNS lookup time |
| `connect` | TCP connection time |
| `ssl` | SSL/TLS handshake time |
| `send` | Time to send request |
| `wait` | Time waiting for response (TTFB) |
| `receive` | Time to receive response |

**GraphQL fields:**
| Field | Description |
|-------|-------------|
| `isGraphQL` | Boolean: is this a GraphQL request? |
| `operationName` | GraphQL operation name |
| `operationType` | GraphQL type (query/mutation/subscription) |
| `gql.query` | Raw GraphQL query string |

**Other fields:**
| Field | Description |
|-------|-------------|
| `startedDateTime` | Request start time |
| `serverIpAddress` | Server IP address |

### Operators

**Comparison operators:**
- `==` - Equality
- `!=` - Inequality
- `>`, `>=`, `<`, `<=` - Numeric comparison

**Logical operators:**
- `&&` - Logical AND
- `||` - Logical OR
- `!` - Logical NOT

**String methods:**
- `.contains("str")` - Contains substring
- `.startsWith("str")` - Starts with prefix
- `.endsWith("str")` - Ends with suffix
- `.matches(/regex/)` - Matches regex (use `/pattern/i` for case-insensitive)

### Examples

```bash
# Status code filtering
harq filter 'status == 200' file.har            # Successful requests
harq filter 'status >= 400' file.har            # Client/server errors
harq filter 'status >= 500' file.har            # Server errors only

# Method filtering
harq filter 'method == "POST"' file.har         # POST requests
harq filter 'method != "GET"' file.har          # Non-GET requests

# URL filtering
harq filter 'host == "api.example.com"' file.har
harq filter 'url.contains("/api/v2/")' file.har
harq filter 'path.startsWith("/users")' file.har
harq filter 'url.matches(/\/users\/\d+/)' file.har

# Performance filtering
harq filter 'time > 1000' file.har              # Requests over 1 second
harq filter 'dns > 100' file.har                # Slow DNS lookups
harq filter 'wait > 500' file.har               # High server response time

# Header filtering
harq filter 'request.header("Authorization") != ""' file.har
harq filter 'response.header("content-type").contains("json")' file.har

# GraphQL filtering
harq filter 'isGraphQL' file.har
harq filter 'isGraphQL && status >= 400' file.har
harq filter 'operationName == "GetUser"' file.har
harq filter 'operationType == "mutation"' file.har

# Complex expressions
harq filter '(status >= 400 || time > 5000) && method == "POST"' file.har
harq filter 'host == "api.example.com" && !isGraphQL' file.har
harq filter 'status == 200 && contentType.contains("json") && time < 100' file.har
```

## Output Formats

Most commands support multiple output formats via `--output`:

| Format | Description | Use case |
|--------|-------------|----------|
| `table` | Colored, formatted tables | Interactive terminal use |
| `json` | Pretty-printed JSON | Parsing with jq, programmatic access |
| `compact` | Tab-separated values | Scripting, piping to other tools |

Status codes are color-coded in table output: green for 2xx, yellow for 3xx, red for 4xx/5xx.

---

# Developer Documentation

## Architecture Overview

```
harq
├── main.rs              # CLI entry point, command dispatch
├── commands/            # Command implementations
│   ├── info.rs          # HAR metadata and summary
│   ├── list.rs          # Entry listing
│   ├── count.rs         # Entry counting
│   ├── view.rs          # Detailed entry view
│   ├── search.rs        # Pattern search
│   ├── filter.rs        # Expression-based filtering
│   ├── body.rs          # Body extraction
│   ├── timing.rs        # Timing analysis
│   └── headers.rs       # Header inspection
├── filter/              # Filter expression engine
│   ├── mod.rs           # Public interface
│   └── eval.rs          # Parser and evaluator
├── har/                 # HAR data model
│   ├── mod.rs           # Public interface
│   └── types.rs         # HAR 1.2 type definitions
└── output/              # Output formatting
    ├── mod.rs           # Format enum and utilities
    ├── table.rs         # Table rendering
    └── json.rs          # JSON rendering
```

## Code Organization

### `src/main.rs`

Entry point defining the CLI structure using Clap's derive macros. Handles argument parsing and dispatches to the appropriate command module.

### `src/har/`

HAR parsing and type definitions implementing the HAR 1.2 specification:

- **`types.rs`** - Serde-enabled structs for the complete HAR format: `Har`, `Log`, `Entry`, `Request`, `Response`, `Timings`, `Content`, `Header`, `Cookie`, `Page`, etc.
- **`mod.rs`** - Re-exports and helper functions for loading HAR files

### `src/commands/`

Each command is implemented as a separate module with a consistent structure:

- `Args` struct with Clap derive for command-specific arguments
- `run(args, har)` function that executes the command
- Internal helper functions for formatting and logic

### `src/filter/`

Custom expression parser and evaluator for the filter command:

- **`eval.rs`** - Recursive descent parser with operator precedence handling
  - Tokenizer for expressions
  - Field extraction from HAR entries
  - Comparison and string method evaluation
  - GraphQL detection heuristics

### `src/output/`

Output formatting utilities:

- **`mod.rs`** - `OutputFormat` enum and shared utilities
- **`table.rs`** - Table rendering using the `tabled` crate
- **`json.rs`** - JSON serialization helpers

## Key Technical Decisions

### CLI Parsing
Uses **Clap v4** with derive macros for declarative argument definitions. Provides automatic help generation, argument validation, and subcommand handling.

### HAR Parsing
Uses **Serde** for JSON deserialization directly into strongly-typed Rust structs. All HAR 1.2 fields are supported with appropriate Option types for optional fields.

### Filter Expression Parser
Custom recursive descent parser supporting:
- Operator precedence (`!` > comparisons > `&&` > `||`)
- Parentheses for grouping
- String methods as postfix operators
- Regex literals with flags

### GraphQL Detection
Heuristic-based detection checking:
- URL path contains "graphql"
- Content-Type is application/json with POST method
- Request body parses as GraphQL query structure

### Binary Optimization
Release profile configured for minimal binary size:
- LTO (Link-Time Optimization) enabled
- Single codegen unit for better optimization
- Symbol stripping

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing with derive macros |
| `serde` / `serde_json` | JSON serialization and HAR parsing |
| `colored` | Terminal color output |
| `tabled` | Table formatting |
| `regex` | Regular expression support in filters and search |
| `chrono` | DateTime handling |
| `base64` | Decoding base64-encoded HAR body content |
| `anyhow` / `thiserror` | Error handling |
| `atty` | TTY detection for auto color mode |

## License

MIT
