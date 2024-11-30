use actix_web::{web, App, HttpResponse, HttpServer};
use std::io;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

use actix_cors::Cors;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/stream", web::get().to(stream_video))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn stream_video() -> HttpResponse {
    let file_path = "./file/test.mp4";
    let file = File::open(file_path).await;

    match file {
        Ok(f) => {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(f);

            let mut content = Vec::new();
            while let Ok(n) = reader.read_to_end(&mut buffer).await {
                if n == 0 {
                    break;
                }
                content.extend_from_slice(&buffer[..n]);
            }

            HttpResponse::Ok().content_type("video/mp4").body(content)
        }

        Err(_) => HttpResponse::NotFound().body("video not found"),
    }
}
