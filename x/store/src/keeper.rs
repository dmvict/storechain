use bytes::Bytes;
use database::Database;
use gears::{
    error::AppError,
    types::context::{QueryContext, TxContext},
};
use prost::Message;
use proto_types::AccAddress;
use store::StoreKey;

use crate::{
    AccountData, Config, MsgVal, QueryAllMessagesResponse, QueryByAccAddressRequest,
    QueryLinkedDataResponse,
};

use crate::{RawAccountData, RawMsgVal};

const MSG_DATA_KEY: [u8; 1] = [0];
const METADATA_DATA_KEY: [u8; 1] = [1];

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

    /// Store message in the chain.
    fn store_message_db<T: Database>(
        &self,
        ctx: &mut TxContext<T, SK>,
        msg: &MsgVal,
        _config: &Config,
    ) -> Result<(), AppError> {
        // Make store key
        let mut store_key = MSG_DATA_KEY.to_vec();
        let addr: Vec<u8> = msg.address.clone().into();
        store_key.append(&mut addr.to_vec());
        let msgv: Vec<u8> = msg.msg.clone().into();
        store_key.append(&mut msgv.to_vec());
        let msg_store = ctx.get_mutable_kv_store(&self.store_key);
        let chain_data: RawMsgVal = msg.to_owned().into();
        msg_store.set(store_key, chain_data.encode_to_vec());
        Ok(())
    }

    /// Query messages owned by a user from db.
    pub fn query_all_messages_by_addr<T: Database>(
        &self,
        ctx: &QueryContext<T, SK>,
        req: QueryByAccAddressRequest,
        _config: &Config,
    ) -> QueryAllMessagesResponse {
        let mut store_key = MSG_DATA_KEY.to_vec();
        let addr: Vec<u8> = req.address.clone().into();
        store_key.append(&mut addr.to_vec());

        let msg_store = ctx.get_kv_store(&self.store_key);
        let prefix_store = msg_store.get_immutable_prefix_store(store_key);
        let all_raw_data = prefix_store.range(..);

        let mut messages = vec![];

        for (_, row) in all_raw_data {
            let message: RawMsgVal = RawMsgVal::decode::<Bytes>(row.into())
                .expect("invalid data in database - possible database corruption");
            messages.push(message);
        }
        QueryAllMessagesResponse { messages }
    }

    /// Store account metadata in the separate database.
    pub fn store_metadata<T: Database>(
        &self,
        ctx: &mut TxContext<T, SK>,
        msg: &AccountData,
        _config: &Config,
    ) -> Result<(), AppError> {
        // Make store key
        let mut store_key = METADATA_DATA_KEY.to_vec();
        let addr: Vec<u8> = msg.wallet_address.clone().into();
        store_key.append(&mut addr.to_vec());

        let msg_store = ctx.get_mutable_kv_store(&self.store_key);
        let chain_data: RawAccountData = msg.to_owned().into();
        msg_store.set(store_key, chain_data.encode_to_vec());
        Ok(())
    }

    /// Get linked data using wallet address.
    pub fn query_linked_data<T: Database>(
        &self,
        ctx: &QueryContext<T, SK>,
        req: QueryByAccAddressRequest,
        _config: &Config,
    ) -> Option<QueryLinkedDataResponse> {
        let mut store_key = METADATA_DATA_KEY.to_vec();
        let addr: Vec<u8> = req.address.clone().into();
        store_key.append(&mut addr.to_vec());

        let msg_store = ctx.get_kv_store(&self.store_key);
        if let Some(raw_data) = msg_store.get(&store_key) {
            let data: RawAccountData = RawAccountData::decode::<Bytes>(raw_data.into())
                .expect("invalid data in database - possible database corruption");
            Some(QueryLinkedDataResponse { data })
        } else {
            None
        }
    }

    // TODO: duplicate because QueryKVStore and KVStore is a different enum types. The members are
    // combined into single interface enum AnyKVStore but it is private enum.
    pub fn tx_linked_data<T: Database>(
        &self,
        ctx: &TxContext<T, SK>,
        address: &AccAddress,
        _config: &Config,
    ) -> Option<QueryLinkedDataResponse> {
        let mut store_key = METADATA_DATA_KEY.to_vec();
        let addr: Vec<u8> = address.clone().into();
        store_key.append(&mut addr.to_vec());

        let msg_store = ctx.get_kv_store(&self.store_key);
        if let Some(raw_data) = msg_store.get(&store_key) {
            let data: RawAccountData = RawAccountData::decode::<Bytes>(raw_data.into())
                .expect("invalid data in database - possible database corruption");
            Some(QueryLinkedDataResponse { data })
        } else {
            None
        }
    }
}
