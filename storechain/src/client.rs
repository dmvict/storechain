use anyhow::Result;
use clap::Subcommand;
use proto_types::AccAddress;
use st::cli::{
    query::{run_messages_query_command, QueryCli as MessagesQueryCli},
    tx::{run_messages_tx_command, Cli},
};
use tendermint_informal::block::Height;

use crate::message::Message;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Msg(Cli),
}

pub fn tx_command_handler(command: Commands, from_address: AccAddress) -> Result<Message> {
    match command {
        Commands::Msg(args) => {
            run_messages_tx_command(args, from_address).map(|msg| Message::Msg(msg))
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum QueryCommands {
    Msg(MessagesQueryCli),
}

pub fn query_command_handler(
    command: QueryCommands,
    node: &str,
    height: Option<Height>,
) -> Result<()> {
    let res = match command {
        QueryCommands::Msg(args) => run_messages_query_command(args, node, height),
    }?;

    println!("{}", res);
    Ok(())
}
