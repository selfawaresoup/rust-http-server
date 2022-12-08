use std::fmt;
use std::io;
use std::io::Write;
use std::net::TcpStream;

use crate::http::Header;

#[derive(Debug)]
pub struct Response {
	pub status: u16,
	headers: Vec<Header>,
	writer: io::BufWriter<TcpStream>,
	body_active: bool,
}

impl Response {
	pub fn new(writer: io::BufWriter<TcpStream>) -> Self {
		Self{
			status: 200,
			headers: vec![],
			body_active: false,
			writer
		}
	}

	#[allow(dead_code)]
	pub fn set_status(&mut self, status: u16) {
		self.status = status;
	}
	
	#[allow(dead_code)]
	pub fn add_header(&mut self, header: Header) {
		if self.body_active {
			return;
		}
		self.headers.push(header);
	}

	fn write_headers(&mut self) -> io::Result<()> {
		if self.body_active {
			return Ok(());
		}

		self.writer.write("HTTP/1.1 ".as_bytes())?;
		self.writer.write(self.status.to_string().as_bytes())?;
		self.writer.write("\r\n".as_bytes())?;

		let mut headers = vec![];
		headers.append(&mut self.headers);

		for header in headers {
			self.writer.write(header.to_string().as_bytes())?;
			self.writer.write("\r\n".as_bytes())?;
		}
		
		Ok(())
	}

	fn close_headers(&mut self) -> io::Result<()>{
		if !self.body_active {
			self.write_headers()?;
			self.writer.write("\r\n".as_bytes())?;
		}
		Ok(())
	}
}

impl fmt::Display for Response {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.status)
	}
}

impl io::Write for Response {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.close_headers()?;
		self.body_active = true;
		self.writer.write(buf)
	}
	
	fn flush(&mut self) -> std::io::Result<()> {
		self.close_headers()?;
		self.body_active = true;
		self.writer.flush()
	}
}
