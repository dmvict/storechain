use ibc_proto::protobuf::Protobuf;
use prost::Message as ProstMessage;
use proto_types::AccAddress;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, ProstMessage)]
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

#[derive(Clone, PartialEq, Serialize, Deserialize, ProstMessage)]
pub struct QueryAllMessagesRequestRaw {
    #[prost(string, tag = "1")]
    pub address: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryAllMessagesRequest {
    /// address is the address to query balances for.
    pub address: proto_types::AccAddress,
}

impl TryFrom<QueryAllMessagesRequestRaw> for QueryAllMessagesRequest {
    type Error = String;
    fn try_from(src: QueryAllMessagesRequestRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            address: AccAddress::from_str(&src.address).unwrap(),
        })
    }
}

impl From<QueryAllMessagesRequest> for QueryAllMessagesRequestRaw {
    fn from(src: QueryAllMessagesRequest) -> Self {
        Self {
            address: src.address.into(),
        }
    }
}

impl Protobuf<QueryAllMessagesRequestRaw> for QueryAllMessagesRequest {}

#[derive(Serialize, Deserialize, Clone, ProstMessage)]
pub struct QueryAllMessagesResponse {
    #[prost(message, repeated, tag = "1")]
    pub messages: Vec<RawMsgVal>,
}

impl Protobuf<QueryAllMessagesResponse> for QueryAllMessagesResponse {}

#[derive(Clone, PartialEq, Serialize, Deserialize, ProstMessage)]
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