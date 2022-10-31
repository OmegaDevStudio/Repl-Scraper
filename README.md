# Scraper

A discord token scraper, designed to scrape forks from replit.com. This script uses the Graphql api on replit to essentially pull forks.

## Setup

Please add an empty folder named data to the project directory.

Requires Rust installed with Cargo. You can follow this [guide](https://doc.rust-lang.org/cargo/getting-started/installation.html) here. Then please set rust to the nightly branch using `rustup default nightly`. After that you may run `cargo run --release` and should start running.

If you are too lazy you can contact me on discord and I can send you the executable.

Once your script is running, you paste the repl in which you want to scrape the forks of. For example /@templates/Discord-Bot-Starter. Hit enter, and it will begin. This will also check the tokens for you and place them in `valid.txt`.

You may need to increase your File Descriptor limit for your PC if you are doing more heavy duty scraping ~ 1500+. If not you may run into errors when attempting to extract.

**This tool generally requires a decent internet connection.**

## Help & Support
- My [discord server](https://discord.gg/jD4C57AJg6)
