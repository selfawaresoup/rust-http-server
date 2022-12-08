use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Write;
use std::io;

mod http;

use http::{Request, Response, Method, Header};

fn dispatch(req: Request, mut res: Response) -> io::Result<()> {
	match req.method {
		Method::GET => {
			match req.url.as_str() {
				"/hello" => {
					res.add_header(Header::new("Conent-Type", "text/plain"));
					res.write("Hello World!\n".as_bytes())?;
				},
				_ => {
					res.set_status(404);
				}
			}
		},
		_ => {
			res.set_status(405);
			res.add_header(Header::new("Allow", "GET"));
		}
	}
	
	println!("{} {}", req, res.status);
	res.flush()?;
	Ok(())
}

fn handle_request(stream: TcpStream, req_id: u64) -> io::Result<()> {
	let read_stream = stream.try_clone()?;
	let reader = io::BufReader::new(read_stream);
	let writer = io::BufWriter::new(stream);
	let mut res = Response::new(writer);

	if let Some(req) = Request::new(req_id, reader) {
		return dispatch(req, res);
	} else {
		res.set_status(400);
		res.write("Request was malformed\n".as_bytes())?;
		res.flush()?;
	}

	Ok(())
}

fn main() -> io::Result<()> {
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