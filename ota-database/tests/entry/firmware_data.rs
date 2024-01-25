#[cfg(test)]
mod tests {

    use crate::basic::{_EndpointFunctions, _init_env, _test_endpoints, create_pool};

    use ota_database::{
        controls::firmware_data,
        models::firmware_data::{FirmwareData, NewFirmwareData, UpdateFirmwareData},
    };

    #[actix_web::test]
    async fn crud() {
        _init_env();

        let pool = create_pool();

        let contact_functions = _EndpointFunctions {
            index: firmware_data::index,
            create: firmware_data::create,
            find: firmware_data::find,
            update: firmware_data::update,
            delete: firmware_data::delete,
        };

        _test_endpoints::<FirmwareData, NewFirmwareData, UpdateFirmwareData, _, _, _, _, _>(
            "/firmware",
            pool.clone(),
            NewFirmwareData::random,
            UpdateFirmwareData::random,
            contact_functions,
        )
        .await;
    }
}
