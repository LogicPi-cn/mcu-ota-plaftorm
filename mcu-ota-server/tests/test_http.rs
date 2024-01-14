use std::env;

use log::debug;

#[tokio::test]
async fn test_list_fw_http() {
    // set log level
    env::set_var("RUST_APP_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    // match http_list_all_fw_info("http://127.0.0.1:20000").await {
    //     Ok(firmware_infos) => {
    //         for info in firmware_infos {
    //             debug!("{}", info);
    //         }
    //     }
    //     Err(e) => debug!("Error: {}", e),
    // }

    // assert_eq!(result.len(), 2);
}
