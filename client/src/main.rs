use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    //创建客户端连接到本地服务器8088端口
    let mut stream = TcpStream::connect("127.0.0.1:8088")?;
    for _ in 0..10 {
        //创建字符实例
        let mut input = String::new();
        //读取键盘输入
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        //把键盘输入写入stream中
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream");
        
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();

        //把键盘输入写入缓冲，发送到服务器端
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        println!("{}", 
            str::from_utf8(&buffer).expect("Could not write buffer as string"));
        println!("");
    }
    Ok(())
}