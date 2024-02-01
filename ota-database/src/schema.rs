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
    device_list (id) {
        id -> Int4,
        device_id -> Int8,
        #[max_length = 255]
        device_name -> Varchar,
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
        #[max_length = 255]
        sn -> Varchar,
        #[max_length = 255]
        device_id -> Varchar,
        fwcode -> Int4,
        version_m -> Int4,
        version_n -> Int4,
        version_l -> Int4,
        success -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        phone -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    config_history,
    device_list,
    firmware_data,
    upgrade_history,
    users,
);
