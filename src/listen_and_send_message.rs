use std::sync::Arc;
use crate::print_stuff;
use anyhow::Error;
use iroh::NodeId;
use iroh_gossip::net::GossipSender;
use crate::message::Message;

pub async fn listen_and_send_message(sender: &Arc<GossipSender>, node_id: NodeId) -> Result<(), Error> {
    let (line_tx, mut line_rx) = tokio::sync::mpsc::channel(1);
    std::thread::spawn(move || input_loop(line_tx));

    while let Some(text) = line_rx.recv().await {
        let message = Message::Message {
            from: node_id,
            text: text.clone(),
        };
        sender.broadcast(message.to_vec().into()).await?;

        print_stuff::print_sent(text);
    }
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
