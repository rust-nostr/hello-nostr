use std::str::FromStr;
use std::time::Duration;

use nostr_sdk::prelude::*;
use tokio::time;

const PRIVATE_KEY: &str = "9c3654397cf1d1cf4068111e23510ed872849353756032b69ab0e475b9564450";

#[tokio::main]
async fn main() -> Result<()> {
    let secret_key = SecretKey::from_str(PRIVATE_KEY)?;
    let my_keys = Keys::new(secret_key);

    let message = format!("Hello, world! My public key is: {}", my_keys.public_key());
    println!("{}", message);

    let opts = Options::new().wait_for_send(true);
    let client = Client::new_with_opts(&my_keys, opts);
    client.add_relay("wss://relay.house", None).await?;
    client.add_relay("wss://relay.damus.io", None).await?;

    client.connect().await;

    let event_id = client.publish_text_note(message, &[]).await?;
    println!("{}", event_id);

    // Retrieve only our last event (from both relays)
    // let filter = Filter::new().id(event_id);

    // Retrieve all the events that we have posted
    let filter = Filter {
        ids: None,
        authors: Some(vec![my_keys.public_key()]),
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

    time::sleep(Duration::from_secs(1)).await;

    let events = client.get_events_of(vec![filter], None).await?;
    println!("{:#?}", events);

    Ok(())
}
