use crate::proxy_addr::ProxyAddr;
use crate::crawlers::build_headers;
use tokio::time::Duration;
use reqwest::Proxy;
use crate::storages;
use serde::Deserialize;

const TEST_ANONYMOUS: bool = true;
const TEST_TIMEOUT: u64 = 10;
const TEST_BATCH: u32 = 20;

#[derive(Debug, Deserialize, PartialEq)]
struct Ip {
	origin: String,
}

async fn test(proxy: ProxyAddr) {
	let proxy_url = format!("http://{}", proxy);
	let url = "https://httpbin.org/ip";
	let client = reqwest::Client::builder().build().unwrap();
	let origin: Option<Ip> = match client.get(url).headers(build_headers()).timeout(Duration::from_secs(TEST_TIMEOUT))
		.send().await.and_then(|s| Ok(s.json())) {
		Ok(f) => f.await.map_or_else(|_| None, |j| Some(j)),
		Err(_) => None
	};
	let client = reqwest::Client::builder().proxy(Proxy::http(&proxy_url).unwrap()).build().unwrap();
	let anonymous: Option<Ip> = match client.get(url).headers(build_headers()).timeout(Duration::from_secs(TEST_TIMEOUT))
		.send().await.and_then(|s| Ok(s.json())) {
		Ok(f) => f.await.map_or_else(|_| None, |j| Some(j)),
		Err(_) => None
	};
	match (&origin, &anonymous) {
		(Some(a), Some(b)) => {
			if a == b {
				storages::redis::decrease_score(proxy);
			} else {
				storages::redis::max_score(proxy);
			}
		}
		_ => storages::redis::decrease_score(proxy),
	}
}

pub async fn run() {
	let mut cursor = 0;
	loop {
		let (next_cursor , proxies) = storages::redis::batch(cursor, TEST_BATCH);
		for proxy in proxies {
			test(proxy).await;
		}
		cursor = next_cursor;
		if cursor == 0 {
			break;
		}
	}
}