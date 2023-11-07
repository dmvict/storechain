use ibc_proto::protobuf::Protobuf;
use proto_types::AccAddress;
use tendermint_abci::Application;

use crate::QueryAllMessagesResponse;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use gears::{
    baseapp::{
        ante::{AuthKeeper, BankKeeper},
        BaseApp, Genesis, Handler,
    },
    client::rest::{error::Error, Pagination, RestState},
    x::params::ParamsSubspaceKey,
};
use proto_messages::cosmos::{bank::v1beta1::QueryAllBalancesRequest, tx::v1beta1::Message};
use store::StoreKey;
use tendermint_proto::abci::RequestQuery;

/// Get all balances for a given address
pub async fn get_messages<
    SK: StoreKey,
    PSK: ParamsSubspaceKey,
    M: Message,
    BK: BankKeeper<SK>,
    AK: AuthKeeper<SK>,
    H: Handler<M, SK, G>,
    G: Genesis,
>(
    Path(address): Path<AccAddress>,
    _pagination: Query<Pagination>,
    State(app): State<BaseApp<SK, PSK, M, BK, AK, H, G>>,
) -> Result<Json<QueryAllMessagesResponse>, Error> {
    let req = QueryAllBalancesRequest {
        address,
        pagination: None,
    };

    let request = RequestQuery {
        data: req.encode_vec().into(),
        path: "/st.store.v1beta1.Query/GetAllMessagesByAddr".into(),
        height: 0,
        prove: false,
    };

    let response = app.query(request);

    Ok(Json(
        QueryAllMessagesResponse::decode(response.value)
            .expect("should be a valid QueryAllBalancesResponse"),
    ))
}

pub fn get_router<
    SK: StoreKey,
    PSK: ParamsSubspaceKey,
    M: Message,
    BK: BankKeeper<SK>,
    AK: AuthKeeper<SK>,
    H: Handler<M, SK, G>,
    G: Genesis,
>() -> Router<RestState<SK, PSK, M, BK, AK, H, G>, Body> {
    Router::new().route("/v1beta1/messages", get(get_messages))
}
