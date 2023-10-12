use gears::x::params::ParamsSubspaceKey;
use store::StoreKey;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Clone)]
pub enum StoreChainStoreKey {
    Bank,
    Auth,
    Params,
    Store,
}

/// WARNING: a key name must not be a prefix of another, there is currently
/// no check in the SDK to prevent this.
impl StoreKey for StoreChainStoreKey {
    fn name(&self) -> &'static str {
        match self {
            StoreChainStoreKey::Bank => "bank",
            StoreChainStoreKey::Auth => "acc",
            StoreChainStoreKey::Params => "params",
            StoreChainStoreKey::Store => "store",
        }
    }
}

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Clone)]
pub enum StoreChainParamsStoreKey {
    Bank,
    Auth,
    BaseApp,
}

/// WARNING: a key name must not be a prefix of another, there is currently
/// no check in the SDK to prevent this.
impl ParamsSubspaceKey for StoreChainParamsStoreKey {
    fn name(&self) -> &'static str {
        match self {
            Self::Bank => "bank/",
            Self::Auth => "auth/",
            Self::BaseApp => "baseapp/",
        }
    }
}
