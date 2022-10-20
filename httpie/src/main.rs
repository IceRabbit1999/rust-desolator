use std::collections::HashMap;
use std::str::FromStr;
use clap::Parser;
use anyhow::{anyhow, Error, Result};
use reqwest::{Client, Url};

#[tokio::main]
async fn main() -> Result<()>{
    let opts: Opts = Opts::parse();
    let client = Client::new();
    let result = match opts.sub_cmd {
        SubCommand::Get(ref args) => my_get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

async fn my_get(client: Client, args: &Get) -> Result<()>{
    let resp = client.get(&args.url).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    println!("{:?}", resp.text().await?);

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "iceRabbit")]
struct Opts {
    #[clap(subcommand)]
    sub_cmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut split = s.split('=');
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self{
            k: split.next().ok_or_else(err)?.to_string(),
            v: split.next().ok_or_else(err)?.to_string(),
        })
    }
}

#[derive(Parser, Debug)]
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

fn parse_url(s: &str) -> Result<String> {
    let _result: Url = s.parse()?;
    Ok(s.into())
}

fn parse_kv_pair(body: &str) -> Result<KvPair> {
    Ok(body.parse()?)
}
