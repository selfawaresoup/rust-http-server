use std::fmt;

#[derive(Debug)]
enum Method {
	HEAD,
	DELETE,
	GET,
	PUT,
	POST,
}

impl Method {
	pub fn from_string(name: &str) -> Option<Self> {
		match name {
			"HEAD" => Some(Self::HEAD),
			"DELETE" => Some(Self::DELETE),
			"GET" => Some(Self::GET),
			"PUT" => Some(Self::PUT),
			"POST" => Some(Self::POST),
			_ => None
		}
	}
}

impl fmt::Display for Method {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Self::HEAD => "HEAD",
			Self::DELETE => "DELETE",
			Self::GET => "GET",
			Self::PUT => "PUT",
			Self::POST => "POST",
		})
	}
}

#[derive(Debug)]
pub struct Request {
	id: u64,
	method: Method,
	url: String,
	version: String,
}

impl Request {
	pub fn from_start_line(start_line: &String, id: u64) -> Option<Self> {
		let mut parts = start_line.split_whitespace();

		let method = Method::from_string(parts.next()?)?;
		let url = parts.next()?.to_string();
		let version = parts.next()?.to_string();

		Some(Self{id, method, url, version})
	}
}

impl fmt::Display for Request {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {} {} {}", self.id, self.version, self.method, self.url)
	}
}