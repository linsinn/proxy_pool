use crate::crawlers::get_html;
use scraper::{Html, Selector};
use crate::ip_addr::IpAddr;

pub async fn crawl() -> Result<Vec<IpAddr>, anyhow::Error> {
	let url = "http://www.data5u.com";
	let html = get_html(url).await?;
	parse(&html)
}

fn parse(html: &String) -> Result<Vec<IpAddr>, anyhow::Error> {
	let mut addrs = vec![];
	let document = Html::parse_document(html);
	let selector = Selector::parse(".wlist ul.l2").unwrap();
	for wl in  document.select(&selector) {
		let selector = Selector::parse("span:first-child").unwrap();
		let ip: String = wl.select(&selector).next().unwrap().text().collect();
		let selector = Selector::parse("span:nth-child(2)").unwrap();
		let port= wl.select(&selector).next().unwrap().text().collect::<String>().parse::<u32>().unwrap();
		addrs.push(IpAddr::new(ip, port));
	}
	Ok(addrs)
}