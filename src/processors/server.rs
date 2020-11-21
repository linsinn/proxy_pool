use crate::storages;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn index() -> impl Responder {
	HttpResponse::Ok().content_type("text/html").body("<h2>Welcome to Proxy Pool System</h2>")
}

#[get("/random")]
async fn get_proxy() -> impl Responder {
	if let Some(proxy) =  storages::redis::random() {
		proxy.to_string()
	} else {
		format!("can't find any available proxy")
	}
}

#[get("/count")]
async fn get_count() -> impl Responder {
	let cnt = storages::redis::count();
	format!("{}", cnt)
}

pub async fn run() -> tokio::io::Result<()> {
	HttpServer::new(|| App::new().service(index).service(get_proxy).service(get_count))
		.bind("127.0.0.1:8080")?
		.run()
		.await
}