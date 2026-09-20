use std::time::Instant;

use clap::Parser;
use netspeed::calc::mbps;

/// Cloudflare's own public, unauthenticated speed-test backend — the
/// same one speed.cloudflare.com's web page itself calls from the
/// browser. `__down` streams back exactly `bytes` random bytes;
/// `__up` accepts and discards a POST body of any size.
const CF_HOST: &str = "https://speed.cloudflare.com";

#[derive(Parser)]
#[command(
    name = "netspeed",
    about = "Measures real download/upload throughput and latency against a public speed-test endpoint"
)]
struct Cli {
    /// Bytes to download for the throughput test.
    #[arg(long, default_value_t = 25_000_000)]
    download_bytes: u64,
    /// Bytes to upload for the throughput test.
    #[arg(long, default_value_t = 10_000_000)]
    upload_bytes: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    print!("latency: ");
    let latency_start = Instant::now();
    client
        .get(format!("{CF_HOST}/__down?bytes=0"))
        .send()
        .await?;
    println!("{:?}", latency_start.elapsed());

    print!("downloading {} bytes... ", cli.download_bytes);
    let download_start = Instant::now();
    let response = client
        .get(format!("{CF_HOST}/__down?bytes={}", cli.download_bytes))
        .send()
        .await?;
    let body = response.bytes().await?;
    let download_elapsed = download_start.elapsed();
    println!("{:?}", download_elapsed);
    println!(
        "download: {:.2} Mbps ({} bytes)",
        mbps(body.len() as u64, download_elapsed),
        body.len()
    );

    print!("uploading {} bytes... ", cli.upload_bytes);
    let payload = vec![0x42u8; cli.upload_bytes];
    let upload_start = Instant::now();
    client
        .post(format!("{CF_HOST}/__up"))
        .body(payload)
        .send()
        .await?;
    let upload_elapsed = upload_start.elapsed();
    println!("{:?}", upload_elapsed);
    println!(
        "upload: {:.2} Mbps ({} bytes)",
        mbps(cli.upload_bytes as u64, upload_elapsed),
        cli.upload_bytes
    );

    Ok(())
}
