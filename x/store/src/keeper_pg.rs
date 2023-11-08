use database::Database;
use gears::{
    error::AppError,
    types::context::{QueryContext, TxContext},
};
use proto_types::AccAddress;
use store::StoreKey;

use crate::{
    AccountData, Config, MsgVal, QueryAllMessagesResponse, QueryByAccAddressRequest,
    QueryLinkedDataResponse,
};

use crate::connect;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Keeper<SK: StoreKey> {
    store_key: SK,
}

impl<SK: StoreKey> Keeper<SK> {
    pub fn new(store_key: SK) -> Self {
        Keeper { store_key }
    }

    /// Store message in the chain.
    pub fn store_message<T: Database>(
        &self,
        ctx: &mut TxContext<T, SK>,
        msg: &MsgVal,
        config: &Config,
    ) -> Result<(), AppError> {
        if self.tx_linked_data(ctx, &msg.address, config).is_some() {
            self.store_message_db(ctx, msg, config)?;
        } else {
            return Err(AppError::InvalidRequest(
                "Can't contribute data without existing account.".into(),
            ));
        }

        Ok(())
    }

    fn store_message_db<T: Database>(
        &self,
        _ctx: &mut TxContext<T, SK>,
        msg: &MsgVal,
        config: &Config,
    ) -> Result<(), AppError> {
        let pg_pool = connect(&config.pg_url).unwrap();
        let _id = async_std::task::block_on(crate::add_msg(&pg_pool, msg));
        Ok(())
    }

    /// Query messages owned by a user from db.
    pub fn query_all_messages_by_addr<T: Database>(
        &self,
        _ctx: &QueryContext<T, SK>,
        req: QueryByAccAddressRequest,
        config: &Config,
    ) -> QueryAllMessagesResponse {
        let pg_pool = connect(&config.pg_url).unwrap();
        let messages =
            async_std::task::block_on(crate::select_msgs_by_addr(&pg_pool, &req.into())).unwrap();
        QueryAllMessagesResponse { messages }
    }

    /// Store account metadata in the separate database.
    pub fn store_metadata<T: Database>(
        &self,
        _ctx: &mut TxContext<T, SK>,
        msg: &AccountData,
        config: &Config,
    ) -> Result<(), AppError> {
        let pg_pool = connect(&config.pg_url).unwrap();
        let _id = async_std::task::block_on(crate::add_linked_data(&pg_pool, msg)).unwrap();
        Ok(())
    }

    /// Get linked data using wallet address.
    pub fn query_linked_data<T: Database>(
        &self,
        _ctx: &QueryContext<T, SK>,
        req: QueryByAccAddressRequest,
        config: &Config,
    ) -> Option<QueryLinkedDataResponse> {
        self._query_linked_data(&req.address, config)
    }

    #[inline]
    pub fn _query_linked_data(
        &self,
        address: &AccAddress,
        config: &Config,
    ) -> Option<QueryLinkedDataResponse> {
        let pg_pool = connect(&config.pg_url).unwrap();
        let linked_data =
            async_std::task::block_on(crate::query_linked_data(&pg_pool, &address.clone().into()));
        if let Ok(data) = linked_data {
            Some(QueryLinkedDataResponse { data })
        } else {
            None
        }
    }

    pub fn tx_linked_data<T: Database>(
        &self,
        _ctx: &TxContext<T, SK>,
        address: &AccAddress,
        config: &Config,
    ) -> Option<QueryLinkedDataResponse> {
        self._query_linked_data(address, config)
    }
}
