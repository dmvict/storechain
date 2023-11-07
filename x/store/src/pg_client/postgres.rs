use async_std::task::block_on;
use sqlx::{postgres::PgPool, Error};

use crate::{MsgVal, QueryByAccAddressRequestRaw, RawMsgVal};

/// Connect to postgres database.
pub fn connect(url: impl AsRef<str>) -> Result<PgPool, Error> {
    std::env::set_var("DATABASE_URL", url.as_ref());
    // for now we use sync client
    block_on(PgPool::connect(url.as_ref()))
}

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
