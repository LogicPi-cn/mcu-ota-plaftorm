-- Your SQL goes here

CREATE TABLE IF NOT EXISTS firmware_data (
    id            SERIAL PRIMARY KEY,
		fwcode        INTEGER   NOT NULL,
		version_m     INTEGER   NOT NULL,
		version_n     INTEGER   NOT NULL,
		version_l     INTEGER   NOT NULL,
		fwsize        INTEGER   NOT NULL,
    fwdata        BYTEA     NOT NULL,
    created_at    TIMESTAMP NOT NULL DEFAULT(NOW()), -- COMMENT '创建时间',
    updated_at    TIMESTAMP NOT NULL DEFAULT(NOW())  -- COMMENT '更新时间',
);


CREATE TABLE IF NOT EXISTS upgrade_history (
    id            SERIAL PRIMARY KEY,
    sn            INTEGER   NOT NULL,
    device_id     BIGINT    NOT NULL,
    fwcode        INTEGER   NOT NULL,
		version_m     INTEGER   NOT NULL,
		version_n     INTEGER   NOT NULL,
		version_l     INTEGER   NOT NULL,
    success       BOOLEAN   NOT NULL,
    created_at    TIMESTAMP NOT NULL DEFAULT(NOW()), -- COMMENT '创建时间',
    updated_at    TIMESTAMP NOT NULL DEFAULT(NOW())  -- COMMENT '更新时间',
);