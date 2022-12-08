use std::fmt;

#[derive(Debug)]
pub struct Header {
	name: String,
	value: String,
}

impl Header {
	pub fn new(name: &str, value: &str) -> Self {
		Self{name: name.to_string(), value: value.to_string()}
	}

	pub fn from_str(line: &str) -> Option<Self> {
		let mut parts = line.split(":");
		let name = parts.next()?.to_string();
		let value = parts.next()?.trim().to_string();

		Some(Self{name, value})
	}
}

impl fmt::Display for Header {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.name, self.value)
	}
}
