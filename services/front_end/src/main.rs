use std::{
    fs::{self, File},
    io::{self, BufReader},
};

use anyhow::{Result, Context};

use ntex::web;
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer},
};


#[ntex::main]
async fn main() {
    match start().await {
        Ok(_) => (),
        Err(e) => eprintln!("CRITICAL ERROR: {e}"),
    }
}


async fn start() -> Result<()> {
    let cert: Vec<CertificateDer> = rustls_pemfile::certs(
        &mut BufReader::new(&mut File::open("cert.crt")?)
    ).collect::<Result<Vec<CertificateDer>, io::Error>>()?;

    let key: PrivateKeyDer = rustls_pemfile::private_key(
        &mut BufReader::new(&mut File::open("key.pem")?)
    )?.context("Error initializing the private key")?;

    let server_config: ServerConfig = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert, key)?;

    web::HttpServer::new(|| {
        web::App::new()
            .service(index)
            .service(login)
            .service(register)
            .service(transfer)
    })
        .bind_rustls(("0.0.0.0", 443), server_config)?
        .run()
        .await?;
    Ok(())
}


#[web::get("/")]
async fn index() -> impl web::Responder {
    match fs::read_to_string("./html/index.html") {
        Ok(resp) => web::HttpResponse::Ok().body(resp),
        Err(e) => {
            let resp: String = format!("ERROR: {e}");
            eprintln!("{resp}");
            web::HttpResponse::InternalServerError().body(resp)
        },
    }
}


#[web::get("/login")]
async fn login() -> impl web::Responder {
    web::HttpResponse::Ok().body("login success")
}


#[web::get("/register")]
async fn register() -> impl web::Responder {
    web::HttpResponse::Ok().body("register success")
}


#[web::get("/transfer")]
async fn transfer() -> impl web::Responder {
    web::HttpResponse::Ok().body("transfer success")
}
