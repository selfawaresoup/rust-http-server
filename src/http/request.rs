use std::fmt;
use std::io;
use std::io::BufRead;
use std::net::TcpStream;

use crate::http::{Header, Method};

#[derive(Debug)]
pub struct Request {
	pub id: u64,
	pub method: Method,
	pub url: String,
	version: String,
	pub headers: Vec<Header>,
	reader: io::BufReader<TcpStream>,
}

impl Request {
	pub fn new(id: u64, mut reader: io::BufReader<TcpStream>) -> Option<Self> {
		let mut start_line = String::new();
		match reader.read_line(&mut start_line) {
			Ok(_) => {
				let mut parts = start_line.split_whitespace();
		
				let method = Method::from_str(parts.next()?)?;
				let url = parts.next()?.to_string();
				let version = parts.next()?.to_string();
		
				let mut req = Self{
					id,
					method,
					url,
					version,
					headers: vec![],
					reader
				};

				match req.read_headers() {
					Ok(_) => Some(req),
					Err(_) => None
				}				
			},
			Err(_) => None,
		}
	}

	fn read_headers(&mut self) -> io::Result<()> {
			// read headers
		loop {
			let mut header_line = String::new();
			self.reader.read_line(&mut header_line)?;

			// empty line marks end of header section
			if header_line.eq("\r\n") { break; }

			if let Some(header) = Header::from_str(&header_line) {
				self.headers.push(header);
			}
		}
		Ok(())
	}
}

impl fmt::Display for Request {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {} {} {}", self.id, self.version, self.method, self.url)
	}
}

impl io::Read for Request {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.reader.read(buf)
	}
}
