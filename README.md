# rocketchat-hooks [![Build Status](https://travis-ci.org/kpcyrd/rocketchat-hooks.svg?branch=master)](https://travis-ci.org/kpcyrd/rocketchat-hooks) [![Crates.io](https://img.shields.io/crates/v/rocketchat-hooks.svg)](https://crates.io/crates/rocketchat-hooks)

```toml
[dependencies]
rocketchat-hooks = "*"
```

## Usage

```rust, no_run
extern crate rocketchat_hooks;

fn main() {
    rocketchat_hooks::post("https://example.com/hooks/secret", "username", ":banana:", "ohai").unwrap();
}
```

## License

MIT
