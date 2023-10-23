use std::str::FromStr;

use bytes::Bytes;
use ibc_proto::{google::protobuf::Any, protobuf::Protobuf};
use prost::Message as ProstMessage;
use proto_types::AccAddress;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "@type")]
pub enum Message {
    #[serde(rename = "/st.store.v1beta1.MsgVal")]
    Store(MsgVal),
    // TODO: design request
    Get(MsgVal),
}

impl proto_messages::cosmos::tx::v1beta1::Message for Message {
    fn get_signers(&self) -> Vec<&AccAddress> {
        match &self {
            Message::Store(msg) => vec![&msg.address],
            Message::Get(msg) => vec![&msg.address],
        }
    }

    fn validate_basic(&self) -> Result<(), String> {
        match &self {
            Message::Store(_) => Ok(()),
            Message::Get(_) => Ok(()),
        }
    }
}

impl From<Message> for Any {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Store(msg) => Any {
                type_url: "/st.store.v1beta1.MsgVal".to_string(),
                value: msg.encode_vec(),
            },
            Message::Get(msg) => Any {
                type_url: "/st.store.v1beta1.MsgVal".to_string(),
                value: msg.encode_vec(),
            },
        }
    }
}

impl TryFrom<Any> for Message {
    type Error = proto_messages::Error;

    fn try_from(value: Any) -> Result<Self, Self::Error> {
        match value.type_url.as_str() {
            "/st.store.v1beta1.MsgVal" => {
                let msg = MsgVal::decode::<Bytes>(value.value.clone().into())?;
                Ok(Message::Store(msg))
            }
            _ => Err(proto_messages::Error::DecodeGeneral(
                "message type not recognized".into(),
            )),
        }
    }
}
