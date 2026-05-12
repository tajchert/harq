# Changelog

## 0.2.0 - 2026-05-12

- Decode HAR request and response bodies, including base64 `postData.text` and gzip-compressed responses.
- Add `harq curl N` to emit a request as a runnable `curl` command.
- Add `harq list --method POST,PUT,DELETE` for common write-request triage.
- Add `harq headers --name NAME` for exact, case-insensitive header lookup.
- Make `harq view --headers-only` show only request and response headers.
- Polish interactive color output while keeping JSON, compact, body, and curl output plain.
- Publish prebuilt macOS release archives for faster Homebrew installs without Rust.

## 0.1.0

- Initial release.
