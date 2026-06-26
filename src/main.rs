use clap::Parser;
use std::process;

mod args;
mod client;
mod output;
mod request;

use args::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = match client::build(args.location) {
        Ok(c) => c,
        Err(e) => {
            if !args.silent {
                eprintln!("error: {e}");
            }
            process::exit(1);
        }
    };

    if args.verbose && !args.silent {
        let method = request::resolve_method(&args);
        eprintln!("> {method} {} HTTP/2", args.url);
        for h in &args.headers {
            eprintln!("> {h}");
        }
        if let Some(cookie) = &args.cookie {
            eprintln!("> Cookie: {cookie}");
        }
        eprintln!(">");
    }

    let response = match request::send(&client, &args).await {
        Ok(r) => r,
        Err(e) => {
            if !args.silent {
                eprintln!("error: {e}");
            }
            process::exit(exit_code_for(&e));
        }
    };

    if let Err(e) = output::write(response, &args).await {
        if !args.silent {
            eprintln!("error: {e}");
        }
        process::exit(1);
    }
}

fn exit_code_for(e: &wreq::Error) -> i32 {
    use std::error::Error as StdError;

    // Walk the full error source chain for DNS-specific OS error strings.
    let mut source: Option<&dyn StdError> = Some(e);
    while let Some(err) = source {
        let msg = err.to_string().to_lowercase();
        if msg.contains("dns")
            || msg.contains("resolve")
            || msg.contains("no such host")
            || msg.contains("nodename nor servname")
            || msg.contains("name or service not known")
            || msg.contains("no address associated")
            || msg.contains("failed to lookup")
        {
            return 6;
        }
        source = err.source();
    }

    if e.is_connect() {
        7
    } else {
        1
    }
}
