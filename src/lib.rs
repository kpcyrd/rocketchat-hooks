extern crate hyper;
extern crate hyper_rustls;
extern crate futures;
extern crate tokio_core;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod client;
mod error;
mod structs;

pub use error::Error;
pub use structs::Message;
pub use client::Client;

pub fn post(url: &str, username: &str, icon_emoji: &str, text: &str) -> Result<(), error::Error> {
    let client = Client::new(url)?;
    let msg = Message::new()
                .with_username(username.to_owned())
                .with_icon_emoji(icon_emoji.to_owned())
                .with_text(text.to_owned());
    client.post(msg)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
