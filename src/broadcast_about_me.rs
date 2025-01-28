use std::sync::Arc;
use crate::cli_parser::Args;
use crate::message::Message;
use anyhow::Error;
use iroh::NodeId;
use iroh_gossip::net::GossipSender;

pub async fn broadcast_about_me(
    sender: &Arc<GossipSender>,
    args: Args,
    node_id: NodeId,
) -> Result<(), Error> {
    if let Some(name) = args.name {
        let message = Message::AboutMe {
            from: node_id,
            name,
        };
        sender.broadcast(message.to_vec().into()).await?;
    }
    Ok(())
}
