use std::{ 
    fs,
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream}, 
    thread, 
    time::Duration,
};
use hello::ThreadPool;
 
fn main() { 
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Gagal membuka server: {e}");
            return; // Hentikan program dengan aman
        }
    };
    let pool = match ThreadPool::build(4) {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create thread pool: {e}");
            return;
        }
    };

 
    for stream in listener.incoming() { 
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error accepting connection: {:?}", e);
                return; // Hentikan program dengan aman
            }
        };
 
        pool.execute(|| {
            if let Err(e) = handle_connection(stream) {
                eprintln!("Failed to handle connection: {e}");
            }
        });
    } 
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut stream); 
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => return Err(e), // Kalau gagal baca, kembalikan error ke luar
        None => return Ok(()),         // Kalau pengunjung tiba-tiba menutup tab browser
    };

    let (status_line, filename) = match &request_line[..] { 
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"), 
        "GET /sleep HTTP/1.1" => { 
            thread::sleep(Duration::from_secs(10)); 
            ("HTTP/1.1 200 OK", "hello.html") 
        }, 
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"), 
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;

    Ok(())
}