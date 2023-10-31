use database::Database;
use gears::{error::AppError, types::context::TxContext};
use ibc_proto::protobuf::Protobuf;
use prost::Message as ProstMessage;
use store::StoreKey;
use tendermint_proto::abci::RequestBeginBlock;

use crate::{proto::QueryByAccAddressRequest, Keeper, Message};

#[derive(Debug, Clone)]
pub struct Handler<SK: StoreKey> {
    keeper: Keeper<SK>,
}

impl<SK: StoreKey> Handler<SK> {
    pub fn new(keeper: Keeper<SK>) -> Self {
        Handler { keeper }
    }

    pub fn handle<DB: Database>(
        &self,
        ctx: &mut TxContext<DB, SK>,
        msg: &Message,
    ) -> Result<(), AppError> {
        match msg {
            Message::Store(msg) => self.keeper.store_message(ctx, msg),
            Message::Link(msg) => self.keeper.store_metadata(ctx, msg),
            Message::Get(msg) => self.keeper.get_message(ctx, msg),
        }
    }

    pub fn handle_begin_block<DB: Database>(
        &self,
        ctx: &mut TxContext<DB, SK>,
        _request: RequestBeginBlock,
    ) {
        let _contribution_threshold: u32 = 2;
        let _block_time = ctx.get_header().time.unix_timestamp();

        let (_need_pub_keys, _need_secret_keys) = self.keeper.get_empty_keypairs(ctx);

        // TODO:
        // self.keeper
        //     .make_public_keys(ctx, need_pub_keys, block_time, contribution_threshold);
        //
        // self.keeper.make_secret_keys(ctx, need_secret_keys);
    }

    pub fn handle_query<DB: Database>(
        &self,
        ctx: &gears::types::context::QueryContext<DB, SK>,
        query: tendermint_proto::abci::RequestQuery,
    ) -> std::result::Result<bytes::Bytes, AppError> {
        match query.path.as_str() {
            "/st.store.v1beta1.Query/GetAllMessages" => {
                Ok(self.keeper.query_all_messages(ctx).encode_to_vec().into())
            }
            "/st.store.v1beta1.Query/GetLinkedData" => {
                let data = query.data.clone();
                let req = QueryByAccAddressRequest::decode(data)?;

                if let Some(data) = self.keeper.query_linked_data(ctx, req) {
                    Ok(data.encode_to_vec().into())
                } else {
                    Err(AppError::InvalidRequest(
                        "the account does not exists".into(),
                    ))
                }
            }
            _ => Err(AppError::InvalidRequest("query path not found".into())),
        }
    }
}
