![coorl](banner.png)

A curl-compatible HTTP client that sends requests with a **Chrome TLS fingerprint** instead of the default OpenSSL one.

## The problem

Many sites and APIs detect and block automated HTTP clients by inspecting the TLS handshake fingerprint (JA3/JA4). Standard curl advertises an OpenSSL fingerprint that's trivially identifiable as non-browser traffic. `coorl` uses the same TLS fingerprint as Chrome, making requests indistinguishable from a real browser at the transport layer.

## Installation

Download the latest binary for your platform from the [releases page](https://github.com/koss-lebedev/coorl/releases):

| Platform              | Download                                 |
| --------------------- | ---------------------------------------- |
| macOS (Apple Silicon) | `coorl-aarch64-apple-darwin.tar.gz`      |
| macOS (Intel)         | `coorl-x86_64-apple-darwin.tar.gz`       |
| Linux (arm64)         | `coorl-aarch64-unknown-linux-gnu.tar.gz` |
| Linux (x86_64)        | `coorl-x86_64-unknown-linux-gnu.tar.gz`  |

Extract and place the `coorl` binary somewhere on your `$PATH`.

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

## Verifying the fingerprint

You can confirm the fingerprint difference using [tls.peet.ws](https://tls.peet.ws), which echoes back the JA3/JA4 hashes of the TLS handshake it received.

With standard `curl`, the handshake is plainly an OpenSSL client:

```sh
curl -s https://tls.peet.ws/api/all | jq '.tls | {ja3_hash, ja4}'
```

```json
{
  "ja3_hash": "375c6162a492dfbf2795909110ce8424",
  "ja4": "t13d497h2_0d8feac7bc37_7395dae3b2f3"
}
```

With `coorl`, the same request produces Chrome's fingerprint instead:

```sh
coorl -s https://tls.peet.ws/api/all | jq '.tls | {ja3_hash, ja4}'
```

```json
{
  "ja3_hash": "64f2443ea70fee8e38a10c68fd299e49",
  "ja4": "t13d1516h2_8daaf6152771_d8a2da3f94cd"
}
```

## Exit codes

| Code | Meaning               |
| ---- | --------------------- |
| `0`  | Success               |
| `1`  | Generic error         |
| `6`  | DNS resolution failed |
| `7`  | Connection refused    |
