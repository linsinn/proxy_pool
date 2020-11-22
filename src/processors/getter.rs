use crate::storages;
use crate::proxy_addr::ProxyAddr;
use crate::crawlers::*;

const MAX_PROXY_NUMBER: u32 = 50000;

fn is_full() -> bool {
	storages::redis::count() >= MAX_PROXY_NUMBER
}

pub async fn run() {
	let mut proxies = vec![];
	if !is_full() {
		proxies.extend(data5u::crawl().await.unwrap_or_else(|_| vec![]));
		proxies.extend(ip3366::crawl().await.unwrap_or_else(|_| vec![]));
		proxies.extend(kuaidaili::crawl().await.unwrap_or_else(|_| vec![]));
		proxies.extend(xiladaili::crawl().await.unwrap_or_else(|_| vec![]));
	}
	for proxy in proxies {
		storages::redis::add_proxy(proxy);
	}
}