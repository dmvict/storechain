CREATE TABLE IF NOT EXISTS accounts
(
    id                 BIGSERIAL PRIMARY KEY,
    wallet_address     TEXT    NOT NULL UNIQUE,
    name               TEXT    NOT NULL,
    email              TEXT    NOT NULL,
    phone              TEXT    NOT NULL,
    address            TEXT
);
