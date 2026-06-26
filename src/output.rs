use std::{
    fs,
    io::{self, Write},
};
use wreq::Response;
use crate::args::Args;

pub async fn write(response: Response, args: &Args) -> io::Result<()> {
    let status = response.status();
    let version = response.version();
    let headers = response.headers().clone();
    let body = response
        .bytes()
        .await
        .map_err(io::Error::other)?;

    if args.verbose && !args.silent {
        eprintln!("< {:?} {}", version, status);
        for (name, value) in &headers {
            eprintln!("< {}: {}", name, value.to_str().unwrap_or("<binary>"));
        }
        eprintln!("<");
    }

    if args.include {
        print!("{:?} {}\r\n", version, status);
        for (name, value) in &headers {
            print!("{}: {}\r\n", name, value.to_str().unwrap_or("<binary>"));
        }
        print!("\r\n");
    }

    match &args.output {
        Some(path) => fs::write(path, &body),
        None => io::stdout().write_all(&body),
    }
}
