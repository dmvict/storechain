use database::Database;
use gears::{error::AppError, types::context::TxContext};
use ibc_proto::protobuf::Protobuf;
use prost::Message as ProstMessage;
use store::StoreKey;
use tendermint_proto::abci::RequestBeginBlock;

use crate::{proto::QueryByAccAddressRequest, Config, Keeper, Message};

#[derive(Debug, Clone)]
pub struct Handler<SK: StoreKey> {
    keeper: Keeper<SK>,
    config: Config,
}

impl<SK: StoreKey> Handler<SK> {
    pub fn new(keeper: Keeper<SK>, config: Config) -> Self {
        Handler { keeper, config }
    }

    pub fn handle<DB: Database>(
        &self,
        ctx: &mut TxContext<DB, SK>,
        msg: &Message,
    ) -> Result<(), AppError> {
        match msg {
            Message::Store(msg) => self.keeper.store_message(ctx, msg, &self.config),
            Message::Link(msg) => self.keeper.store_metadata(ctx, msg, &self.config),
        }
    }

    pub fn handle_begin_block<DB: Database>(
        &self,
        _ctx: &mut TxContext<DB, SK>,
        _request: RequestBeginBlock,
    ) {
        // TODO: implement genesis func
    }

    pub fn handle_query<DB: Database>(
        &self,
        ctx: &gears::types::context::QueryContext<DB, SK>,
        query: tendermint_proto::abci::RequestQuery,
    ) -> std::result::Result<bytes::Bytes, AppError> {
        match query.path.as_str() {
            "/st.store.v1beta1.Query/GetAllMessagesByAddr" => {
                let data = query.data.clone();
                let req = QueryByAccAddressRequest::decode(data)?;

                Ok(self
                    .keeper
                    .query_all_messages_by_addr(ctx, req, &self.config)
                    .encode_to_vec()
                    .into())
            }
            "/st.store.v1beta1.Query/GetLinkedData" => {
                let data = query.data.clone();
                let req = QueryByAccAddressRequest::decode(data)?;

                if let Some(data) = self.keeper.query_linked_data(ctx, req, &self.config) {
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
