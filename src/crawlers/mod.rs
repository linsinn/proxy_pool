use reqwest::header::{HeaderMap, HeaderValue};

pub mod data5u;

fn build_headers() -> HeaderMap {
	let mut header_map = HeaderMap::new();
	header_map.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.198 Safari/537.36"));
	header_map
}