use proxy_pool::crawlers::data5u;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
	let addrs = data5u::Data5u::crawl().await?;
	dbg!(addrs);
	Ok(())
}
