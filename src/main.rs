use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::process::Command;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/check_code")]
async fn check_code(req_body: String) -> impl Responder {
    let code = req_body.as_str();
    fs::write("code.cpp", code).expect("Unable to write file");
    let output = Command::new("cppcheck")
        .arg("code.cpp")
        .arg("--error-exitcode=1")
        .output()
        .expect("failed to execute process");
    fs::remove_file("code.cpp").expect("Unable to remove file");
    if output.status.success() {
        return HttpResponse::Ok().body("No errors found");
    }
    return HttpResponse::Ok().body(output.stderr);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(check_code))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
