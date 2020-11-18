use std::fmt::{self, Display, Formatter, Debug};

pub struct IpAddr {
	ip: String,
	port: u32,
}

impl IpAddr {
	pub fn new(ip: String, port: u32) -> Self {
		Self {
			ip,
			port
		}
	}
}

impl Display for IpAddr {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.ip, self.port)
	}
}

impl Debug for IpAddr {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.ip, self.port)
	}
}
