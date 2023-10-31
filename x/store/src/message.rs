use crate::proto::{AccountData, MsgVal};
use bytes::Bytes;
use ibc_proto::{google::protobuf::Any, protobuf::Protobuf};
use proto_types::AccAddress;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "@type")]
pub enum Message {
    #[serde(rename = "/st.store.v1beta1.MsgVal")]
    Store(MsgVal),
    #[serde(rename = "/st.store.v1beta1.AccountData")]
    Link(AccountData),
    // TODO: design request
    Get(MsgVal),
}

impl proto_messages::cosmos::tx::v1beta1::Message for Message {
    fn get_signers(&self) -> Vec<&AccAddress> {
        match &self {
            Message::Store(msg) => vec![&msg.address],
            Message::Link(msg) => vec![&msg.wallet_address],
            Message::Get(msg) => vec![&msg.address],
        }
    }

    fn validate_basic(&self) -> Result<(), String> {
        match &self {
            Message::Store(_) => Ok(()),
            Message::Link(_) => Ok(()),
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
            Message::Link(msg) => Any {
                type_url: "/st.store.v1beta1.AccountData".to_string(),
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
            "/st.store.v1beta1.AccountData" => {
                let msg = AccountData::decode::<Bytes>(value.value.clone().into())?;
                Ok(Message::Link(msg))
            }
            _ => Err(proto_messages::Error::DecodeGeneral(
                "message type not recognized".into(),
            )),
        }
    }
}
