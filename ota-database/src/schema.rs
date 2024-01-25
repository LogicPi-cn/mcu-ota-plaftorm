// @generated automatically by Diesel CLI.

diesel::table! {
    config_history (id) {
        id -> Int4,
        group_id -> Int4,
        op_code -> Int4,
        sync_ts -> Timestamp,
        interval -> Int4,
        t_max -> Int4,
        t_min -> Int4,
        human -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    firmware_data (id) {
        id -> Int4,
        fwcode -> Int4,
        version_m -> Int4,
        version_n -> Int4,
        version_l -> Int4,
        fwsize -> Int4,
        fwdata -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    upgrade_history (id) {
        id -> Int4,
        sn -> Int4,
        device_id -> Int8,
        fwcode -> Int4,
        version_m -> Int4,
        version_n -> Int4,
        version_l -> Int4,
        success -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    config_history,
    firmware_data,
    upgrade_history,
);
