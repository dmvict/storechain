use gears::{
    config::Config,
    types::context::{InitContext, TxContext},
    x::params::Keeper as ParamsKeeper,
};
use proto_messages::cosmos::base::v1beta1::SendCoins;
use proto_types::AccAddress;
use tendermint_proto::abci::{RequestBeginBlock, RequestQuery};

use database::Database;
use gears::{error::AppError, types::context::QueryContext};

use crate::{
    config::AppConfig,
    genesis::GenesisState,
    message::Message,
    store_keys::{StoreChainParamsStoreKey, StoreChainStoreKey},
};

#[derive(Debug, Clone)]
pub struct Handler {
    bank_handler: bank::Handler<StoreChainStoreKey, StoreChainParamsStoreKey>,
    auth_handler: auth::Handler<StoreChainStoreKey, StoreChainParamsStoreKey>,
    store_handler: bank::Handler<StoreKey>,
}

impl Handler {
    pub fn new(_cfg: Config<AppConfig>) -> Handler {
        let params_keeper = ParamsKeeper::new(StoreChainStoreKey::Params);

        let auth_keeper = auth::Keeper::new(
            StoreChainStoreKey::Auth,
            params_keeper.clone(),
            StoreChainParamsStoreKey::Auth,
        );

        let bank_keeper = bank::Keeper::new(
            StoreChainStoreKey::Bank,
            params_keeper,
            StoreChainParamsStoreKey::Bank,
            auth_keeper.clone(),
        );

        let store_keeper = st::Keeper::new(StoreChainParamsStoreKey::Store);

        Handler {
            bank_handler: bank::Handler::new(bank_keeper),
            auth_handler: auth::Handler::new(auth_keeper),
            store_handler: store::Handler::new(store_keeper),
        }
    }
}

impl gears::baseapp::Handler<Message, StoreChainStoreKey, GenesisState> for Handler {
    fn handle_tx<DB: Database>(
        &self,
        ctx: &mut TxContext<DB, StoreChainStoreKey>,
        msg: &Message,
    ) -> Result<(), AppError> {
        match msg {
            Message::Msg(msg) => self.store_handler.handle(ctx, msg),
        }
    }

    fn handle_init_genesis<DB: Database>(
        &self,
        ctx: &mut InitContext<DB, StoreChainStoreKey>,
        genesis: GenesisState,
    ) {
        self.bank_handler.init_genesis(ctx, genesis.bank);
        self.auth_handler.init_genesis(ctx, genesis.auth);
    }

    fn handle_query<DB: Database>(
        &self,
        ctx: &QueryContext<DB, StoreChainStoreKey>,
        query: RequestQuery,
    ) -> Result<bytes::Bytes, AppError> {
        if query.path.starts_with("/cosmos.auth") {
            self.auth_handler.handle_query(ctx, query)
        } else if query.path.starts_with("/cosmos.bank") {
            self.bank_handler.handle_query(ctx, query)
        } else if query.path.starts_with("/st.store") {
            self.store_handler.handle_query(ctx, query)
        } else {
            Err(AppError::InvalidRequest("query path not found".into()))
        }
    }

    // TODO: move this into the SDK
    fn handle_add_genesis_account(
        &self,
        genesis_state: &mut GenesisState,
        address: AccAddress,
        coins: SendCoins,
    ) -> Result<(), AppError> {
        self.auth_handler
            .handle_add_genesis_account(&mut genesis_state.auth, address.clone())?;
        self.bank_handler
            .handle_add_genesis_account(&mut genesis_state.bank, address, coins);

        Ok(())
    }

    fn handle_begin_block<DB: Database>(
        &self,
        ctx: &mut TxContext<DB, StoreChainStoreKey>,
        request: RequestBeginBlock,
    ) {
        self.store_handler.handle_begin_block(ctx, request)
    }
}
