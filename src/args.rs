use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "coorl", about = "curl-compatible HTTP client with Chrome TLS fingerprinting")]
pub struct Args {
    /// Target URL
    pub url: String,

    /// HTTP method (default: GET, or POST when --data is set)
    #[arg(short = 'X', long = "request")]
    pub method: Option<String>,

    /// Request header, repeatable: -H "Key: Value"
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    /// Request body
    #[arg(short = 'd', long = "data")]
    pub data: Option<String>,

    /// Include response headers in stdout output
    #[arg(short = 'i', long = "include")]
    pub include: bool,

    /// Print request and response headers to stderr
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Write response body to file instead of stdout
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,

    /// Follow redirects (up to 10)
    #[arg(short = 'L', long = "location")]
    pub location: bool,

    /// Suppress error output to stderr
    #[arg(short = 's', long = "silent")]
    pub silent: bool,

    /// Send cookies: "name=val; name2=val2"
    #[arg(short = 'b', long = "cookie")]
    pub cookie: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_url() {
        let a = Args::parse_from(["coorl", "https://example.com"]);
        assert_eq!(a.url, "https://example.com");
    }

    #[test]
    fn multiple_headers() {
        let a = Args::parse_from(["coorl", "-H", "A: 1", "-H", "B: 2", "https://example.com"]);
        assert_eq!(a.headers, vec!["A: 1", "B: 2"]);
    }

    #[test]
    fn flags_default_false() {
        let a = Args::parse_from(["coorl", "https://example.com"]);
        assert!(!a.include);
        assert!(!a.verbose);
        assert!(!a.location);
        assert!(!a.silent);
    }

    #[test]
    fn silent_flag() {
        let a = Args::parse_from(["coorl", "-s", "https://example.com"]);
        assert!(a.silent);
    }

    #[test]
    fn cookie_flag() {
        let a = Args::parse_from(["coorl", "-b", "session=abc", "https://example.com"]);
        assert_eq!(a.cookie, Some("session=abc".into()));
    }
}
