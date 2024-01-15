use actix_web::{App,HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/day1", "public").index_file("pages/day1.html"))
            .service(actix_files::Files::new("/", "public").index_file("pages/index.html"))
                //post request /get
    })
    .bind(("0.0.0.0", 4200))?
    .run()
    .await
}

