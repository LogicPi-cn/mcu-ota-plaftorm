#[cfg(test)]
mod tests {

    use crate::basic::{_EndpointFunctions, _init_env, _test_endpoints, create_pool};

    use ota_database::{
        controls::upgrade_history,
        models::upgrade_history::{NewUpgradeHistory, UpdateUpgradeHistory, UpgradeHistory},
    };

    #[actix_web::test]
    async fn crud() {
        _init_env();

        let pool = create_pool();

        let contact_functions = _EndpointFunctions {
            index: upgrade_history::index,
            create: upgrade_history::create,
            find: upgrade_history::find,
            update: upgrade_history::update,
            delete: upgrade_history::delete,
        };

        _test_endpoints::<UpgradeHistory, NewUpgradeHistory, UpdateUpgradeHistory, _, _, _, _, _>(
            "/history",
            pool.clone(),
            NewUpgradeHistory::random,
            UpdateUpgradeHistory::random,
            contact_functions,
        )
        .await;
    }
}
