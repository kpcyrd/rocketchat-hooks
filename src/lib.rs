extern crate hyper;
extern crate hyper_rustls;
extern crate futures;
extern crate tokio_core;

#[macro_use] extern crate error_chain;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod client;
mod structs;

pub use structs::Message;
pub use client::Client;

mod errors {
    use serde_json;
    use std::io;
    use hyper;

    error_chain! {
        errors {
            Status(t: hyper::StatusCode) {
                description("invalid status code")
                display("server sent status code: '{}'", t)
            }
        }

        foreign_links {
            Io(io::Error);
            Uri(hyper::error::UriError);
            Http(hyper::error::Error);
            Json(serde_json::Error);
        }
    }
}
pub use errors::*;

pub fn post(url: &str, username: &str, icon_emoji: &str, text: &str) -> Result<()> {
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
