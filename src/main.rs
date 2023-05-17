#![deny(unsafe_code)]

use clap::Parser;
use reqwest::Client;
use std::fs::File;
use std::io::{BufWriter, Write};

const APIURL: &str = "https://api.pwnedpasswords.com/range/";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of output file
    #[arg(short, long)]
    filename: String,

    /// Filter to passwords breached this many times
    #[arg(short, long, default_value_t = 3)]
    minimum: u8,
}

fn main() {
    let args = Args::parse();
    println!(
        "Downloading to filename {}, with minimum count {}",
        args.filename, args.minimum
    );

    println!("Kicking off Pwned Downloader!");
    // Create new file - guarantee no existing file is clobbered.
    let file = File::options()
        .write(true)
        .create_new(true)
        .open(args.filename)
        .unwrap();
    let mut file = BufWriter::new(file);
    // Creating a reqwest client once and reusing it should reduce connection setup time
    let rclient = reqwest::Client::new();

    for n in 0..=0xFF {
        let range = format!("{n:05X}");
        // Status updates
        if n % 0xF == 0 {
            println!("Processing range: {range}");
        }
        let pwnlist = get_pwn_url(&rclient, &format!("{APIURL}{range}?mode=ntlm")).unwrap();
        let filtered_pwnlist = filter_list(&range, &pwnlist, args.minimum);
        write!(file, "{filtered_pwnlist}").unwrap();
    }
}

#[tokio::main]
async fn get_pwn_url(rclient: &Client, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = rclient.get(url).send().await?.text().await?;
    Ok(resp)
}

fn filter_list(range: &str, pwnlist: &str, minimum: u8) -> String {
    let mut ret = String::new();
    for k in pwnlist.lines() {
        /*
        Example single line: FE78C878023BA4AB83C782E1A0B2316DF99:4
        Goal is to filter our strings where the decimal after the : is less than our minimum
        The range then needs to be prepended to make it the full hash
        */
        let v: Vec<&str> = k.split(':').collect();
        assert!(v.len() == 2);
        let seen: u32 = v[1].parse().expect("Invalid API Output");
        if seen >= minimum.into() {
            ret.push_str(&format!("{range}{k}\r\n"));
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    /*
    There's very few practical tests for this app. Note even the API output isn't stable, so that can't be tested.
    This test largely allows this app to ensure it compiles and runs
    */
    use super::filter_list;

    #[test]
    fn filters_properly() {
        let input = include_str!("../tests/input.txt");
        let desired_output = include_str!("../tests/output.txt");
        let output = filter_list("0000F", &input, 3);
        assert_eq!(output, desired_output);
    }
}
