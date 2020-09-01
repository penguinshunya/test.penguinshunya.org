use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
  "hello, world"
}

async fn index2() -> impl Responder {
  "hello world again"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(index))
      .route("/again", web::get().to(index2))
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
