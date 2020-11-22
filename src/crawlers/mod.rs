use reqwest::header::{HeaderMap, HeaderValue};
use crate::proxy_addr::ProxyAddr;

pub mod data5u;
pub mod ip3366;
pub mod kuaidaili;
pub mod xiladaili;

pub fn build_headers() -> HeaderMap {
	let mut header_map = HeaderMap::new();
	header_map.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.198 Safari/537.36"));
	header_map
}

async fn get_html(url: &str) -> Result<String, anyhow::Error> {
	let client = reqwest::Client::new();
	let html = client.get(url).headers(build_headers())
		.send()
		.await?
		.text()
		.await?;
	Ok(html)
}

#[cfg(tests)]
mod tests {
	use super::*;
	#[test]
	fn test_ip3366() {
		println!("asdfasdf");
		ip3366::crawl();
		assert!(false);
	}
}
