use reqwest::header::{HeaderMap, HeaderValue};
use crate::ip_addr::IpAddr;

mod data5u;
mod ip3366;
mod kuaidaili;
mod xiladaili;

pub async fn get_proxies() -> Vec<IpAddr> {
	let mut proxies = vec![];
	proxies.extend(data5u::crawl().await.unwrap_or_else(|_| vec![]));
	proxies.extend(ip3366::crawl().await.unwrap_or_else(|_| vec![]));
	proxies.extend(kuaidaili::crawl().await.unwrap_or_else(|_| vec![]));
	proxies.extend(xiladaili::crawl().await.unwrap_or_else(|_| vec![]));
	proxies
}

fn build_headers() -> HeaderMap {
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
