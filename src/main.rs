mod broadcast_about_me;
mod cli_parser;
mod listen_and_send_message;
mod print_stuff;
mod subscribe_loop;
mod ticket;
mod message;

use crate::broadcast_about_me::broadcast_about_me;
use crate::cli_parser::parse_cli;
use crate::listen_and_send_message::listen_and_send_message;
use anyhow::Result;
use colored::*;
use iroh::protocol::Router;
use iroh::{Endpoint, SecretKey};
use iroh_gossip::net::{Gossip, GOSSIP_ALPN};
use ticket::Ticket;

#[tokio::main]
async fn main() -> Result<()> {
    let (args, topic, nodes) = parse_cli()?;

    let endpoint: Endpoint = Endpoint::builder()
        .secret_key(SecretKey::generate(rand::rngs::OsRng))
        .discovery_n0()
        .bind()
        .await?;

    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;

    let router = Router::builder(endpoint.clone())
        .accept(GOSSIP_ALPN, gossip.clone())
        .spawn()
        .await?;

    let ticket = {
        let me = endpoint.node_addr().await?;
        let nodes = nodes.iter().cloned().chain([me]).collect();
        Ticket { topic, nodes }
    };

    print_stuff::print_console(
        format!("{} {}", "Ticket to join us: ", ticket.to_string().bold()).yellow(),
    );

    let node_ids = nodes.iter().map(|x| x.node_id).collect();

    if nodes.is_empty() {
        print_stuff::print_info("waiting for nodes to join us...".to_string());
    } else {
        print_stuff::print_info(format!(
            "{} {} {}",
            "trying to connect to ",
            nodes.len(),
            "nodes..."
        ));
        for nodes in nodes.into_iter() {
            endpoint.add_node_addr(nodes)?;
        }
    }
    let (sender, receiver) = gossip.subscribe_and_join(topic, node_ids).await?.split();

    print_stuff::print_info("connected!".to_string());

    broadcast_about_me(&sender, args, endpoint.node_id()).await?;

    tokio::spawn(subscribe_loop::subscribe_loop(receiver));

    listen_and_send_message(&sender, endpoint.node_id()).await?;

    router.shutdown().await?;

    Ok(())
}