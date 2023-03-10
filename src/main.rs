use std::{
    fs,
    io::{prelude::*, ErrorKind},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use ws_rust::ThreadPool;

const BASE_URL: &str = "127.0.0.1:7878";

fn main() {
    let listener_result = TcpListener::bind(&BASE_URL);

    let listener = match listener_result {
        Ok(l) => l,
        Err(error) => match error.kind() {
            ErrorKind::AddrNotAvailable => match TcpListener::bind(&BASE_URL) {
                Ok(l) => l,
                Err(e) => panic!("Проблемы с привязкой к порту: {:?}", e),
            },
            other_error => {
                panic!("Какая-то неведома проблема: {:?}", other_error);
            }
        },
    };

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        match stream {
            Ok(s) => {
                pool.execute(|| {
                    bootstrap(s);
                });
            }
            Err(e) => panic!("Соединение не удалось: {:?}", e),
        };
    }

    println!("Выключение.");
}

fn bootstrap(mut stream: TcpStream) {
    let mut buffer = [0; 124];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents_result = fs::read_to_string(filename);

    let contents = match contents_result {
        Ok(c) => c,
        Err(e) => panic!("Возникла какая-то ошибка: {}", e),
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents,
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
