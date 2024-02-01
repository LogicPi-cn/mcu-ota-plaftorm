#[cfg(test)]
mod tests {

    use crate::basic::{_EndpointFunctions, _init_env, _test_endpoints, create_pool};

    use ota_database::{
        controls::user,
        models::user::{NewUser, UpdateUser, User},
    };

    #[actix_web::test]
    async fn crud() {
        _init_env();

        let pool = create_pool();

        let contact_functions = _EndpointFunctions {
            index: user::index,
            create: user::create,
            find: user::find,
            update: user::update,
            delete: user::delete,
        };

        _test_endpoints::<User, NewUser, UpdateUser, _, _, _, _, _>(
            "/user",
            pool.clone(),
            NewUser::random,
            UpdateUser::random,
            contact_functions,
        )
        .await;
    }
}
