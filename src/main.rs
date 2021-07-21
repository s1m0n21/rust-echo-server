use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 1024]; // 创建 buffer

    let addr = stream.peer_addr()?; // 保存连接地址
    println!("[{} online]", addr); // 打印连接

    loop {
        let read = stream.read(&mut buf)?; // 读取流
        if &buf[..read] == [255, 244, 255, 253, 6] { // 判断消息是否为 control c
            println!("[{} received exit signal]", addr); // 打印接收到终止命令
            break // 终止循环
        }

        match String::from_utf8(Vec::from(&buf[..read])) { // 将 bytes 转换为 String，判断是否成功
            Ok(m) => print!("[{}]: {}", addr, m), // 转换成功，打印接收到的消息字符串
            _ => println!("[{} parse message failed, raw = {:?}]", addr, &buf[..read]) // 转换失败，打印原始 bytes
        }

        stream.write(&buf[..read])?; // 返回消息
    }

    println!("[{} offline]", addr); // 循环终止代表接收到退出信号，打印连接关闭

    Ok(()) // 返回
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9099").unwrap(); // 创建监听器
    println!("listen on {}", listener.local_addr().unwrap()); // 打印监听地址

    for stream in listener.incoming() { // 读取连接
        let stream = stream.unwrap(); // 检查连接返回是否为 Ok()
        spawn(move || { // 创建线程
            handle_client(stream).unwrap_or_else(|err| eprintln!("{:?}", err)) // 启动 handle_client 并处理错误
        });
    }
}