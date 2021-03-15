use actix_web::{get, middleware, web, App, HttpResponse, HttpRequest, HttpServer, Responder};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

// curl http://localhost:8080/22/bob/index.html
#[get("/{id}/{name}/index.html")]
async fn get_id_name(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

// curl http://localhost:8080/name/bob
#[get("/name/{name}")]
async fn get_name(web::Path(name): web::Path<String>) -> impl Responder {
    format!("name: {}", name)
}

#[get("/again")]
async fn again() -> impl Responder {
    println!("GET: /again");
    HttpResponse::Ok().body("Hello world again!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(||
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!"}))
            .service(web::resource("/").to(index))
            .service(get_id_name)
            .service(get_name)
            .service(again)
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"Hello world!"##);

        Ok(())
    }
}