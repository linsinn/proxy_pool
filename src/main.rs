mod crawlers;
mod ip_addr;
mod storages;

use crate::crawlers::*;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	let proxies = get_proxies().await;
	for proxy in proxies {
		storages::redis::add(proxy);
	}
	Ok(())
}
