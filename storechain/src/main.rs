use anyhow::Result;
use auth::Keeper as AuthKeeper;
use bank::Keeper as BankKeeper;
use client::query_command_handler;
use client::tx_command_handler;
use gears::x::params::Keeper as ParamsKeeper;
use rest::get_router;

use crate::genesis::GenesisState;
use crate::handler::Handler;
use crate::store_keys::{StoreChainParamsStoreKey, StoreChainStoreKey};

mod client;
mod config;
mod genesis;
mod handler;
mod message;
mod rest;
mod store_keys;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("GIT_HASH");

fn main() -> Result<()> {
    let params_keeper = ParamsKeeper::new(StoreChainStoreKey::Params);

    let auth_keeper = AuthKeeper::new(
        StoreChainStoreKey::Auth,
        params_keeper.clone(),
        StoreChainParamsStoreKey::Auth,
    );

    let bank_keeper = BankKeeper::new(
        StoreChainStoreKey::Bank,
        params_keeper.clone(),
        StoreChainParamsStoreKey::Bank,
        auth_keeper.clone(),
    );

    gears::app::run(
        APP_NAME,
        VERSION,
        GenesisState::default(),
        bank_keeper,
        auth_keeper,
        params_keeper,
        StoreChainParamsStoreKey::BaseApp,
        |cfg| Handler::new(cfg),
        query_command_handler,
        tx_command_handler,
        get_router(),
    )
}
