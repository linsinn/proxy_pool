use crate::ip_addr::IpAddr;
use std::time::Duration;
use std::ops::DerefMut;
use r2d2::Pool;
use once_cell::sync::Lazy;

const CACHE_POOL_MAX_OPEN: u32 = 16;
const CACHE_POOL_MIN_IDLE: u32 = 8;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

static REDIS_POOL: Lazy<Pool<redis::Client>> = Lazy::new( || {
	let client = redis::Client::open("redis://127.0.0.1/").unwrap();
	r2d2::Pool::builder()
		.max_size(CACHE_POOL_MAX_OPEN)
		.max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
		.min_idle(Some(CACHE_POOL_MIN_IDLE))
		.build(client)
		.unwrap()
	}
);

pub fn add(proxy: IpAddr) {
	let pool = REDIS_POOL.clone();
	let mut conn = pool.get().unwrap();
	redis::cmd("zadd")
		.arg("proxies")
		.arg("NX")
		.arg(100)
		.arg(proxy.to_string())
		.query::<i32>(conn.deref_mut()).unwrap();
}

