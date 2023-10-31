use anyhow::{Ok, Result};
use clap::{Args, Subcommand};

use proto_types::AccAddress;

use crate::proto::MsgVal;
use crate::Message as StoreMessage;

#[derive(Args, Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: MessageCommands,
}

#[derive(Subcommand, Debug)]
pub enum MessageCommands {
    /// Send funds from one account to another
    Store {
        id: u64,
        address: AccAddress,
        msg: String,
    },
    Link {
        wallet: AccAddress,
        name: String,
        phone: String,
        email: String,
        address: Option<String>,
    },
}

pub fn run_messages_tx_command(args: Cli, _from_address: AccAddress) -> Result<StoreMessage> {
    match args.command {
        MessageCommands::Store { address, id, msg } => {
            Ok(StoreMessage::Store(MsgVal { address, id, msg }))
        }
        MessageCommands::Link { wallet, name, phone, email, address } => {
            Ok(StoreMessage::Link(crate::AccountData { wallet_address: wallet, name, email, phone, address, id: 0 }))
        }
    }
}
