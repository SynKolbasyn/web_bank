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
use serde::Deserialize;

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
            .service(login_data)
            .service(register_data)
            .service(transfer_data)
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


#[web::post("/login/")]
async fn login_data(form: web::types::Form<LoginData>) -> impl Responder {
    HttpResponse::Ok().body(format!("Login DONE!\n\nLogin: {}\nPassword: {}", form.login, form.password))
}


#[web::post("/register/")]
async fn register_data(form: web::types::Form<RegisterData>) -> impl Responder {
    HttpResponse::Ok().body(format!("Register DONE!\n\nLogin: {}\nPassword: {}", form.login, form.password))
}


#[web::post("/transfer/")]
async fn transfer_data(form: web::types::Form<TransferData>) -> impl Responder {
    HttpResponse::Ok().body(format!("Transfer DONE!\n\nLogin: {}\nAmount: {}", form.login, form.amount))
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


#[derive(Deserialize)]
struct LoginData {
    login: String,
    password: String,
}


#[derive(Deserialize)]
struct RegisterData {
    login: String,
    password: String,
}


#[derive(Deserialize)]
struct TransferData {
    login: String,
    amount: String,
}
