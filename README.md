# coorl: curl pretending to be Chrome

A curl-compatible HTTP client that sends requests with a **Chrome TLS fingerprint** instead of the default OpenSSL one.

## The problem

Many sites and APIs detect and block automated HTTP clients by inspecting the TLS handshake fingerprint (JA3/JA4). Standard curl advertises an OpenSSL fingerprint that's trivially identifiable as non-browser traffic. `coorl` uses the same TLS fingerprint as Chrome, making requests indistinguishable from a real browser at the transport layer.

## Installation

```sh
cargo install --path .
```

## Usage

`coorl` accepts the same flags as curl for the most common use cases:

```sh
coorl [OPTIONS] <URL>
```

### Options

| Flag           | Long form    | Description                                          |
| -------------- | ------------ | ---------------------------------------------------- |
| `-X <METHOD>`  | `--request`  | HTTP method (default: GET, or POST when `-d` is set) |
| `-H <header>`  | `--header`   | Request header, repeatable                           |
| `-d <data>`    | `--data`     | Request body                                         |
| `-i`           | `--include`  | Include response headers in stdout                   |
| `-v`           | `--verbose`  | Print request/response headers to stderr             |
| `-o <file>`    | `--output`   | Write response body to file instead of stdout        |
| `-L`           | `--location` | Follow redirects (up to 10)                          |
| `-s`           | `--silent`   | Suppress error output                                |
| `-b <cookies>` | `--cookie`   | Send cookies (`"name=val; name2=val2"`)              |

### Examples

```sh
# Simple GET
coorl https://httpbin.org/get

# POST JSON
coorl -X POST -H "Content-Type: application/json" -d '{"key":"value"}' https://httpbin.org/post

# Follow redirects and show response headers
coorl -L -i https://httpbin.org/redirect/1

# Save response to file
coorl -o response.html https://example.com

# Verbose mode (prints request/response headers to stderr)
coorl -v https://httpbin.org/get

# Send cookies
coorl -b "session=abc123; theme=dark" https://httpbin.org/cookies
```

## Exit codes

| Code | Meaning               |
| ---- | --------------------- |
| `0`  | Success               |
| `1`  | Generic error         |
| `6`  | DNS resolution failed |
| `7`  | Connection refused    |
