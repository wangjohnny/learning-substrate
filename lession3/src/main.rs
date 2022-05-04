use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

fn main() -> Result<()> {
    //在本地8080端口创建TCP连接
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    //客户端连接集合，支持多客户端连接
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    //不断的接收连接进来
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let handle = thread::spawn(move || {
                    //如发生错误，打印错误
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
                });
                // 把客户端连接加入集合
                thread_vec.push(handle);
            }
            Err(_e) => eprintln!("onnection failed!"), //连接错误
        }
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    //创建一个缓冲区，用来保存数据
    let mut buf = [0; 512];
    //循环读取数据，如果读取数据异常直接返回
    while match stream.read(&mut buf) {
        Ok(n) if n == 0 => {
            //如果读取到的⻓度是0，表示关闭连接，函数返回
            false
        }
        Ok(n) => {
            println!("received:{}", String::from_utf8_lossy(&buf[..n]));
            //将读取到的内容返写到客户端去
            stream.write(&buf[..n])?;
            // thread::sleep(time::Duration::from_secs(1));
            true
        }
        Err(_e) => {
            false //遇到错误，返回
        }
    } {}
    Ok(())
}
