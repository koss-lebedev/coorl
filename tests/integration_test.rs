use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_coorl"))
}

// ── Methods ──────────────────────────────────────────────────────────────────

#[test]
fn get_returns_body() {
    let out = bin().args(["https://httpbingo.org/get"]).output().unwrap();
    assert!(out.status.success(), "exit: {}", out.status);
    let body = String::from_utf8_lossy(&out.stdout);
    assert!(body.contains("httpbingo.org"), "body: {body}");
}

#[test]
fn post_sends_body() {
    let out = bin()
        .args(["-X", "POST", "-H", "Content-Type: application/json",
               "-d", r#"{"hello":"world"}"#, "https://httpbingo.org/post"])
        .output().unwrap();
    assert!(out.status.success());
    let body = String::from_utf8_lossy(&out.stdout);
    assert!(body.contains("hello"), "body: {body}");
    assert!(body.contains("world"), "body: {body}");
}

#[test]
fn put_method() {
    let out = bin()
        .args(["-X", "PUT", "https://httpbingo.org/put"])
        .output().unwrap();
    assert!(out.status.success());
    let body = String::from_utf8_lossy(&out.stdout);
    assert!(body.contains("httpbingo.org"));
}

#[test]
fn delete_method() {
    let out = bin()
        .args(["-X", "DELETE", "https://httpbingo.org/delete"])
        .output().unwrap();
    assert!(out.status.success());
}

#[test]
fn head_method_empty_body() {
    let out = bin()
        .args(["-X", "HEAD", "https://httpbingo.org/get"])
        .output().unwrap();
    assert!(out.status.success());
    assert!(out.stdout.is_empty(), "HEAD should have no body");
}

// ── Output flags ──────────────────────────────────────────────────────────────

#[test]
fn include_flag_prepends_headers() {
    let out = bin().args(["-i", "https://httpbingo.org/get"]).output().unwrap();
    assert!(out.status.success());
    let body = String::from_utf8_lossy(&out.stdout);
    assert!(body.starts_with("HTTP/"), "should start with status line: {body}");
    assert!(body.contains("content-type"), "should include headers: {body}");
}

#[test]
fn verbose_writes_to_stderr_not_stdout() {
    let out = bin().args(["-v", "https://httpbingo.org/get"]).output().unwrap();
    assert!(out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stderr.contains('>'), "request headers should be on stderr: {stderr}");
    assert!(stderr.contains('<'), "response headers should be on stderr: {stderr}");
    assert!(!stdout.contains("HTTP/"), "status line must not appear on stdout: {stdout}");
}

#[test]
fn output_flag_writes_to_file() {
    let path = "/tmp/coorl-test-output.json";
    let out = bin()
        .args(["-o", path, "https://httpbingo.org/get"])
        .output().unwrap();
    assert!(out.status.success());
    assert!(out.stdout.is_empty(), "stdout should be empty when -o is set");
    let content = std::fs::read_to_string(path).expect("output file not created");
    assert!(content.contains("httpbingo.org"));
    std::fs::remove_file(path).ok();
}

#[test]
fn silent_suppresses_stderr() {
    let out = bin()
        .args(["-s", "-v", "https://httpbingo.org/get"])
        .output().unwrap();
    assert!(out.status.success());
    assert!(out.stderr.is_empty(), "silent should suppress stderr");
}

// ── Redirect handling ─────────────────────────────────────────────────────────

#[test]
fn redirect_not_followed_by_default() {
    let out = bin()
        .args(["https://httpbingo.org/redirect/1"])
        .output().unwrap();
    assert!(out.status.success());
}

#[test]
fn redirect_followed_with_location_flag() {
    let out = bin()
        .args(["-L", "https://httpbingo.org/redirect/1"])
        .output().unwrap();
    assert!(out.status.success());
    let body = String::from_utf8_lossy(&out.stdout);
    assert!(body.contains("httpbingo.org"), "should have followed redirect: {body}");
}

// ── Cookie header ─────────────────────────────────────────────────────────────

#[test]
fn cookie_sent_in_request() {
    let out = bin()
        .args(["-b", "session=abc123", "https://httpbingo.org/cookies"])
        .output().unwrap();
    assert!(out.status.success());
    let body = String::from_utf8_lossy(&out.stdout);
    assert!(body.contains("abc123"), "cookie should appear in response: {body}");
}

// ── TLS fingerprint ───────────────────────────────────────────────────────────

#[test]
fn chrome_tls_fingerprint() {
    let out = bin()
        .args(["https://tls.peet.ws/api/all"])
        .output().unwrap();
    assert!(out.status.success());
    let body = String::from_utf8_lossy(&out.stdout);
    assert!(body.contains("ja3") || body.contains("JA3"), "should get TLS fingerprint data: {body}");
}
