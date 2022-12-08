use std::fmt;

#[derive(Debug)]
pub enum Method {
	HEAD,
	DELETE,
	GET,
	PUT,
	POST,
}

impl Method {
	pub fn from_str(name: &str) -> Option<Self> {
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