use crate::print_stuff;
use futures_lite::StreamExt;
use iroh_gossip::net::{Event, GossipEvent, GossipReceiver, GossipSender};
use std::collections::HashMap;
use std::sync::Arc;
use crate::message::Message;

pub async fn subscribe_loop(mut receiver: GossipReceiver, sender: Arc<GossipSender>) -> anyhow::Result<()> {
    let mut names = HashMap::new();

    while let Some(event) = receiver.try_next().await? {
        if let Event::Gossip(GossipEvent::Received(msg)) = event {
            match Message::from_bytes(&msg.content)? {
                Message::AboutMe { from, name } => {
                    names.insert(from, name.clone());
                    print_stuff::print_info(format!(
                        "{} is now knows as {}",
                        from.fmt_short(),
                        name
                    ));
                    sender.broadcast(Message::AboutUs {  names: names.clone() }.to_vec().into()).await?;
                }
                Message::AboutUs { names: incoming_names } => {
                    for (key, val)  in incoming_names {
                        names.insert(key, val);

                    }
                }
                Message::Message { from, text } => {
                    let name = names
                        .get(&from)
                        .map_or_else(|| from.fmt_short(), String::to_string);
                    print_stuff::print_received(format!("{}: {}", name, text));
                }
            }
        }
    }
    Ok(())
}
