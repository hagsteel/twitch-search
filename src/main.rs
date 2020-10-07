use std::env;
use std::process::exit;

use serde_json::Value;

const ROOT_URL: &'static str = "https://api.twitch.tv/helix/search/channels?query=science%20%26%20technology&live_only=true&first=100";

macro_rules! to_str {
    ($val: expr, $key: expr) => {
        $val.get($key).unwrap().as_str().unwrap().to_string()
    };
}

#[derive(Debug)]
struct Entry {
    lang: String,
    display_name: String,
    title: String,
    game_id: String,
}

fn filter(entry: &Entry) -> bool {
    if entry.title.to_lowercase().contains("rust") {
        true
    } else {
        false
    }
}

fn print(entry: Entry) {
    print!("{} | ", entry.lang);
    print!("https://twitch.tv/{:<20} | ", entry.display_name);
    print!("{}\n", entry.title);
}

fn to_entry(value: &mut Value) -> Entry {
    let value = value.take();
    Entry {
        lang: to_str!(value, "broadcaster_language"),
        display_name: to_str!(value, "display_name"),
        title: to_str!(value, "title"),
        game_id: to_str!(value, "game_id"),
    }
}

fn fetch(after: Option<String>) -> (Vec<Entry>, Option<String>) {
    let url = match after {
        Some(after) => format!("{}&after={}", ROOT_URL, after),
        None => ROOT_URL.to_string(),
    };

    let client_id = match env::var("TWITCHY_CLIENT_ID") {
        Ok(cid) => cid,
        Err(_e) => {
            eprintln!("Client id missing");
            exit(1);
        }
    };

    let token = match env::var("TWITCHY_TOKEN") {
        Ok(t) => t,
        Err(_e) => {
            eprintln!("OAuth token missing");
            exit(1);
        }
    };

    let resp = ureq::get(&url)
        .set("Authorization", &format!("Bearer {}", token))
        .set("Client-Id", &client_id)
        .call();

    let mut json = match resp.into_json() {
        Ok(j) => j,
        Err(e) => {
            eprintln!("failed to serialize json: {:?}", e);
            exit(1);
        }
    };

    let pagination = json
        .get_mut("pagination")
        .take()
        .and_then(|v| v.get("cursor").take())
        .and_then(|v| v.as_str())
        .map(|v| v.to_string());

    let data = match json.get_mut("data") {
        Some(Value::Array(a)) => a.into_iter().map(to_entry).collect::<Vec<_>>(),
        _ => {
            exit(0);
        }
    };

    (data, pagination)
}

fn main() {
    println!("Searching...");
    let mut page = None;
    loop {
        let (entries, p) = fetch(page);
        page = p;
        for entry in entries.into_iter().filter(filter).collect::<Vec<_>>() {
            print(entry);
        }

        if page.is_none() {
            break;
        }
    }
    println!("Done...");
}
