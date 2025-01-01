use base64::Engine;
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION}, Client};
use serde_json::Value;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Retrieve the artist name from CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <artist_name>");
        return;
    }

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set in .env file");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set in .env file");

    // Get the token
    let token = get_token(client_id, client_secret).await;

    if let Some(token) = token {
        // Get the artist name from the first argument
        let artist_name = args[1].clone();

        // Search for the artist
        if let Some(artist_id) = search_for_artist(token.clone(), &artist_name).await {
            // Get top 10 tracks by the artist
            get_top_tracks(token, artist_id, "US".to_string(), &artist_name).await;
        }
    }
}

async fn get_token(client_id: String, client_secret: String) -> Option<String> {
    let auth_string = format!("{}:{}", client_id, client_secret);
    let auth_base64 = base64::engine::general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = "https://accounts.spotify.com/api/token";

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Basic {}", auth_base64)).unwrap(),
    );
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    let data = [("grant_type", "client_credentials")];

    let client = Client::new();
    let response = client.post(url).headers(headers).form(&data).send().await;

    match response {
        Ok(resp) => {
            if let Ok(body) = resp.text().await {
                if let Some(token) = extract_access_token(&body) {
                    return Some(token);
                }
            }
        }
        Err(_) => eprintln!("Error getting token"),
    }

    None
}

fn extract_access_token(response_body: &str) -> Option<String> {
    if let Ok(parsed_json) = serde_json::from_str::<Value>(response_body) {
        parsed_json["access_token"].as_str().map(|s| s.to_string())
    } else {
        None
    }
}

async fn search_for_artist(token: String, name: &String) -> Option<String> {
    let url = format!("https://api.spotify.com/v1/search?q={}&type=artist&limit=1", name);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());

    let client = Client::new();
    let response = client.get(url).headers(headers).send().await;

    match response {
        Ok(resp) => {
            if let Ok(body) = resp.text().await {
                // Parse the response to get the artist ID
                if let Ok(json) = serde_json::from_str::<Value>(&body) {
                    // Ensure the items array is not empty
                    let items = json["artists"]["items"].as_array().unwrap();
                    if items.is_empty() {
                        eprintln!("No artist found for '{}'", name);
                        return None;  // No artist found
                    }

                    // Check if the artist name contains the query string (allow partial matches)
                    if let Some(artist_name) = items[0]["name"].as_str() {
                        if !artist_name.to_lowercase().contains(&name.to_lowercase()) {
                            eprintln!("No match for '{}' found, but found: '{}'", name, artist_name);
                            return None;  // Partial match check failed
                        }
                    }

                    // Return the artist ID if everything is valid
                    if let Some(artist_id) = items[0]["id"].as_str() {
                        return Some(artist_id.to_string());
                    }
                }
            }
        }
        Err(_) => eprintln!("Error during search for artist"),
    }

    None
}



async fn get_top_tracks(token: String, artist_id: String, country: String, artist_name: &String) {
    let url = format!(
        "https://api.spotify.com/v1/artists/{}/top-tracks?market={}",
        artist_id, country
    );

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());

    let client = Client::new();
    let response = client.get(url).headers(headers).send().await;

    match response {
        Ok(resp) => {
            if let Ok(body) = resp.text().await {
                // Parse the response to get top tracks
                if let Ok(json) = serde_json::from_str::<Value>(&body) {
                    println!("Top 10 tracks by {}", artist_name);
                    let tracks = &json["tracks"];
                    for (index, track) in tracks.as_array().unwrap().iter().enumerate() {
                        let name = track["name"].as_str().unwrap();
                        let popularity = track["popularity"].as_i64().unwrap();
                        
                        println!(
                            "Track {}: {} (Popularity: {})",
                            index + 1,
                            name,
                            popularity
                        );
                    }
                }
            }
        }
        Err(_) => eprintln!("Error fetching top tracks"),
    }
}
