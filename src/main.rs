use clap::Parser;
use sha2::{Sha512, Digest};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use url::Url; 

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
}

fn main() {
    let args = Args::parse();
    let mut hasher = Sha512::new();
    let url = Url::parse(&args.url);
    
    
    match url {
        Ok(u) => {
            hasher.update(u.to_string().as_bytes());
            let sha = hasher.finalize();
            let sha_base = URL_SAFE.encode(sha);
            let sha_base_truncated = sha_base[..5].to_string();

            println!("({u}) -> {sha_base_truncated}");
        }, 
        Err(e) => println!("URL cannot be parsed: {e}"),
    };
}