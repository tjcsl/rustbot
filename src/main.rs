extern crate irc;

use irc::client::prelude::*;

fn main() {
    let mut reactor = IrcReactor::new().unwrap();
    let config = Config::load("config.toml").unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();
}
