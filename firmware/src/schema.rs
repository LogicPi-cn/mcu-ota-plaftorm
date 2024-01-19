// @generated automatically by Diesel CLI.

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
    firmware_data,
    upgrade_history,
);
