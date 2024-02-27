-- Your SQL goes here

CREATE TABLE IF NOT EXISTS firmware_data (
    id            SERIAL    PRIMARY KEY,
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
    sn            VARCHAR(255)   NOT NULL,
    device_id     VARCHAR(255)   NOT NULL,
    fwcode        INTEGER        NOT NULL,
    version_m     INTEGER        NOT NULL,
    version_n     INTEGER        NOT NULL,
    version_l     INTEGER        NOT NULL,
    success       BOOLEAN        NOT NULL,
    created_at    TIMESTAMP      NOT NULL DEFAULT(NOW()), -- COMMENT '创建时间',
    updated_at    TIMESTAMP      NOT NULL DEFAULT(NOW())  -- COMMENT '更新时间',
);

CREATE TABLE IF NOT EXISTS config_history (
    id            SERIAL    PRIMARY KEY,
    group_id      INTEGER   NOT NULL,   -- 分组id
    op_code       INTEGER   NOT NULL,   -- 操作码
    sync_ts       TIMESTAMP NOT NULL,   -- 同步时间
    interval      INTEGER   NOT NULL,   -- 上报时间间隔
    t_max         INTEGER   NOT NULL,   -- 温度报警上限
    t_min         INTEGER   NOT NULL,   -- 温度报警下限
    human         BOOLEAN   NOT NULL,   -- 人体感应
    created_at    TIMESTAMP NOT NULL DEFAULT(NOW()), -- COMMENT '创建时间',
    updated_at    TIMESTAMP NOT NULL DEFAULT(NOW())  -- COMMENT '更新时间',
);

CREATE TABLE IF NOT EXISTS device_list (
    id            SERIAL       PRIMARY KEY,
    device_id     BIGINT       NOT NULL,
    device_name   VARCHAR(255) NOT NULL,
    created_at    TIMESTAMP    NOT NULL DEFAULT(NOW()), -- COMMENT '创建时间',
    updated_at    TIMESTAMP    NOT NULL DEFAULT(NOW())  -- COMMENT '更新时间',
);

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()), 
    username      VARCHAR(100) NOT NULL,
    password      VARCHAR(100) NOT NULL,
    email         VARCHAR(100) NOT NULL,
    verified      BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at    TIMESTAMP    NOT NULL DEFAULT(NOW()), -- COMMENT '创建时间',
    updated_at    TIMESTAMP    NOT NULL DEFAULT(NOW())  -- COMMENT '更新时间',
);

CREATE INDEX users_email_idx ON users (email);