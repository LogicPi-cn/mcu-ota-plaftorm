use ::r2d2::Pool;
use actix_http::Request;
use actix_web::body::BoxBody;
use actix_web::dev::{HttpServiceFactory, Service, ServiceResponse};
use actix_web::http::Error;
use firmware::models::basic::HasId;
use firmware::DbPool;
use log::debug;
use std::sync::Once;

use actix_web::{body, test, web, App};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

static INIT: Once = Once::new();

pub fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool.");

    pool
}

pub fn _init_env() {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
        std::env::set_var("RUST_APP_LOG", "debug");
        pretty_env_logger::init_custom_env("RUST_APP_LOG");
    });
}

pub struct _EndpointFunctions<F1, F2, F3, F4, F5> {
    pub index: F1,
    pub create: F2,
    pub find: F3,
    pub update: F4,
    pub delete: F5,
}

pub async fn _test_endpoints<T0, Tn, Tu, F1, F2, F3, F4, F5>(
    path: &str,
    pool: DbPool,
    new_data_fn: fn() -> Tn,
    update_data_fn: fn() -> Tu,
    functions: _EndpointFunctions<F1, F2, F3, F4, F5>,
) where
    T0: Serialize
        + Default
        + Debug
        + std::cmp::PartialEq
        + for<'de> serde::Deserialize<'de>
        + HasId,
    Tn: Serialize + Debug + for<'de> serde::Deserialize<'de> + Clone,
    Tu: Serialize + Debug + for<'de> serde::Deserialize<'de> + Clone,
    F1: 'static + HttpServiceFactory,
    F2: 'static + HttpServiceFactory,
    F3: 'static + HttpServiceFactory,
    F4: 'static + HttpServiceFactory,
    F5: 'static + HttpServiceFactory,
{
    // run the service
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // .wrap(Logger::default())
            .service(
                web::scope(path)
                    .service(functions.index)
                    .service(functions.create)
                    .service(functions.find)
                    .service(functions.update)
                    .service(functions.delete),
            ),
    )
    .await;

    // create
    let new_data = new_data_fn();
    let inserted_data: T0 = _create_data(&app, path, new_data.clone()).await.unwrap();
    debug!("Insert : {:?}", inserted_data);

    // find
    let new_id = inserted_data.id();
    let found_data: T0 = _find_data(&app, path, new_id).await.unwrap();
    debug!("Found  : {:?}", found_data);
    assert_eq!(found_data, inserted_data);

    // update
    let update_data = update_data_fn();
    debug!("Updated: {:?}", update_data);
    _update_data(&app, update_data, path, new_id).await;

    // delete
    debug!("Deleted: {:?}", &new_data.clone());
    _delete_data(&app, &path, new_id).await;
}

async fn _create_data<T0, T1, T2>(app: &T1, path: &str, new_data: T2) -> Result<T0, Error>
where
    T1: Service<Request, Response = ServiceResponse<BoxBody>, Error = actix_web::error::Error>,
    T2: Serialize,
    T0: DeserializeOwned,
{
    let req = test::TestRequest::post()
        .uri(path)
        .set_json(&new_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = resp.into_body();
    let bytes = body::to_bytes(body).await.unwrap();
    let reader = std::io::Cursor::new(bytes.to_vec());
    let inserted_data: T0 = serde_json::from_reader(reader).unwrap();

    Ok(inserted_data)
}

async fn _find_data<T1, T2>(app: &T1, path: &str, id: i32) -> Result<T2, Error>
where
    T1: Service<Request, Response = ServiceResponse<BoxBody>, Error = actix_web::error::Error>,
    T2: DeserializeOwned,
{
    let req_url = format!("{}/{}", path, id);
    let req = test::TestRequest::get().uri(&req_url).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let bytes = test::read_body(resp).await;
    let ssss = std::str::from_utf8(&bytes).unwrap();
    let found_data: T2 = serde_json::from_str(ssss).unwrap();

    Ok(found_data)
}

async fn _update_data<T1, T2>(app: &T1, update_data: T2, path: &str, id: i32)
where
    T1: Service<Request, Response = ServiceResponse<BoxBody>, Error = actix_web::error::Error>,
    T2: Serialize,
{
    let req_url = format!("{}/{}", path, id);
    let req = test::TestRequest::patch()
        .uri(&req_url)
        .set_json(&update_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

async fn _delete_data<T1>(app: &T1, path: &str, id: i32)
where
    T1: Service<Request, Response = ServiceResponse<BoxBody>, Error = actix_web::error::Error>,
{
    let req_url = format!("{}/{}", path, id);
    let req = test::TestRequest::delete().uri(&req_url).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
