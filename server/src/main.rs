use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    //创建一个u8数组
    let mut buf = [0; 512];
    for _ in 0..1000 {     
        //把stream读入数组中
        let len = stream.read(&mut buf)?;
        //读完数组打印OK，同时返回OK枚举值
        if len == 0 {
            println!("OK");
            return Ok(())
        }
        //stream.write(echo_string.as_bytes())?;
        // 输出读取到的内容
        println!("read {} bytes: {:?}", len, str::from_utf8(&buf[..len]));
    
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    //创建本地服务端，监听端口8088
    let listener = TcpListener::bind("127.0.0.1:8088")?;
    //创建线程数组容器
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    //遍历监听到的stream
    for stream in listener.incoming() {
        //获取stream的Ok成员值，如果出错则打印failed
        let stream = stream.expect("failed!");
        //针对监听到的stream创建线程，处理客户端发送的字符
        let handle = thread::spawn(move || {
            let r = handle_client(stream);
            //对handle_client函数进行match
            match r {
                Ok(()) => return (),
                Err(e) => {
                    eprintln!("{:?}", e);
                },
            }
        });
        //把线程句柄加入线程数组容器
        thread_vec.push(handle);
    }

    //阻塞每一个线程，直到线程都执行完毕
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
