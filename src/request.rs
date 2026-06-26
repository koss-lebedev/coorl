use wreq::{
    Client, Method, Response,
    header::{HeaderMap, HeaderName, HeaderValue, COOKIE},
};
use crate::args::Args;

pub fn resolve_method(args: &Args) -> Method {
    let raw = match &args.method {
        Some(m) => m.to_uppercase(),
        None => if args.data.is_some() { "POST".into() } else { "GET".into() },
    };
    raw.parse().unwrap_or(Method::GET)
}

pub fn build_headers(args: &Args) -> HeaderMap {
    let mut map = HeaderMap::new();
    for h in &args.headers {
        if let Some((name, rest)) = h.split_once(':') {
            if let (Ok(name), Ok(value)) = (
                HeaderName::from_bytes(name.trim().as_bytes()),
                HeaderValue::from_str(rest.trim_start()),
            ) {
                map.insert(name, value);
            }
        }
    }
    if let Some(cookie) = &args.cookie {
        if let Ok(value) = HeaderValue::from_str(cookie) {
            map.insert(COOKIE, value);
        }
    }
    map
}

pub async fn send(client: &Client, args: &Args) -> wreq::Result<Response> {
    let method = resolve_method(args);
    let headers = build_headers(args);
    let mut builder = client.request(method, &args.url).headers(headers);
    if let Some(body) = &args.data {
        builder = builder.body(body.clone());
    }
    builder.send().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::Args;
    use clap::Parser;

    fn parse(argv: &[&str]) -> Args {
        Args::parse_from(argv)
    }

    #[test]
    fn method_defaults_to_get() {
        let args = parse(&["coorl", "https://example.com"]);
        assert_eq!(resolve_method(&args), Method::GET);
    }

    #[test]
    fn data_implies_post() {
        let args = parse(&["coorl", "-d", "body", "https://example.com"]);
        assert_eq!(resolve_method(&args), Method::POST);
    }

    #[test]
    fn explicit_method_overrides() {
        let args = parse(&["coorl", "-X", "PUT", "https://example.com"]);
        assert_eq!(resolve_method(&args), Method::PUT);
    }

    #[test]
    fn explicit_get_with_data() {
        let args = parse(&["coorl", "-X", "GET", "-d", "body", "https://example.com"]);
        assert_eq!(resolve_method(&args), Method::GET);
    }

    #[test]
    fn headers_parsed() {
        let args = parse(&["coorl", "-H", "Content-Type: application/json", "-H", "X-Foo: bar", "https://example.com"]);
        let map = build_headers(&args);
        assert!(map.contains_key("content-type"));
        assert!(map.contains_key("x-foo"));
    }

    #[test]
    fn cookie_added_to_headers() {
        let args = parse(&["coorl", "-b", "session=abc", "https://example.com"]);
        let map = build_headers(&args);
        assert!(map.contains_key("cookie"));
        assert_eq!(map["cookie"], "session=abc");
    }
}
