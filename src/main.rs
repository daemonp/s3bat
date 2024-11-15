use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use bat::PrettyPrinter;
use regex::Regex;
use clap::Parser;
use std::error::Error;

#[derive(Parser)]
#[command(version, about = "Display S3 files with syntax highlighting")]
struct Args {
    /// S3 URIs to display
    uris: Vec<String>,

    /// Specify the language for syntax highlighting
    #[arg(short = 'l', long)]
    language: Option<String>,

    /// Enable line numbers
    #[arg(short = 'n', long = "numbers")]
    numbers: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    if args.uris.is_empty() {
        eprintln!("Usage: s3bat s3://bucket/key");
        std::process::exit(1);
    }

    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    for arg in args.uris {
        let (bucket, key) = parse_s3_uri(&arg)?;

        let resp = client
            .get_object()
            .bucket(bucket)
            .key(key.clone())
            .send()
            .await
            .map_err(|e| {
                if e.to_string().contains("NoSuchKey") {
                    format!("Error: Object not found - {}", e)
                } else if e.to_string().contains("NoSuchBucket") {
                    format!("Error: Bucket not found - {}", e)
                } else {
                    format!("Error accessing S3: {}", e)
                }
            })?;

        let content_type = resp.content_type().unwrap_or("text/plain").to_string();
        let body = resp.body.collect().await?.into_bytes();

        PrettyPrinter::new()
            .input_from_bytes(&body)
            .language(args.language.as_deref()
                .or_else(|| guess_language(&key, &content_type))
                .unwrap_or("Plain Text"))
            .grid(true)
            .line_numbers(args.numbers)
            .paging_mode(bat::PagingMode::Always)
            .print()
            .map_err(|e| format!("Failed to print: {}", e))?;
    }

    Ok(())
}

fn parse_s3_uri(uri: &str) -> Result<(String, String), Box<dyn Error>> {
    let re = Regex::new(r"s3://([^/]*)(/.*)")?;
    let caps = re.captures(uri).ok_or("Invalid S3 URI format")?;

    let bucket = caps.get(1).unwrap().as_str().to_string();
    let key = caps.get(2).unwrap().as_str().trim_start_matches('/').to_string();

    Ok((bucket, key))
}

fn guess_language(key: &str, content_type: &String) -> Option<&'static str> {
    if let Some(ext) = key.split('.').last() {
        match ext {
            "js" => Some("JavaScript"),
            "py" => Some("Python"),
            "rs" => Some("Rust"),
            "json" => Some("JSON"),
            "yml" | "yaml" => Some("YAML"),
            "xml" => Some("XML"),
            "md" => Some("Markdown"),
            _ => None
        }
    } else if content_type.contains("javascript") {
        Some("JavaScript")
    } else if content_type.contains("python") {
        Some("Python")
    } else {
        None
    }
}
