use async_std::task::block_on;
use sqlx::{postgres::PgPool, Error};

use crate::{AccountData, MsgVal, QueryByAccAddressRequestRaw, RawAccountData, RawMsgVal};

/// Connect to postgres database.
pub fn connect(url: impl AsRef<str>) -> Result<PgPool, Error> {
    std::env::set_var("DATABASE_URL", url.as_ref());
    // for now we use sync client
    block_on(PgPool::connect(url.as_ref()))
}

/// Add message into messages table.
pub async fn add_msg(pool: &PgPool, msg: &MsgVal) -> anyhow::Result<i64> {
    let raw_msg = RawMsgVal::from(msg.clone());
    let rec = sqlx::query!(
        r#"
INSERT INTO msgs ( id, address, msg )
VALUES ( $1, $2, $3 )
RETURNING id
        "#,
        raw_msg.id as i64,
        raw_msg.address,
        raw_msg.msg
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn add_linked_data(pool: &PgPool, acc: &AccountData) -> anyhow::Result<i64> {
    let raw_acc = RawAccountData::from(acc.clone());
    let rec = sqlx::query!(
        r#"
INSERT INTO accounts ( id, wallet_address, name, email, phone, address )
VALUES ( $1, $2, $3, $4, $5, $6 )
RETURNING id
        "#,
        raw_acc.id as i64,
        raw_acc.wallet_address,
        raw_acc.name,
        raw_acc.email,
        raw_acc.phone,
        raw_acc.address,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

/// Select all messages of an account.
pub async fn select_msgs_by_addr(
    pool: &PgPool,
    query: &QueryByAccAddressRequestRaw,
) -> anyhow::Result<Vec<RawMsgVal>> {
    let rec = sqlx::query!(
        r#"
SELECT id, address, msg
FROM msgs
WHERE address = $1
ORDER BY id
        "#,
        query.address
    )
    .fetch_all(pool)
    .await?;

    let v = rec
        .into_iter()
        .map(|row| RawMsgVal {
            id: row.id as u64,
            msg: row.msg,
            address: row.address,
        })
        .collect::<Vec<_>>();

    Ok(v)
}

#[cfg(feature = "postgres")]
pub async fn query_linked_data(
    pool: &PgPool,
    query_addr: &String,
) -> anyhow::Result<RawAccountData> {
    let rec = sqlx::query!(
        r#"
SELECT *
FROM accounts
WHERE wallet_address = $1
ORDER BY id
        "#,
        query_addr
    )
    .fetch_one(pool)
    .await?;

    let address = if let Some(addr) = rec.address {
        addr
    } else {
        "".into()
    };
    Ok(RawAccountData {
        id: rec.id as u64,
        wallet_address: rec.wallet_address,
        name: rec.name,
        email: rec.email,
        phone: rec.phone,
        address,
    })
}
