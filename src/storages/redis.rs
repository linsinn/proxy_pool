use crate::proxy_addr::ProxyAddr;
use std::time::Duration;
use std::ops::DerefMut;
use r2d2::Pool;
use once_cell::sync::Lazy;
use std::panic::resume_unwind;
use std::str::FromStr;

const REDIS_KEY: &str = "proxies";

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

pub fn add_proxy(proxy: ProxyAddr) {
	let pool = REDIS_POOL.clone();
	let mut conn = pool.get().unwrap();
	redis::cmd("zadd")
		.arg(REDIS_KEY)
		.arg("NX")
		.arg(100)
		.arg(proxy.to_string())
		.query::<i32>(conn.deref_mut()).unwrap();
}


pub fn random() -> Option<ProxyAddr> {
	let pool = REDIS_POOL.clone();
	let mut conn = pool.get().unwrap();
	let result: Vec<String> = redis::cmd("zrevrange")
		.arg(REDIS_KEY)
		.arg(1)
		.arg(100)
		.query(conn.deref_mut()).unwrap();
	if result.len() == 0 {
		None
	} else {
		match ProxyAddr::from_str(&result[0]) {
			Ok(proxy) => Some(proxy),
			Err(_) => None
		}
	}
}

pub fn decrease_score(proxy: ProxyAddr) {
	let pool = REDIS_POOL.clone();
	let mut conn = pool.get().unwrap();
	redis::cmd("zincrby")
		.arg(REDIS_KEY)
		.arg(-1)
		.arg(proxy.to_string())
		.execute(conn.deref_mut());
	let score: i32 = redis::cmd("zscore")
		.arg(REDIS_KEY)
		.arg(proxy.to_string())
		.query(conn.deref_mut()).unwrap();
	if score == 0 {
		redis::cmd("zrem")
			.arg(REDIS_KEY)
			.arg(proxy.to_string())
			.execute(conn.deref_mut());
	}
}

pub fn count() -> u32 {
	let pool = REDIS_POOL.clone();
	let mut conn = pool.get().unwrap();
	redis::cmd("zcard").arg(REDIS_KEY).query(conn.deref_mut()).unwrap()
}