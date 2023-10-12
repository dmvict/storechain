// use std::{collections::HashMap, thread};
//
// use bytes::Bytes;
use database::{Database, PrefixDB};
use gears::{
    error::AppError,
    types::context::{Context, QueryContext, TxContext},
};
// use prost::Message;
use store::{MutablePrefixStore, StoreKey};

use crate::{message::{MsgVal, RawMsgVal, RawMsgKeyPair}, /* utils::run_tx_command, */ Config};

// use chrono::Utc;

const MSG_DATA_KEY: [u8; 1] = [0];
const KEYPAIR_DATA_KEY: [u8; 1] = [1];

#[derive(Debug, Clone)]
pub struct Keeper<SK: StoreKey> {
    store_key: SK,
}

impl<SK: StoreKey> Keeper<SK> {
    pub fn new(store_key: SK) -> Self {
        Keeper { store_key }
    }

    pub fn open_process_count<T: Database>(
        &self,
        ctx: &mut TxContext<T, SK>,
        msg: String,
    ) -> u32 {
        let tlcs_store = ctx.get_kv_store(&self.store_key);

        let mut store_key = KEYPAIR_DATA_KEY.to_vec();
        store_key.append(&mut msg.to_le_bytes().to_vec());

        let prefix_store = tlcs_store.get_immutable_prefix_store(store_key);
        let the_keys = prefix_store.range(..);
        the_keys.count() as u32
    }

    pub fn store_message<T: Database>(
        &self,
        ctx: &mut TxContext<T, SK>,
        msg: &MsgVal,
    ) -> Result<(), AppError> {
        // Make store key
        let mut store_key = MSG_DATA_KEY.to_vec();
        store_key.append(&mut msg.msg.to_le_bytes().to_vec());

        let addr: Vec<u8> = msg.address.clone().into();
        store_key.append(&mut addr.to_vec());

        //
        let keycount = self.open_process_count(ctx, msg.msg);

        if msg.id <= (keycount - 1) {
            let tlcs_store = ctx.get_mutable_kv_store(&self.store_key);
            let chain_data: RawMsgVal = msg.to_owned().into();
            tlcs_store.set(store_key.into(), chain_data.encode_to_vec());
        } else {
            return Err(AppError::InvalidRequest(
                "Can't contribute data without existing keypair request.".into(),
            ));
        }

        Ok(())
    }

    pub fn get_empty_keypairs<'a, T: Database>(
        &self,
        ctx: &'a mut TxContext<T, SK>,
    ) -> (
        HashMap<Vec<u8>, RawMsgKeyPair>,
        HashMap<Vec<u8>, RawMsgKeyPair>,
    ) {
        let mut need_pub_key: HashMap<Vec<u8>, RawMsgKeyPair> = HashMap::new();
        let mut need_priv_key: HashMap<Vec<u8>, RawMsgKeyPair> = HashMap::new();

        let tlcs_store = ctx.get_kv_store(&self.store_key);
        let store_key = KEYPAIR_DATA_KEY.to_vec();
        let prefix_store = tlcs_store.get_immutable_prefix_store(store_key);
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

    pub fn make_public_keys<'a, T: Database>(
        &self,
        ctx: &'a mut TxContext<T, SK>,
        new_key_list: HashMap<Vec<u8>, RawMsgKeyPair>,
        cur_time: i64,
        contribution_threshold: u32,
    ) {
        let mut tmp_store: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();

        for (key, mut keypair) in new_key_list {
            let mut all_participant_data: Vec<Vec<u8>> = vec![];
            let mut contrib_count: u32 = 0;

            if keypair.pubkey_time < cur_time {
                let round_all_participant_data =
                    self.get_this_round_all_participant_data(ctx, keypair.round, keypair.scheme);

                for (_, row) in round_all_participant_data {
                    let contribution: RawMsgContribution =
                        RawMsgContribution::decode::<Bytes>(row.into())
                            .expect("invalid data in database - possible database corruption");

                    all_participant_data.push(contribution.data);
                    contrib_count += 1;
                }

                if contrib_count > contribution_threshold {
                    info!("MAKE_PK: making key for round: {:?}", keypair.round);
                    let public_key =
                        make_public_key(scheme_to_string(keypair.scheme), &all_participant_data);
                    keypair.public_key = hex::encode(&public_key);

                    tmp_store.insert(key, keypair.encode_to_vec());
                }
            }
        }

        let tlcs_store = ctx.get_mutable_kv_store(&self.store_key);
        for (mut k, v) in tmp_store {
            info!("MAKE_PK: storing new key");
            let mut prefix = KEYPAIR_DATA_KEY.to_vec();
            prefix.append(&mut k);
            tlcs_store.set(prefix, v)
        }
    }

    pub fn make_secret_keys<'a, T: Database>(
        &self,
        ctx: &'a mut TxContext<T, SK>,
        new_key_list: HashMap<Vec<u8>, RawMsgKeyPair>,
    ) {
        let mut tmp_store: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut loe_signature: String;

        for (key, mut keypair) in new_key_list {
            let mut all_participant_data: Vec<Vec<u8>> = vec![];

            match self.get_this_round_loe_signature(ctx, keypair.round) {
                Some(data) => {
                    loe_signature = data;
                }
                None => continue,
            }

            let round_all_participant_data =
                self.get_this_round_all_participant_data(ctx, keypair.round, keypair.scheme);

            for (_, row) in round_all_participant_data {
                let contribution: RawMsgContribution =
                    RawMsgContribution::decode::<Bytes>(row.into())
                        .expect("invalid data in database - possible database corruption");

                all_participant_data.push(contribution.data.clone());
            }

            let secret_key = make_secret_key(
                scheme_to_string(keypair.scheme),
                loe_signature,
                all_participant_data,
            );

            keypair.private_key = hex::encode(secret_key);
            tmp_store.insert(key, keypair.encode_to_vec());
        }

        let tlcs_store = ctx.get_mutable_kv_store(&self.store_key);
        for (mut k, v) in tmp_store {
            let mut prefix = KEYPAIR_DATA_KEY.to_vec();
            prefix.append(&mut k);
            tlcs_store.set(prefix, v)
        }
    }

    pub fn query_all_messages<T: Database>(
        &self,
        ctx: &QueryContext<T, SK>,
    ) -> Vec<RawMsgVal> {
        let store_key = MSG_DATA_KEY.to_vec();

        let tlcs_store = ctx.get_kv_store(&self.store_key);
        let prefix_store = tlcs_store.get_immutable_prefix_store(store_key);
        let all_raw_data = prefix_store.range(..);

        let mut contributions = vec![];

        for (_, row) in all_raw_data {
            let contribution: RawMsgVal = RawMsgVal::decode::<Bytes>(row.into())
                .expect("invalid data in database - possible database corruption");
            contributions.push(contribution);
        }
        contributions
    }
}
