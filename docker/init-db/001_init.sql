-- MCU OTA Platform Database Initialization Script
-- 数据库初始化脚本

-- 启用扩展
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 固件数据表
CREATE TABLE IF NOT EXISTS firmware_data (
    id            SERIAL    PRIMARY KEY,
    fwcode        INTEGER   NOT NULL,
    version_m     INTEGER   NOT NULL,
    version_n     INTEGER   NOT NULL,
    version_l     INTEGER   NOT NULL,
    fwsize        INTEGER   NOT NULL,
    fwdata        BYTEA     NOT NULL,
    created_at    TIMESTAMP NOT NULL DEFAULT(NOW()),
    updated_at    TIMESTAMP NOT NULL DEFAULT(NOW())
);

-- 升级历史表
CREATE TABLE IF NOT EXISTS upgrade_history (
    id            SERIAL PRIMARY KEY,
    sn            VARCHAR(255)   NOT NULL,
    device_id     VARCHAR(255)   NOT NULL,
    fwcode        INTEGER        NOT NULL,
    version_m     INTEGER        NOT NULL,
    version_n     INTEGER        NOT NULL,
    version_l     INTEGER        NOT NULL,
    success       BOOLEAN        NOT NULL,
    created_at    TIMESTAMP      NOT NULL DEFAULT(NOW()),
    updated_at    TIMESTAMP      NOT NULL DEFAULT(NOW())
);

-- 配置历史表
CREATE TABLE IF NOT EXISTS config_history (
    id            SERIAL    PRIMARY KEY,
    group_id      INTEGER   NOT NULL,
    op_code       INTEGER   NOT NULL,
    sync_ts       TIMESTAMP NOT NULL,
    interval      INTEGER   NOT NULL,
    t_max         INTEGER   NOT NULL,
    t_min         INTEGER   NOT NULL,
    human         BOOLEAN   NOT NULL,
    created_at    TIMESTAMP NOT NULL DEFAULT(NOW()),
    updated_at    TIMESTAMP NOT NULL DEFAULT(NOW())
);

-- 设备列表
CREATE TABLE IF NOT EXISTS device_list (
    id            SERIAL       PRIMARY KEY,
    device_id     BIGINT       NOT NULL,
    device_name   VARCHAR(255) NOT NULL,
    created_at    TIMESTAMP    NOT NULL DEFAULT(NOW()),
    updated_at    TIMESTAMP    NOT NULL DEFAULT(NOW())
);

-- 用户表
CREATE TABLE IF NOT EXISTS users (
    id            UUID         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    username      VARCHAR(100) NOT NULL,
    password      VARCHAR(100) NOT NULL,
    email         VARCHAR(100) NOT NULL,
    verified      BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at    TIMESTAMP    NOT NULL DEFAULT(NOW()),
    updated_at    TIMESTAMP    NOT NULL DEFAULT(NOW())
);

-- 创建索引
CREATE INDEX IF NOT EXISTS users_email_idx ON users (email);
CREATE INDEX IF NOT EXISTS firmware_data_fwcode_idx ON firmware_data (fwcode);
CREATE INDEX IF NOT EXISTS upgrade_history_fwcode_idx ON upgrade_history (fwcode);
CREATE INDEX IF NOT EXISTS config_history_group_id_idx ON config_history (group_id);
