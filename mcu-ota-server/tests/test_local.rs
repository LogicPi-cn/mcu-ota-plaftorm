use std::env;

use log::debug;
use mcu_ota_server::{package::rx_package::send_fw_query_info_pkg, request_process::handle_client};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    task,
};

#[tokio::test]
async fn test_tcp_server() {
    // set log level
    env::set_var("RUST_APP_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    // 启动服务器
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind address");

    // 启动服务器任务
    let server_task = task::spawn(async move {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        // 处理连接的任务
        task::spawn(async move {
            if let Err(error) = handle_client(socket).await {
                eprintln!("Error handling client: {}", error);
            }
        });
    }); // 启动服务器

    // 等待服务器启动
    std::thread::sleep(std::time::Duration::from_secs(1));

    // 连接服务器
    let mut stream = TcpStream::connect("127.0.0.1:8080")
        .await
        .expect("Failed to connect to server");

    // 读取本地固件
    let new_fw_info = get_firmware_info("/home/craftor/ftp", 0x1987).unwrap();
    debug!("{}", new_fw_info);

    // 请求固件信息
    send_fw_query_info_pkg(&new_fw_info, &mut stream)
        .await
        .unwrap();

    // 读取响应
    let mut response = vec![0; 1024];
    let n = stream
        .read(&mut response)
        .await
        .expect("Failed to read from stream");
    let response = &response[..n];

    print_bytes_as_hex(response);

    // 断言响应是否正确
    assert_eq!(response.len(), 15);

    // 关闭连接
    drop(stream);

    // 等待服务器线程结束
    server_task.await.expect("Failed to join server thread");
}

fn print_bytes_as_hex(bytes: &[u8]) {
    for byte in bytes {
        print!("{:02X} ", byte);
    }
    println!();
}
