use ibc_proto::protobuf::Protobuf;
use prost::Message;
use proto_types::AccAddress;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Message)]
pub struct RawMsgKeyPair {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(int64, tag = "2")]
    pub pubkey_time: i64,
    #[prost(string, tag = "3")]
    pub public_key: String,
    #[prost(string, tag = "4")]
    pub private_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MsgKeyPair {
    pub id: u32,
    pub pubkey_time: i64,
    pub public_key: String,
    pub private_key: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Message)]
pub struct QueryByAccAddressRequestRaw {
    #[prost(string, tag = "1")]
    pub address: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryByAccAddressRequest {
    /// address is the address to query balances for.
    pub address: proto_types::AccAddress,
}

impl TryFrom<QueryByAccAddressRequestRaw> for QueryByAccAddressRequest {
    type Error = String;
    fn try_from(src: QueryByAccAddressRequestRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            address: AccAddress::from_str(&src.address).unwrap(),
        })
    }
}

impl From<QueryByAccAddressRequest> for QueryByAccAddressRequestRaw {
    fn from(src: QueryByAccAddressRequest) -> Self {
        Self {
            address: src.address.into(),
        }
    }
}

impl Protobuf<QueryByAccAddressRequestRaw> for QueryByAccAddressRequest {}

// ===================================
// ============ Responses ============
// ===================================

#[derive(Serialize, Deserialize, Clone, Message)]
pub struct QueryLinkedDataResponse {
    #[prost(message, required, tag = "1")]
    pub data: RawAccountData,
}

impl Protobuf<QueryLinkedDataResponse> for QueryLinkedDataResponse {}

#[derive(Serialize, Deserialize, Clone, Message)]
pub struct QueryAllMessagesResponse {
    #[prost(message, repeated, tag = "1")]
    pub messages: Vec<RawMsgVal>,
}

impl Protobuf<QueryAllMessagesResponse> for QueryAllMessagesResponse {}

// =========================================
// ============ Message structs ============
// =========================================

#[derive(Clone, PartialEq, Serialize, Deserialize, Message)]
pub struct RawMsgVal {
    #[prost(string, tag = "1")]
    pub address: String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    #[prost(string, tag = "3")]
    pub msg: String,
}

impl From<MsgVal> for RawMsgVal {
    fn from(src: MsgVal) -> Self {
        Self {
            address: src.address.into(),
            id: src.id,
            msg: src.msg,
        }
    }
}

/// Struct that keeps message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsgVal {
    pub address: AccAddress,
    pub id: u64,
    pub msg: String,
}

impl Protobuf<RawMsgVal> for MsgVal {}

impl From<RawMsgVal> for MsgVal {
    fn from(src: RawMsgVal) -> Self {
        Self {
            address: AccAddress::from_bech32(&src.address).unwrap(),
            id: src.id,
            msg: src.msg,
        }
    }
}

//
#[derive(Clone, PartialEq, Serialize, Deserialize, Message)]
pub struct RawAccountData {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub wallet_address: String,
    #[prost(string, tag = "3")]
    pub name: String,
    #[prost(string, tag = "4")]
    pub email: String,
    #[prost(string, tag = "5")]
    pub phone: String,
    #[prost(string, tag = "6")]
    pub address: String,
}

impl From<AccountData> for RawAccountData {
    fn from(src: AccountData) -> Self {
        let address = if let Some(address) = src.address {
            address
        } else {
            "".into()
        };
        Self {
            id: src.id,
            wallet_address: src.wallet_address.into(),
            name: src.name,
            email: src.email,
            phone: src.phone,
            address,
        }
    }
}

/// Keep account metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountData {
    pub id: u64,
    pub wallet_address: AccAddress,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Option<String>,
}

impl Protobuf<RawAccountData> for AccountData {}

impl From<RawAccountData> for AccountData {
    fn from(src: RawAccountData) -> Self {
        let address = if src.address.is_empty() {
            None
        } else {
            Some(src.address)
        };
        Self {
                id: src.id,
                wallet_address: AccAddress::from_bech32(&src.wallet_address).unwrap(),
                name: src.name,
                email: src.email,
                phone: src.phone,
                address,
        }
    }
}
