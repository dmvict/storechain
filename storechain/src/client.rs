use anyhow::Result;
use auth::cli::query::{run_auth_query_command, QueryCli as AuthQueryCli};
use bank::cli::{
    query::{run_bank_query_command, QueryCli as BankQueryCli},
    tx::{run_bank_tx_command, Cli as BankCli},
};
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
    Bank(BankCli),
    Msg(Cli),
}

pub fn tx_command_handler(command: Commands, from_address: AccAddress) -> Result<Message> {
    match command {
        Commands::Bank(args) => run_bank_tx_command(args, from_address).map(Message::Bank),
        Commands::Msg(args) => run_messages_tx_command(args, from_address).map(Message::Msg),
    }
}

#[derive(Subcommand, Debug)]
pub enum QueryCommands {
    Bank(BankQueryCli),
    Auth(AuthQueryCli),
    Msg(MessagesQueryCli),
}

pub fn query_command_handler(
    command: QueryCommands,
    node: &str,
    height: Option<Height>,
) -> Result<()> {
    let res = match command {
        QueryCommands::Msg(args) => run_messages_query_command(args, node, height),
        QueryCommands::Bank(args) => run_bank_query_command(args, node, height),
        QueryCommands::Auth(args) => run_auth_query_command(args, node, height),
    }?;

    println!("{}", res);
    Ok(())
}
