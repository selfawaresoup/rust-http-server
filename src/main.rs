use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::io::{BufReader, BufRead, Write, Result};

mod http;

use http::request::Request;

fn handle_request(mut stream: TcpStream, req_id: u64) -> Result<()> {
	let mut reader = BufReader::new(&stream);
	let mut start_line = String::new();
	reader.read_line(&mut start_line)?;

	if let Some(req) = Request::from_start_line(&start_line, req_id) {
		stream.write("HTTP/1.1 200 OK\n".as_bytes())?;
		println!("{}", req);
	} else {
		// couldn't parse the start line, request is malformed
		stream.write("HTTP/1.1 400 Bad Request\n".as_bytes())?;
		println!("Malformed start line: {}", start_line);
	}

	stream.flush()?;
	stream.shutdown(Shutdown::Both)?;
	Ok(())
}

fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1:8000")?;
	let mut next_req_id: u64 = 0;
	
	for stream in listener.incoming() {
		next_req_id = next_req_id.wrapping_add(1);
		match stream {
			Ok(st) => {
				let req_id = next_req_id;
				thread::spawn(move || {
					match handle_request(st, req_id) {
						Ok(_) => (),
						Err(err) => println!("{:?}", err)
					}
				});
			},
			Err(err) => {
				println!("{:?}", err);
			}
		}
	}
	Ok(())
}