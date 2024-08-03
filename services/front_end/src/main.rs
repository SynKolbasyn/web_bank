use std::fs;

use ntex::web;

use anyhow::Result;


#[ntex::main]
async fn main() {
    match start().await {
        Ok(_) => (),
        Err(e) => eprintln!("CRITICAL ERROR: {e}"),
    }
}


async fn start() -> Result<()> {
    web::HttpServer::new(|| web::App::new().service(site))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;
    Ok(())
}


#[web::get("/")]
async fn site() -> impl web::Responder {
    match fs::read_to_string("./html/index.html") {
        Ok(resp) => web::HttpResponse::Ok().body(resp),
        Err(e) => {
            let resp: String = format!("ERROR: {e}");
            eprintln!("{resp}");
            web::HttpResponse::NotFound().body(resp)
        },
    }
}
