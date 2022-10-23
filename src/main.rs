use anyhow::Result;
use rayon::prelude::*;
use reqwest::blocking::Client;
use std::fs;
use std::time::Duration;

mod ports;
mod subdomain;

use ports::MOST_COMMON_PORTS_100;

use ports::Subdomain;

fn main() -> Result<()> {
    let wordlist: Vec<String> = fs::read_to_string("input/subdomains_wordlist.txt")?
        .split("\n")
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect();

    let target = "google.com";

    let http_client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let pool = rayon::ThreadPoolBuilder::new().num_threads(256).build()?;

    pool.install(|| {
        let valid_subdomains: Vec<String> = wordlist
            .into_par_iter()
            .map(|word| format!("{}.{}", word, target))
            .filter(|target| subdomain::enumerate(&http_client, target))
            .collect();

        let results: Vec<Subdomain> = valid_subdomains
            .into_par_iter()
            .map(|subdomain| ports::scan_ports(subdomain, MOST_COMMON_PORTS_100))
            .collect();

        println!("{:?}", results);
    });

    Ok(())
}
