use std::fmt::{self, Display, Formatter, Debug};
use std::str::FromStr;

pub struct ProxyAddr {
	ip: String,
	port: u32,
}

impl ProxyAddr {
	pub fn new(ip: String, port: u32) -> Self {
		Self {
			ip,
			port
		}
	}
}

impl FromStr for ProxyAddr {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let ip_port = s.split(":").collect::<Vec<&str>>();
		if ip_port.len() != 2 {
			Err(anyhow::anyhow!("invalid proxy str"))
		} else {
			Ok(Self {
				ip: ip_port[0].to_string(),
				port: ip_port[1].parse::<u32>()?,
			})
		}
	}
}

impl Display for ProxyAddr {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.ip, self.port)
	}
}

impl Debug for ProxyAddr {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.ip, self.port)
	}
}
