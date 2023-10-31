use std::collections::HashMap;

use bytes::Bytes;
use database::Database;
use gears::{
    error::AppError,
    types::context::{QueryContext, TxContext},
};
use proto_types::AccAddress;
use prost::Message;
use store::StoreKey;

use crate::{MsgVal, QueryByAccAddressRequest, QueryAllMessagesResponse, RawMsgKeyPair, RawMsgVal, AccountData, RawAccountData, QueryLinkedDataResponse};

const MSG_DATA_KEY: [u8; 1] = [0];
const KEYPAIR_DATA_KEY: [u8; 1] = [1];
const METADATA_DATA_KEY: [u8; 1] = [2];

#[derive(Debug, Clone)]
pub struct Keeper<SK: StoreKey> {
    store_key: SK,
}

impl<SK: StoreKey> Keeper<SK> {
    pub fn new(store_key: SK) -> Self {
        Keeper { store_key }
    }

    pub fn store_message<T: Database>(
        &self,
        ctx: &mut TxContext<T, SK>,
        msg: &MsgVal,
    ) -> Result<(), AppError> {
        // Make store key
        let mut store_key = MSG_DATA_KEY.to_vec();
        let addr: Vec<u8> = msg.address.clone().into();
        store_key.append(&mut addr.to_vec());
        let msgv: Vec<u8> = msg.msg.clone().into();
        store_key.append(&mut msgv.to_vec());

        let metadata = self.tx_linked_data(ctx, &msg.address);

        if metadata.is_some() {
            let msg_store = ctx.get_mutable_kv_store(&self.store_key);
            let chain_data: RawMsgVal = msg.to_owned().into();
            msg_store.set(store_key, chain_data.encode_to_vec());
        } else {
            return Err(AppError::InvalidRequest(
                "Can't contribute data without existing account.".into(),
            ));
        }

        Ok(())
    }

    pub fn get_message<T: Database>(
        &self,
        ctx: &mut TxContext<T, SK>,
        msg: &MsgVal,
    ) -> Result<(), AppError> {
        // Make store key
        let mut store_key = MSG_DATA_KEY.to_vec();
        let addr: Vec<u8> = msg.address.clone().into();
        store_key.append(&mut addr.to_vec());

        //

        let msg_store = ctx.get_mutable_kv_store(&self.store_key);
        msg_store.get(store_key.as_slice());

        Ok(())
    }

    pub fn get_empty_keypairs<T: Database>(
        &self,
        ctx: &mut TxContext<T, SK>,
    ) -> (
        HashMap<Vec<u8>, RawMsgKeyPair>,
        HashMap<Vec<u8>, RawMsgKeyPair>,
    ) {
        let mut need_pub_key: HashMap<Vec<u8>, RawMsgKeyPair> = HashMap::new();
        let mut need_priv_key: HashMap<Vec<u8>, RawMsgKeyPair> = HashMap::new();

        let msg_store = ctx.get_kv_store(&self.store_key);
        let store_key = KEYPAIR_DATA_KEY.to_vec();
        let prefix_store = msg_store.get_immutable_prefix_store(store_key);
        let keypairs = prefix_store.range(..);

        for (index, keypair) in keypairs {
            let the_keys: RawMsgKeyPair = RawMsgKeyPair::decode::<Bytes>(keypair.into())
                .expect("invalid data in database - possible database corruption");
            if the_keys.public_key.len() == 0 {
                need_pub_key.insert(index.into(), the_keys);
            } else if the_keys.private_key.len() == 0 {
                need_priv_key.insert(index.into(), the_keys);
            }
        }

        return (need_pub_key, need_priv_key);
    }

    pub fn query_all_messages<T: Database>(
        &self,
        ctx: &QueryContext<T, SK>,
    ) -> QueryAllMessagesResponse {
        let store_key = MSG_DATA_KEY.to_vec();

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
