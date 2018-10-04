use errors::*;
use opener;
use std::fs;
use std::thread;
use std::time::Duration;
use api::{API_URL, Client};
use paths;
use term;


pub fn load_token() -> Result<String> {
    let path = paths::data_dir()?;
    let path = path.join("auth");
    let session = fs::read_to_string(path)?;
    Ok(session.trim().to_string())
}

pub fn save_token(session: &str) -> Result<()> {
    let path = paths::data_dir()?;
    let path = path.join("auth");
    fs::write(path, format!("{}\n", session))?;
    Ok(())
}

pub fn run_login() -> Result<()> {
    let mut client = Client::new(API_URL)?;

    if let Ok(session) = load_token() {
        client.authenticate(session);
        if let Ok(user) = client.verify_session() {
            term::info(&format!("Logged in as {:?}", user));
            return Ok(());
        }
    }

    let session = Client::random_session();
    client.authenticate(session.clone());
    let url = format!("{}/auth/{}", API_URL, session);

    term::success(&format!("Opening url: {}", url));
    opener::open(url)?;

    for _ in 0..24 {
        thread::sleep(Duration::from_secs(5));

        if let Ok(user) = client.verify_session() {
            save_token(&session)?;
            term::info(&format!("Logged in as {:?}", user));
            return Ok(());
        }
    }

    bail!("Authentication timed out")
}
