use crate::print_stuff;
use crate::ticket::Ticket;
use anyhow::Error;
use clap::Parser;
use iroh::NodeAddr;
use iroh_gossip::proto::TopicId;
use std::str::FromStr;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub name: Option<String>,
    #[clap(short, long, default_value = "0")]
    bind_port: u16,
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    Open,
    Join { ticket: String },
}

pub fn parse_cli() -> Result<(Args, TopicId, Vec<NodeAddr>), Error> {
    let args = Args::parse();

    match &args.command {
        Command::Open => {
            let topic = TopicId::from_bytes(rand::random());
            print_stuff::print_info(format!(
                "{} {}",
                "opening chat room for topic ",
                topic.to_string()
            ));
            print_stuff::print_info("**********************".to_string());
            Ok((args, topic, vec![]))
        }
        Command::Join { ticket } => {
            // let Ticket { topic, nodes } = Ticket::from_str(ticket)?;
            let Ticket { topic, nodes } = Ticket::from_str(ticket)?;
            print_stuff::print_info(format!(
                "{} {}",
                "joining chat room for topic ",
                topic.to_string()
            ));
            Ok((args, topic, nodes))
        }
    }
}
