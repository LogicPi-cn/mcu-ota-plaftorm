use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;

fn send_request() {
    let mut stream =
        TcpStream::connect("ota.logicpi.cn:9999").expect("Failed to connect to server");

    // 发送请求数据
    let request_data = b"Hello, server!";
    stream.write_all(request_data).unwrap();

    // 接收响应数据
    let mut response = Vec::new();
    stream.read_to_end(&mut response).unwrap();

    println!("Received response: {:?}", response);
}

fn main() {
    let num_threads = 100; // 并发连接数

    let mut threads = Vec::new();
    for _ in 0..num_threads {
        let thread = thread::spawn(|| {
            send_request();
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
