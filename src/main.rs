mod crawlers;
mod ip_addr;
use crate::crawlers::*;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
	let addrs = get_proxies().await;
	println!("{}\n{:?}", addrs.len(), addrs);
	Ok(())
}
