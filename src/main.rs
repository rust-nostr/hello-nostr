use nostr_sdk::prelude::*;
use std::str::FromStr;

const PRIVATE_KEY: &str = "9c3654397cf1d1cf4068111e23510ed872849353756032b69ab0e475b9564450";

#[tokio::main]
async fn main() -> Result<()> {
    let secret_key = SecretKey::from_str(PRIVATE_KEY).unwrap();
    let my_keys = Keys::new(secret_key);

    let message = format!(
        "Hello, world! My public key is: {}",
        my_keys.public_key().to_string()
    );
    println!("{}", message);

    let client = Client::new(&my_keys);
    client.add_relay("wss://relay.house", None).await?;
    client.add_relay("wss://relay.damus.io", None).await?;
    let event = client.publish_text_note(message, &[]).await?;
    println!("{:#?}", event);

    let filter = Filter {
        ids: Some(vec![event.to_string()]),
        authors: None,
        kinds: None,
        events: None,
        pubkeys: None,
        hashtags: None,
        references: None,
        search: None,
        since: None,
        until: None,
        limit: None,
    };

    let events = client.get_events_of(vec![filter], None).await?;
    println!("{:#?}", events);

    Ok(())
}
