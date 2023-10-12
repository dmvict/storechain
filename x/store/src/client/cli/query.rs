use anyhow::Result;
use clap::{Args, Subcommand};

use gears::client::query::run_query;
use ibc_proto::cosmos::bank::v1beta1::QueryAllBalancesResponse as RawQueryAllBalancesResponse;
use ibc_proto::protobuf::Protobuf;
use proto_messages::cosmos::bank::v1beta1::{QueryAllMessagesRequest, QueryAllMessagesResponse};
use proto_types::AccAddress;
use tendermint_informal::block::Height;

#[derive(Args, Debug)]
pub struct QueryCli {
    #[command(subcommand)]
    command: MessageCommands,
}

#[derive(Subcommand, Debug)]
pub enum MessageCommands {
    /// Query for account balances by address
    Messages {
        /// address
        address: AccAddress,
    },
}

pub fn run_messages_query_command(
    args: QueryCli,
    node: &str,
    height: Option<Height>,
) -> Result<String> {
    match args.command {
        MessageCommands::Messages { address } => {
            let query = QueryAllMessagesRequest {
                address,
                pagination: None,
            };

            let res = run_query::<QueryAllBalancesResponse, RawQueryAllBalancesResponse>(
                query.encode_vec(),
                "/st.store.v1beta1.Query/GetAllMessages".into(),
                node,
                height,
            )?;

            Ok(serde_json::to_string_pretty(&res)?)
        }
    }
}
