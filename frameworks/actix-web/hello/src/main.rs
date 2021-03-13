use actix_web::{get, web, App, HttpServer, Responder};

// curl http://localhost:8080/22/bob/index.html
#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

// curl http://localhost:8080/name/bob
#[get("/name/{name}")]
async fn get_name(web::Path(name): web::Path<String>) -> impl Responder {
    format!("name: {}", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(index)
            .service(get_name)
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}