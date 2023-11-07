CREATE TABLE IF NOT EXISTS msgs
(
    id          BIGSERIAL PRIMARY KEY,
    address     TEXT    NOT NULL,
    msg         TEXT    NOT NULL
);
