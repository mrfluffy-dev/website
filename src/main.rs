use actix_web::{App,HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/example", "public").index_file("pages/example.html"))
            .service(actix_files::Files::new("/", "public").index_file("pages/index.html"))
                //post request /get
    })
    .bind(("127.0.0.1", 4200))?
    .run()
    .await
}

