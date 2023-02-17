use std::{
    fs,
    io::prelude::*,
    io::ErrorKind,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use ws_rust::ThreadPool;

fn main() {
    const BASE_URL: &str = "127.0.0.1:7878";
    let listener_result = TcpListener::bind(BASE_URL);

    let listener = match listener_result {
        Ok(l) => l,
        Err(error) => match error.kind() {
            ErrorKind::AddrNotAvailable => match TcpListener::bind(BASE_URL) {
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
        let stream = stream.unwrap();

        pool.execute(|| {
            bootstrap(stream);
        });
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

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents,
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
