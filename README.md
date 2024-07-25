## Smite-rs 
[![Crates.io](https://img.shields.io/crates/v/smite.svg)](https://crates.io/crates/smite-rs)
[![Documentation](https://docs.rs/smite/badge.svg)](https://docs.rs/smite-rs)

Smite-rs is a Rust library for interacting with the Smite API. It is designed to be easy to use and provide a simple interface for interacting with the API.

### Quick Start

In order to use the Smite API, you will need to obtain a **dev-id** and **auth-key** from the [Hi-Rez Developer Portal](https://fs12.formsite.com/HiRez/form48/secure_index.html). Once you have it, you can use it to create a new `Client` instance and start making requests.

```rust
use smite::client::Client;

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("my-dev-id".to_string(), "my-auth-key".to_string());
    let player_name = "player-name";

    // API may return multiple players with the same name
    let player_info = &client.get_player(player_name).await?[0];

    println!(
        "Player {player_name} played for {} hours.",
        player_info.hours_played
    );

    Ok(())
}
```

### Custom requests

Some endpoints are not yet fully supported by library.
In this case you can use `Client::make_request` method to make custom requests.

```rust 
use serde_json::Value;
use smite::client::Client;

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let dev_id = "my-dev-id";
    let auth_key = "my-auth-key";

    let client = Client::new(dev_id.to_string(), auth_key.to_string());
    let res: Value = client.make_request("gettopmatches", true, &[]).await?;

    // ...
    // Or if you want to use custom struct make sure it implements `serde_json::Deserialize`
    // let response: MyCustomStruct = client.make_request("gettopmatches", true, &[]).await?;

    Ok(())
}
```
