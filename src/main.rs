mod crawlers;
mod proxy_addr;
mod storages;
mod processors;

use crate::crawlers::*;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
	processors::server::run().await;
	Ok(())
}
