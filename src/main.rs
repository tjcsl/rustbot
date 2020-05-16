use irc::client::prelude::*;

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let client = Client::new("config.toml").await?;
    client.identify()?;
    println!("Identity: {}", client.current_nickname());
    Ok(())
}
