extern crate rocketchat_hooks;

use std::env;

fn main() {
    let hook = env::var("HOOK").unwrap_or("http://example.com/hooks/secret".to_owned());
    rocketchat_hooks::post(&hook, "username", ":banana:", "ohai").unwrap();
}
