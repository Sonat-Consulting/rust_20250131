use crate::cli_parser::Args;
use crate::message::Message;
use anyhow::Error;
use iroh::NodeId;
use iroh_gossip::net::GossipSender;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;


pub async fn broadcast_about_me(
    sender: &Arc<GossipSender>,
    args: Args,
    node_id: NodeId,
) -> Result<(), Error> {
    // This function will run when opening or joining a topic. i.e cargo run -- open
    // It will broadcast information about me: name and from (node_id).
    // It should only execute if args.name is something.
    // You need to create the missing variant on the enum Message: "Message::AboutMe"  (message.rs)
    // The "from" property is always "node_id".
    // To broadcast the message you should use: sender.broadcast(message.to_vec().into()).await?;

    // Needed to broadcast message before all other assignments are done.
    // sleep(Duration::from_secs(3)).await;
    Ok(())
}
