use crate::print_stuff;
use anyhow::Error;
use iroh::NodeId;
use iroh_gossip::net::GossipSender;
use crate::message::Message;
use std::sync::Arc;

pub async fn listen_and_send_message(sender: &Arc<GossipSender>, node_id: NodeId) -> Result<(), Error> {
    // todo!();
    let (line_sender, mut line_receiver) = tokio::sync::mpsc::channel(1);
    std::thread::spawn(move || input_loop(line_sender));

    // This function should listen for changes from stdin.
    // If text has been entered then it should create a message.
    // Make sure to implement the variant Message::Message
    // and broadcast it using sender.broadcast(message.to_vec().into()).await?;
    // After broadcast it should print the message it sent. Look in print_stuff
    
    // When this function is complete you should be able to send messages to another topic
    // cargo run -- -n MyName join <ticket>
    // then write something and hit enter
    Ok(())

}

fn input_loop(line_tx: tokio::sync::mpsc::Sender<String>) -> anyhow::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    loop {
        stdin.read_line(&mut buffer)?;
        line_tx.blocking_send(buffer.clone())?;
        buffer.clear();
    }
}
