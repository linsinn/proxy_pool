use crate::crawlers::get_html;
use scraper::{Html, Selector};
use crate::proxy_addr::ProxyAddr;

pub async fn crawl() -> Result<Vec<ProxyAddr>, anyhow::Error> {
	let url = "http://www.xiladaili.com/";
	let html = get_html(&url).await?;
	parse(&html)
}

fn parse(html: &String) -> Result<Vec<ProxyAddr>, anyhow::Error> {
	let mut addrs = vec![];
	let document = Html::parse_document(html);
	let selector = Selector::parse("table tbody tr td:first-child").unwrap();
	for wl in  document.select(&selector) {
		if let Some(ip_port) =  wl.first_child().unwrap().value().as_text() {
			let ip_port = ip_port.to_string();
			let ip_port: Vec<&str> = ip_port.split(":").collect();
			if ip_port.len() == 2 {
				addrs.push(ProxyAddr::new(ip_port[0].to_string(), ip_port[1].parse().unwrap()));
			}
		}
	}
	Ok(addrs)
}