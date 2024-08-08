use std::{
    fs::{self, File},
    io::{self, BufReader},
    path::Path,
};

use anyhow::{Result, Context};

use ntex::web::{self, HttpResponse, HttpServer, Responder};
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

    HttpServer::new(|| {
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
async fn index() -> impl Responder {
    get_response("./html/index.html")
}


#[web::get("/login/")]
async fn login() -> impl Responder {
    get_response("./html/login.html")
}


#[web::get("/register/")]
async fn register() -> impl Responder {
    get_response("./html/register.html")
}


#[web::get("/transfer/")]
async fn transfer() -> impl Responder {
    get_response("./html/transfer.html")
}


fn get_response<P: AsRef<Path>>(path: P) -> impl Responder {
    match fs::read_to_string(path) {
        Ok(resp) => HttpResponse::Ok().body(resp),
        Err(e) => {
            let resp: String = format!("ERROR: {e}");
            eprintln!("{resp}");
            HttpResponse::InternalServerError().body(resp)
        },
    }
}
