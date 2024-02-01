#[cfg(test)]
mod tests {

    use crate::basic::{_EndpointFunctions, _init_env, _test_endpoints, create_pool};

    use ota_database::{
        controls::config_history,
        models::config_history::{ConfigHistory, NewConfigHistory, UpdateConfigHistory},
    };

    #[actix_web::test]
    async fn crud() {
        _init_env();

        let pool = create_pool();

        let contact_functions = _EndpointFunctions {
            index: config_history::index,
            create: config_history::create,
            find: config_history::find,
            update: config_history::update,
            delete: config_history::delete,
        };

        _test_endpoints::<ConfigHistory, NewConfigHistory, UpdateConfigHistory, _, _, _, _, _>(
            "/config",
            pool.clone(),
            NewConfigHistory::random,
            UpdateConfigHistory::random,
            contact_functions,
        )
        .await;
    }
}
