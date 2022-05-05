// use std::fs;
// use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
// use std::thread;
// use std::time::Duration;

// use test_server::ThreadPool;


fn main() {
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // let pool = ThreadPool::new(4);

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     pool.execute(|| {
    //         handle_connection(stream);
    //     });
    // }

    // println!("Shutting down.");
    // Esta es la ip de torrent.ubuntu.com al puerto 6969 que en general es el default para http
    make_request("91.189.95.21:6969");
}

fn make_request(addr: &str) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let request = "GET /announce?info_hash=778ce280b595e57780ff083f2eb6f897dfa4a4ee&peer_id=1234 HTTP/1.1\r\nHost: torrent.ubuntu.com\r\nContent-type: text/plain\r\n\r\n";
            stream.write(request.as_bytes()).unwrap();
            
            let mut data = [0 as u8; 512]; // esto es un buffer de 256 bytes, fijarse que viene cortada la respuesta
            //porque no le da el buffer para todo. Habría que hacerlo dinamico o algo asi
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        let text = String::from_utf8((&data).to_vec()).unwrap();
                        println!("Respuesta: {}", text);
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
        },
        Err(e) => println!("error: {}", e)
    }
}

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();

//     let get = b"GET / HTTP/1.1\r\n";
//     let sleep = b"GET /sleep HTTP/1.1\r\n";

//     let (status_line, filename) = if buffer.starts_with(get) {
//         ("HTTP/1.1 200 OK", "hello.html")
//     } else if buffer.starts_with(sleep) {
//         thread::sleep(Duration::from_secs(5));
//         ("HTTP/1.1 200 OK", "hello.html")
//     } else {
//         ("HTTP/1.1 404 NOT FOUND", "404.html")
//     };

//     let contents = fs::read_to_string(filename).unwrap();

//     let response = format!(
//         "{}\r\nContent-Length: {}\r\n\r\n{}",
//         status_line,
//         contents.len(),
//         contents
//     );

//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }