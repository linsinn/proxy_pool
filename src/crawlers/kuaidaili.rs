use crate::crawlers::get_html;
use scraper::{Html, Selector};
use crate::proxy_addr::ProxyAddr;

pub async fn crawl() -> Result<Vec<ProxyAddr>, anyhow::Error> {
	let url = "https://www.kuaidaili.com/free/inha/";
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
	let document = Html::parse_document(html);
	let selector = Selector::parse("table tr").unwrap();
	for wl in  document.select(&selector) {
		let selector = Selector::parse(r#"td[data-title="IP"]"#).unwrap();
		let ip: String = match wl.select(&selector).next() {
			Some(ip_node) => ip_node.text().collect(),
			_ => continue,
		};
		let selector = Selector::parse(r#"td[data-title="PORT"]"#).unwrap();
		let port: u32 = match wl.select(&selector).next() {
			Some(port_node) => port_node.text().collect::<String>().parse::<u32>().unwrap(),
			_ => continue,
		};
		addrs.push(ProxyAddr::new(ip, port));
	}
	Ok(addrs)
}