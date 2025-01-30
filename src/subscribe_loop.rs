use crate::print_stuff;
use futures_lite::StreamExt;
use iroh_gossip::net::{Event, GossipEvent, GossipReceiver, GossipSender};
use std::sync::Arc;
use std::collections::HashMap;
use iroh::NodeId;
use crate::message::Message;

pub async fn subscribe_loop(mut receiver: GossipReceiver, sender: Arc<GossipSender>) -> anyhow::Result<()> {
    todo!(); // Remove this todo.
    let mut names: HashMap<NodeId, String>= HashMap::new();

    while let Some(event) = receiver.try_next().await? {
        if let Event::Gossip(GossipEvent::Received(msg)) = event {
            match Message::from_bytes(&msg.content)? {
                // Remove this comment when Message::AboutMe is implemented.
                // Message::AboutMe { from, name } => {
                //     names.insert(from, name.clone());
                //     print_stuff::print_info(format!(
                //         "{} is now knows as {}",
                //         from.fmt_short(),
                //         name
                //     ));
                // }

                // Implement receiving and printing a message here

                // Bonus: Can we keep the names in sync across peers?

                _ => {
                    println!("Implement all cases please.")
                }
            }
        }
    }

    Ok(())
}
