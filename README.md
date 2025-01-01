# Spotify CLI Rust

This project is a simple CLI tool that fetches the top 10 tracks of an artist from Spotify. It uses the Spotify Web API and is implemented in Rust. The tool allows you to search for an artist and retrieve their most popular tracks based on a given country.

## Prerequisites

Before running the project, ensure you have the following set up:

### 1. Rust
Make sure Rust is installed on your system. You can download and install it from the [official Rust website](https://www.rust-lang.org/).

### 2. Spotify Developer Account
You need a Spotify Developer account to access the Spotify Web API. Follow these steps:

- Visit the [Spotify Developer Dashboard](https://developer.spotify.com/dashboard/).
- Create a new application to obtain your `CLIENT_ID` and `CLIENT_SECRET`.

### 3. .env File

Create a `.env` file in the root of the project and add your Spotify `CLIENT_ID` and `CLIENT_SECRET`. The file should look like this:

```
CLIENT_ID=your_client_id
CLIENT_SECRET=your_client_secret
```

### 4. Dependencies

The project uses the following Rust libraries:

- `reqwest`: For making HTTP requests to the Spotify API.
- `serde_json`: For parsing JSON responses.
- `dotenv`: For loading environment variables from the `.env` file.
- `tokio`: For asynchronous runtime support.
- `base64`: For encoding credentials.

Add these dependencies to your `Cargo.toml` file:

```toml
[dependencies]
base64 = "0.22.1"
dotenv = "0.15.0"
reqwest = { version = "0.12.12", features = ["blocking"] }
serde_json = "1.0.134"
tokio = { version = "1.42.0", features = ["full"] }

```

## Usage

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/SpotifyCli_Rust.git
cd SpotifyCli_Rust
```

### 2. Build the Project

```bash
cargo build
```

### 3. Run the Project

Provide an artist name as a command-line argument:

```bash
cargo run "<artist_name>"
```

For example:

```bash
cargo run "Eminem"
```
![Example Output](https://github.com/aniketch07/Spotify-CLI-Rust/blob/main/image_2025-01-01_054230267.png)

This will output the top 10 tracks of the specified artist along with their popularity.

## How It Works

### 1. Authentication
The application uses the client credentials flow to authenticate with the Spotify Web API. The `CLIENT_ID` and `CLIENT_SECRET` are used to retrieve an access token.

### 2. Artist Search
The app searches for the artist using the `v1/search` endpoint of the Spotify API. It retrieves the artist's unique ID based on the provided name.

### 3. Top Tracks
Using the artist's unique ID, the app fetches their top 10 tracks via the `v1/artists/{id}/top-tracks` endpoint and displays them in the terminal.

## Error Handling

- If the artist is not found, an error message is displayed.
- If there is an issue retrieving the top tracks, an error message is displayed.
- If the user does not provide an artist name as a command-line argument, the program displays a usage message.

