use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::path::Path;
use std::fs;
fn main() {
    //주어진 ip로 바인드, 바인드 중 문제가 생기면 중단
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //연결된 데이터를 전달 받기 위해 stream 이용, 마찬가지로 문제 생기면 중단.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //요청을 다루어보자
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 2048];
    let mut path = Path::new("./src/www/hello.html");

    //stream으로 데이터를 읽어 buffer에 채운다
    //자꾸 여기서 버퍼 채우다 죽는다.. 원인이 뭐지..?
    //버퍼의 크기가 너무 작아서 데이터를 모두 다 못읽어서 생기는 문제였다.. 이거 가변으로 설정할 수 있도록 해야겠다.
    stream.read(&mut buffer).expect("Bad Stream");

    //버퍼안에 저장된 데이터를 문자열로 바꾸어 출력한다.(debug)
    //println!("Request : {}", String::from_utf8_lossy(&buffer[..]));

    let contents = fs::read_to_string(&mut path).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
