#[cfg(test)]
mod tests {

    use std::env;

    use actix_web::{test, App};
    use firmware::{
        common::{FirmwareData, FirmwareInfo, FirmwareVersion},
        from_pg::{read_firmware, Database},
    };
    use log::{error, info};
    use sqlx::postgres::PgPoolOptions;

    #[actix_rt::test]
    async fn test_read_firmware() {
        // set log level
        env::set_var("RUST_LOG", "debug,actix_web=debug");
        pretty_env_logger::init_custom_env("RUST_LOG");

        let pool = match PgPoolOptions::new()
            .connect("postgres://craftor:3.1415926@localhost:50000/firmware")
            .await
        {
            Ok(pool) => {
                info!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                error!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };

        let new_db = Database { db: pool.clone() };

        // 插入一些测试数据
        let test_data = FirmwareData {
            info: FirmwareInfo {
                code: 0x1987,
                version: FirmwareVersion { m: 0, n: 1, l: 0 },
                size: 5,
                path: String::from(""),
            },
            data: vec![1, 2, 3, 4, 5],
        };

        new_db.create_firmware_data(&test_data).await.unwrap();

        // 创建应用并注册路由
        let mut app = test::init_service(App::new().app_data(new_db).service(read_firmware)).await;

        // 创建一个测试请求
        let req = test::TestRequest::get().uri("/firmware/1").to_request();

        // 发送请求并获取响应
        let resp = test::call_service(&mut app, req).await;

        // 检查响应状态
        assert!(resp.status().is_success());

        // 检查响应内容
        let result: FirmwareData = test::read_body_json(resp).await;
        assert_eq!(result, test_data);
    }
}
