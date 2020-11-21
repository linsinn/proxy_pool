use crate::crawlers::get_html;
use crate::proxy_addr::ProxyAddr;
use regex::Regex;

pub async fn crawl() -> Result<Vec<ProxyAddr>, anyhow::Error> {
	let url = "http://www.ip3366.net/free/?stype=1&page=";
	let page = 5usize;
	let mut ret = vec![];
	for i in 1..=page {
		let url = format!("{}{}", url, i);
		let html = get_html(&url).await?;
		match parse(&html) {
			Ok(addrs) => ret.extend(addrs),
			Err(_) => continue,
		}
	}
	Ok(ret)
}

fn parse(html: &String) -> Result<Vec<ProxyAddr>, anyhow::Error> {
	let mut addrs = vec![];
	let pattern = Regex::new(r"<tr>\s*<td>(.*?)</td>\s*<td>(.*?)</td>").unwrap();
	for cap in pattern.captures_iter(html) {
		addrs.push(ProxyAddr::new(cap[1].to_string(), cap[2].parse().unwrap()));
	}
	Ok(addrs)
}

