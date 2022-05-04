use std::io::{Read, Result, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub fn main() -> std::io::Result<()> {
    //在本地8080端口创建TCP连接，将字符串解析为一个 SocketAddr 类型，如果解析失败，抛出异常
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr)?;
    //不断的接收连接进来，等同于循环的 listener.accept()
    for stream in listener.incoming() {
        //模式匹配，incoming得到的是一个Result,有可能是Ok，也有可能是Err。
        match stream {
            Ok(stream) => {
                handle_client(stream)
                    .unwrap_or_else(|error| eprintln!("Server error got: {:?}", error));
                //如果遇到错误，会打印错误，错误由 stream的read，write 发出
            }
            Err(e) => eprintln!("onnection failed!"), //连接错误
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    //创建一个缓冲区，用来保存数据
    let mut buf = [0; 500];
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
            true
        }
        Err(e) => {
            false //遇到错误，返回
        }
    } {}
    Ok(())
}
