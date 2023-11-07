use anyhow::Result;
use auth::Keeper as AuthKeeper;
use bank::Keeper as BankKeeper;
use client::query_command_handler;
use client::tx_command_handler;
use gears::utils::get_default_home_dir;
use gears::x::params::Keeper as ParamsKeeper;
use rest::get_router;

use crate::genesis::GenesisState;
use crate::handler::Handler;
use crate::store_keys::{StoreChainParamsStoreKey, StoreChainStoreKey};
use st::Config;

mod client;
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

    let mut home_dir = get_default_home_dir(APP_NAME).unwrap();
    home_dir.push("config/app_conf.toml");
    let contents = match std::fs::read_to_string(home_dir) {
        Ok(s) => s,
        Err(_) => {
            panic!("Could not read file app_conf.toml");
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            panic!("File is corrupt");
        }
    };

    gears::app::run(
        APP_NAME,
        VERSION,
        GenesisState::default(),
        bank_keeper,
        auth_keeper,
        params_keeper,
        StoreChainParamsStoreKey::BaseApp,
        Handler::new(config),
        query_command_handler,
        tx_command_handler,
        get_router(),
    )
}
