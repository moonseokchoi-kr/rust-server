use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::path::Path;
use std::fs;
use std::thread::{self, Thread};
use std::time::Duration;
use single_thread_server::ThreadPool;

fn main() {
    //주어진 ip로 바인드, 바인드 중 문제가 생기면 중단
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    //연결된 데이터를 전달 받기 위해 stream 이용, 마찬가지로 문제 생기면 중단.
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.excute(|| {
        //요청을 다루어보자(closer)
        handle_connection(stream);
        });

    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 2048];
    let path = Path::new("./src/www/");

    //stream으로 데이터를 읽어 buffer에 채운다
    //자꾸 여기서 버퍼 채우다 죽는다.. 원인이 뭐지..?
    //버퍼의 크기가 너무 작아서 데이터를 모두 다 못읽어서 생기는 문제였다.. 이거 가변으로 설정할 수 있도록 해야겠다.
    stream.read(&mut buffer).unwrap();

    //버퍼안에 저장된 데이터를 문자열로 바꾸어 출력한다.(debug)
    //println!("Request : {}", String::from_utf8_lossy(&buffer[..]));

    //요청을 확인하고 routing 해주자
    let get = b"GET / HTTP/1.1\r\n";

    //강제로 부하를 걸어보자
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let html = path.join(filename);
    let contents = fs::read_to_string(html).unwrap();
    let response = format!("{}{}",status_line,contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

